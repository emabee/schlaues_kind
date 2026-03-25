use anyhow::Result;
use rand::seq::SliceRandom;

pub struct Verbs {
    pub verbs: Vec<Verb>,
    indices: Vec<usize>,
}
impl Verbs {
    pub fn parse(raw: &'static str) -> Result<Verbs> {
        let mut verbs = Verbs {
            verbs: serde_json::from_str(raw)?,
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
    pub infinitiv: &'static str,
    pub praesens: Pronomen,
    pub praeteritum: Pronomen,
    pub perfekt: Pronomen,
}

#[derive(Default, serde::Deserialize, serde::Serialize)]
pub struct Pronomen {
    pub ich: &'static str,
    pub du: &'static str,
    pub er_sie_es: &'static str,
    pub wir: &'static str,
    pub ihr: &'static str,
    pub sie: &'static str,
}
