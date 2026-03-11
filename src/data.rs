use anyhow::Result;
use rand::seq::SliceRandom;

use crate::{
    data::math_basics::{Operation, Operator},
    ui::viz::WordCategory,
};
pub mod math_basics;
pub mod verbs;

const TRICKY_WORDS_FILE: &str = "./Listen/Knifflige_Wörter.txt";
const TRICKY_LONG_WORDS_FILE: &str = "./Listen/Lange_knifflige_Wörter.txt";
const TRICKY_SHORT_WORDS_FILE: &str = "./Listen/Kurze_knifflige_Wörter.txt";

pub struct Data {
    pub verbs: Vec<verbs::Verb>,
    pub verb_indices: Vec<usize>,

    pub tricky_words: Vec<String>,
    pub tricky_long_words: Vec<String>,
    pub tricky_short_words: Vec<String>,
    pub tricky_words_indices: Vec<usize>,

    pub operation: Operation,
}
impl Data {
    pub fn new() -> anyhow::Result<Self> {
        let mut data = Data {
            verbs: verbs::read_from_file()?,
            tricky_words: read_list_from_file(TRICKY_WORDS_FILE)?,
            tricky_long_words: read_list_from_file(TRICKY_LONG_WORDS_FILE)?,
            tricky_short_words: read_list_from_file(TRICKY_SHORT_WORDS_FILE)?,
            operation: Operation::new(Operator::default()),
            verb_indices: vec![],
            tricky_words_indices: vec![],
        };
        data.reset_verb_indices();
        data.reset_tricky_words_indices(WordCategory::default());
        Ok(data)
    }

    pub fn reset_verb_indices(&mut self) {
        let mut rng = rand::rng();
        self.verb_indices = (0..self.verbs.len()).collect();
        self.verb_indices.shuffle(&mut rng);
    }

    pub fn reset_tricky_words_indices(&mut self, selected_category: WordCategory) {
        let word_list_len = match selected_category {
            WordCategory::Short => self.tricky_short_words.len(),
            WordCategory::Medium => self.tricky_words.len(),
            WordCategory::Long => self.tricky_long_words.len(),
        };
        self.tricky_words_indices = (0..word_list_len).collect();
        let mut rng = rand::rng();
        self.tricky_words_indices.shuffle(&mut rng);
    }
}

fn read_list_from_file(file: &str) -> Result<Vec<String>> {
    let list: Vec<String> = std::fs::read_to_string(file)?
        .lines()
        .map(ToString::to_string)
        .collect();
    Ok(list)
}
