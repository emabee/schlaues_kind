mod dictates;
mod math_basics;
mod verbs;
mod word_list;

pub use dictates::Dictate;
pub use math_basics::{Operation, Operator};
pub use verbs::{Pronomen, Verb, Verbs};
pub use word_list::WordList;

use anyhow::Result;
use rodio::{DeviceSinkBuilder, MixerDeviceSink};

const IRREGULAR_VERBS: &str = include_str!("./assets/lists/Alle_irregulären_Verben.txt");
const MEDIUM_TRICKY_WORDS: &str = include_str!("./assets/lists/Knifflige_Wörter.txt");
const LONG_TRICKY_WORDS: &str = include_str!("./assets/lists/Lange_knifflige_Wörter.txt");
const SHORT_TRICKY_WORDS: &str = include_str!("./assets/lists/Kurze_knifflige_Wörter.txt");
const DICTATES_2: &str = include_str!("./assets/lists/Diktate 2.txt");
const DICTATES_34: &str = include_str!("./assets/lists/Diktate AI 3 4.txt");

pub struct Data<'a> {
    pub sink: MixerDeviceSink,
    pub verbs: Verbs,
    pub word_lists: [WordList; 3],
    pub operation: Operation,
    pub dictates_2: Vec<Dictate<'a>>,
    pub dictates_34: Vec<Dictate<'a>>,
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
            dictates_2: Dictate::new_list(DICTATES_2),
            dictates_34: Dictate::new_list(DICTATES_34),
            score: 0,
        })
    }
}
