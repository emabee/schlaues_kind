use crate::data::math_basics::Operator;
use anyhow::Result;
use std::fmt::Debug;

pub struct V {
    // pub base_state: BaseState,
    pub modal_state: ModalState,
}
impl V {
    pub fn new() -> Result<Self> {
        Ok(Self {
            // base_state: BaseState::default(),
            modal_state: ModalState::None,
        })
    }
}

pub enum ModalState {
    None,

    About,

    DeclineVerbs {
        verb_idx: usize,
        visibility_level: isize,
    },

    ReadTrickyWords {
        selected_category: WordCategory,
        word_idx: usize,
    },
    BasicMath {
        chosen_operator: Operator,
        show_result: bool,
    },
}
impl ModalState {
    pub fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }
    pub fn close_modal(&mut self) {
        *self = Self::None;
    }
    // pub fn no_modal_is_open(&self) -> bool {
    //     matches!(self, Self::None)
    // }
    pub fn is_ready_for_modal(&self) -> bool {
        matches!(self, ModalState::None)
    }
    pub fn get_id(&self) -> &'static str {
        match self {
            ModalState::None => "None",
            ModalState::About => "About",
            ModalState::DeclineVerbs { .. } => "Verbs",
            ModalState::ReadTrickyWords { .. } => "TrickyWords",
            ModalState::BasicMath { .. } => "BasicMath",
        }
    }
}

#[derive(Clone, PartialEq, Eq, Default, Copy)]
pub enum WordCategory {
    Short,
    #[default]
    Medium,
    Long,
}
impl Debug for WordCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WordCategory::Short => write!(f, "{}", t!("Short_tricky_words")),
            WordCategory::Medium => write!(f, "{}", t!("Medium_tricky_words")),
            WordCategory::Long => write!(f, "{}", t!("Long_tricky_words")),
        }
    }
}
