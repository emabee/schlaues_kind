use anyhow::Result;

const IRREGULAR_VERBS_FILE: &str = "./Listen/Alle_irregulären_Verben.txt";

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

pub fn read_from_file() -> Result<Vec<Verb>> {
    let verbs: Vec<Verb> = serde_json::from_str(&std::fs::read_to_string(IRREGULAR_VERBS_FILE)?)?;
    Ok(verbs)
}
