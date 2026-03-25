use crate::data::Operator;

pub struct V {
    // pub base_state: BaseState,
    pub modal_state: ModalState,
}
impl V {
    pub fn new() -> Self {
        Self {
            // base_state: BaseState::default(),
            modal_state: ModalState::None,
        }
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
        word_list_index: usize,
        word_idx: usize,
    },

    BasicMath {
        current_operator: Operator,
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
    pub fn is_ready_for_modal(&self) -> bool {
        matches!(self, ModalState::None)
    }
    pub fn get_id(&self) -> &'static str {
        match self {
            ModalState::None => "None",
            ModalState::About => "About",
            ModalState::DeclineVerbs { .. } => "DeclineVerbs",
            ModalState::ReadTrickyWords { .. } => "ReadTrickyWords",
            ModalState::BasicMath { .. } => "BasicMath",
        }
    }
}
