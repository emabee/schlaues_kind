mod dictates;
mod math_basics;
mod verbs;
mod word_list;

pub use dictates::Dictate;
pub use math_basics::{Operation, Operator};
pub use verbs::{Pronomen, Verb, Verbs};
pub use word_list::WordList;

use crate::assets::{
    DICTATE_2, DICTATE_34, DICTATES, IRREGULAR_VERBS, LONG_TRICKY_WORDS, MEDIUM_TRICKY_WORDS,
    SHORT_TRICKY_WORDS,
};
use anyhow::Result;
use rodio::{DeviceSinkBuilder, MixerDeviceSink};

pub struct Data<'a> {
    pub sink: MixerDeviceSink,
    pub verbs: Verbs,
    pub word_lists: [WordList; 3],
    pub operation: Operation,
    pub dictates: [Vec<Dictate<'a>>; 2],
    pub score: usize,
}
impl Data<'_> {
    pub fn new() -> Result<Self> {
        Ok(Data {
            sink: {
                let mut sink =
                    DeviceSinkBuilder::open_default_sink().expect("open default audio stream");
                sink.log_on_drop(false);
                sink
            },
            verbs: Verbs::parse(IRREGULAR_VERBS)?,
            word_lists: [
                WordList::new(&t!("Short_tricky_words"), SHORT_TRICKY_WORDS),
                WordList::new(&t!("Medium_tricky_words"), MEDIUM_TRICKY_WORDS),
                WordList::new(&t!("Long_tricky_words"), LONG_TRICKY_WORDS),
            ],
            operation: Operation::new(Operator::default()),
            dictates: [
                Dictate::new_list(DICTATES[DICTATE_2]),
                Dictate::new_list(DICTATES[DICTATE_34]),
            ],
            score: 0,
        })
    }
}
