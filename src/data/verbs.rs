use anyhow::Result;
use rand::seq::SliceRandom;

pub struct Verbs {
    pub verbs: Vec<Verb>,
    indices: Vec<usize>,
}
impl Verbs {
    pub fn read_from_file(file: &str) -> Result<Verbs> {
        let mut verbs = Verbs {
            verbs: serde_json::from_str(&std::fs::read_to_string(file)?)?,
            indices: Vec::default(),
        };
        verbs.reset_indices();
        Ok(verbs)
    }
    fn reset_indices(&mut self) {
        let mut rng = rand::rng();
        self.indices = (0..self.verbs.len()).collect();
        self.indices.shuffle(&mut rng);
    }
    pub fn next_index(&mut self) -> usize {
        if self.indices.is_empty() {
            self.reset_indices();
        }
        self.indices.pop().unwrap(/* OK */)
    }
}

#[derive(Default, serde::Deserialize, serde::Serialize)]
pub struct Verb {
    pub infinitiv: String,
    pub praesens: Pronomen,
    pub praeteritum: Pronomen,
    pub perfekt: Pronomen,
}

#[derive(Default, serde::Deserialize, serde::Serialize)]
pub struct Pronomen {
    pub ich: String,
    pub du: String,
    pub er_sie_es: String,
    pub wir: String,
    pub ihr: String,
    pub sie: String,
}
