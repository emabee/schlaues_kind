mod math_basics;
mod verbs;
mod word_list;

pub use math_basics::{Operation, Operator};
use rodio::MixerDeviceSink;
pub use verbs::{Pronomen, Verb, Verbs};
pub use word_list::WordList;

use anyhow::Result;

const IRREGULAR_VERBS: &str = include_str!("./assets/lists/Alle_irregulären_Verben.txt");
const MEDIUM_TRICKY_WORDS: &str = include_str!("./assets/lists/Knifflige_Wörter.txt");
const LONG_TRICKY_WORDS: &str = include_str!("./assets/lists/Lange_knifflige_Wörter.txt");
const SHORT_TRICKY_WORDS: &str = include_str!("./assets/lists/Kurze_knifflige_Wörter.txt");

pub struct Data {
    pub sink: MixerDeviceSink,
    pub verbs: Verbs,
    pub word_lists: [WordList; 3],
    pub operation: Operation,
    pub score: usize,
}
impl Data {
    pub fn new() -> Result<Self> {
        Ok(Data {
            sink: rodio::DeviceSinkBuilder::open_default_sink().expect("open default audio stream"),
            verbs: Verbs::parse(IRREGULAR_VERBS)?,
            word_lists: [
                WordList::new(&t!("Short_tricky_words"), SHORT_TRICKY_WORDS),
                WordList::new(&t!("Medium_tricky_words"), MEDIUM_TRICKY_WORDS),
                WordList::new(&t!("Long_tricky_words"), LONG_TRICKY_WORDS),
            ],
            operation: Operation::new(Operator::default()),
            score: 0,
        })
    }
}
