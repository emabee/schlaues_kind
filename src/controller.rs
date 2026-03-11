use crate::{
    data::Data,
    ui::{
        Action,
        viz::{ModalState, V, WordCategory},
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
            if data.verb_indices.is_empty() {
                data.reset_verb_indices();
            }

            v.modal_state = ModalState::DeclineVerbs {
                verb_idx: data
                    .verb_indices
                    .pop()
                    .unwrap_or_else(|| unreachable!("No verbs available in act_on_modal.")),
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

            for verb in &data.verbs {
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
            let selected_category = WordCategory::default();
            if data.tricky_words_indices.is_empty() {
                data.reset_tricky_words_indices(selected_category);
            }

            v.modal_state = ModalState::ReadTrickyWords {
                word_idx: data
                    .tricky_words_indices
                    .pop()
                    .unwrap_or_else(|| unreachable!("No tricky words available in act_on_modal.")),
                selected_category,
            };
        }

        Action::ShowMathBasics => {
            v.modal_state = ModalState::BasicMath {
                chosen_operator: *data.operation.operator(),
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
            if data.verb_indices.is_empty() {
                data.reset_verb_indices();
            }
            *verb_idx = data
                .verb_indices
                .pop()
                .unwrap_or_else(|| unreachable!("No verbs available in act_on_modal."));
            *visibility_level = 0;
        }

        (
            Action::NextTrickyWord,
            ModalState::ReadTrickyWords {
                word_idx,
                selected_category,
            },
        ) => {
            if data.tricky_words_indices.is_empty() {
                data.reset_tricky_words_indices(*selected_category);
            }
            *word_idx = data
                .tricky_words_indices
                .pop()
                .unwrap(/* OK */);
        }

        (
            Action::ChangeTrickyWordCategory,
            ModalState::ReadTrickyWords {
                word_idx,
                selected_category,
            },
        ) => {
            data.reset_tricky_words_indices(*selected_category);
            *word_idx = data
                .tricky_words_indices
                .pop()
                .unwrap_or_else(|| unreachable!("No tricky words available in act_on_modal."));
        }

        (
            Action::NextCalculation,
            ModalState::BasicMath {
                chosen_operator: _,
                show_result,
            },
        ) => {
            data.operation.new_operands();
            *show_result = false;
        }

        (
            Action::ShowResult,
            ModalState::BasicMath {
                chosen_operator: _,
                show_result,
            },
        ) => {
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
    true
}
