use rand::seq::SliceRandom;

pub struct WordList {
    description: String,
    pub words: Vec<String>,
    indices: Vec<usize>,
}
impl WordList {
    pub fn new(description: &str, raw: &str) -> WordList {
        let words = Self::read_from(raw);
        WordList {
            description: description.to_string(),
            indices: Self::new_indices(&words),
            words,
        }
    }
    fn read_from(raw: &str) -> Vec<String> {
        raw.lines().map(ToString::to_string).collect()
    }
    pub fn description(&self) -> &str {
        &self.description
    }
    pub fn next_word_index(&mut self) -> usize {
        if self.indices.is_empty() {
            self.indices = Self::new_indices(&self.words);
        }
        self.indices.pop().unwrap(/* OK */)
    }

    fn new_indices(words: &[String]) -> Vec<usize> {
        let mut rng = rand::rng();
        let mut indices: Vec<usize> = (0..words.len()).collect();
        indices.shuffle(&mut rng);
        indices
    }
}
