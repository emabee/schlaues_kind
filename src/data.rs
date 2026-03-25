mod math_basics;
mod verbs;
mod word_list;

pub use math_basics::{Operation, Operator};
pub use verbs::{Pronomen, Verb, Verbs};
pub use word_list::WordList;

use anyhow::Result;

const IRREGULAR_VERBS_FILE: &str = "./Listen/Alle_irregulären_Verben.txt";
const TRICKY_WORDS_FILE: &str = "./Listen/Knifflige_Wörter.txt";
const TRICKY_LONG_WORDS_FILE: &str = "./Listen/Lange_knifflige_Wörter.txt";
const TRICKY_SHORT_WORDS_FILE: &str = "./Listen/Kurze_knifflige_Wörter.txt";

pub struct Data {
    pub verbs: Verbs,
    pub word_lists: [WordList; 3],
    pub operation: Operation,
}
impl Data {
    pub fn new() -> Result<Self> {
        Ok(Data {
            verbs: Verbs::read_from_file(IRREGULAR_VERBS_FILE)?,
            word_lists: [
                WordList::new(t!("Short_tricky_words"), TRICKY_SHORT_WORDS_FILE)?,
                WordList::new(t!("Medium_tricky_words"), TRICKY_WORDS_FILE)?,
                WordList::new(t!("Long_tricky_words"), TRICKY_LONG_WORDS_FILE)?,
            ],
            operation: Operation::new(Operator::default()),
        })
    }
}
