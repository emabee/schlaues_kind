use rand::Rng;

use crate::{
    data::{Data, Operator},
    sounds,
    ui::{
        Action,
        viz::{ModalState, V},
    },
};
use std::{fs::OpenOptions, io::Write};

// The controller is responsible for managing the state of the application and the UI,
// and is the only place where the application data is modified.
// The UI code calls Controller::set_action() to set the next action to be taken.
// The main loop calls Controller::act() to execute the action.
#[derive(Default)]
pub struct Controller {
    next_action: Action,
}
impl Controller {
    // Set the next action to be taken by the controller.
    pub fn set_action(&mut self, action: Action) {
        self.next_action = action;
    }

    // Executes the action set by the UI code.
    #[allow(clippy::too_many_lines)]
    pub fn act(&mut self, data: &mut Data, v: &mut V) {
        let action = std::mem::take(&mut self.next_action);
        let action_s = format!("{action:?}");
        let done = if v.modal_state.is_none() {
            act_on_no_modal(v, data, action)
        } else {
            act_on_modal(v, data, action)
        };
        if !done {
            log::warn!(
                "Unhandled situation: {}, action = {action_s:?}",
                v.modal_state.get_id(),
            );
        }
    }
}

#[allow(clippy::too_many_lines)]
fn act_on_no_modal(v: &mut V, data: &mut Data, action: Action) -> bool {
    match action {
        Action::None => {}

        Action::ShowAbout => {
            v.modal_state = ModalState::About;
        }

        Action::DeclineVerbs => {
            v.modal_state = ModalState::DeclineVerbs {
                verb_idx: data.verbs.next_index(),
                visibility_level: 0,
            };
        }

        Action::PrintVerbs => {
            let mut file = OpenOptions::new()
                .write(true)
                .truncate(true)
                .create(true)
                .open("unregelmässige_verben.csv")
                .unwrap();

            for verb in &data.verbs.verbs {
                file.write_all(
                    format!(
                        "\"{}\",\"{}\",\"{}\"\n",
                        verb.infinitiv, verb.praeteritum.ich, verb.perfekt.ich
                    )
                    .as_bytes(),
                )
                .unwrap();
            }
        }

        Action::ReadTrickyWords => {
            const DEFAULT_WORD_LIST_INDEX: usize = 1;
            v.modal_state = ModalState::ReadTrickyWords {
                word_list_index: DEFAULT_WORD_LIST_INDEX,
                word_idx: data.word_lists[DEFAULT_WORD_LIST_INDEX].next_word_index(),
            };
        }

        Action::ShowMathBasics => {
            v.modal_state = ModalState::BasicMath {
                current_operator: *data.operation.operator(),
                show_result: false,
            };
        }

        Action::CloseModal => {
            v.modal_state.close_modal();
        }

        // Action::Cancel => {
        //     v.modal_state.close_modal();
        // }
        _ => {
            log::error!("Action {action:?} cannot be executed when no modal is open.");
            return false;
        }
    }
    true
}

#[allow(clippy::too_many_lines)]
fn act_on_modal(v: &mut V, data: &mut Data, action: Action) -> bool {
    match (action, &mut v.modal_state) {
        (_, ModalState::None) => {
            unreachable!("act_on_modal called with ModalState::None");
        }

        (
            Action::NextDeclination,
            ModalState::DeclineVerbs {
                verb_idx: _,
                visibility_level,
            },
        ) => {
            *visibility_level += 1;
            data.score += 1;
        }

        (
            Action::RestartVerb,
            ModalState::DeclineVerbs {
                verb_idx: _,
                visibility_level,
            },
        ) => {
            *visibility_level = 0;
        }

        (
            Action::NextVerb,
            ModalState::DeclineVerbs {
                verb_idx,
                visibility_level,
            },
        ) => {
            *verb_idx = data.verbs.next_index();
            *visibility_level = 0;
        }

        (
            Action::NextTrickyWord,
            ModalState::ReadTrickyWords {
                word_list_index,
                word_idx,
            },
        ) => {
            *word_idx = data.word_lists[*word_list_index].next_word_index();
            data.score += 1 + *word_list_index;
        }

        (
            Action::ChangeTrickyWordList(new_list_index),
            ModalState::ReadTrickyWords {
                word_list_index,
                word_idx,
            },
        ) => {
            *word_list_index = new_list_index;
            *word_idx = data.word_lists[*word_list_index].next_word_index();
        }

        (
            Action::NextCalculation,
            ModalState::BasicMath {
                current_operator: _,
                show_result,
            },
        ) => {
            data.operation.new_operands();
            *show_result = false;
        }

        (
            Action::ShowResult,
            ModalState::BasicMath {
                current_operator,
                show_result,
            },
        ) => {
            data.score += match current_operator {
                Operator::Add => 2,
                Operator::Subtract | Operator::Multiply => 3,
                Operator::Divide => 4,
            };
            *show_result = true;
        }

        (Action::CloseModal, _) => {
            v.modal_state.close_modal();
        }

        // (Action::Cancel, _) => {
        //     v.modal_state.close_modal();
        // }
        (action, _) => {
            if !matches!(action, Action::None) {
                return false;
            }
        }
    }

    if data.score > 30 {
        data.score = 0;

        sounds::play(
            &data.sink,
            match rand::rng().next_u32() % 6 {
                0 => sounds::Sound::BellDing,
                1 => sounds::Sound::BellChord,
                2 => sounds::Sound::ChristmasBell,
                3 => sounds::Sound::HandBell,
                4 => sounds::Sound::ShipBell,
                5 => sounds::Sound::Nice,
                6_u32..=u32::MAX => unreachable!(),
            },
        );
    }
    true
}
