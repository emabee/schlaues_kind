// pub struct Dictates<'a>(Vec<Dictate<'a>>);

#[derive(Default)]
pub struct Dictate<'a> {
    pub number: String,
    pub title: &'a str,
    pub lines: Vec<&'a str>,
}

impl<'a> Dictate<'a> {
    pub fn new_list(input: &'a str) -> Vec<Dictate<'a>> {
        let lines = input.lines();
        let mut dictates = Vec::<Dictate>::new();

        let mut dictate = Dictate::default();
        for line in lines {
            if line.is_empty() {
                dictates.push(std::mem::take(&mut dictate));
            } else if line.starts_with("Diktat ") {
                let e = line.find(" - ").unwrap();
                dictate.number = line[7..e]
                    .parse::<usize>()
                    .unwrap_or_else(|_| panic!("{}", line[7..e].to_string()))
                    .to_string();
                dictate.title = &line[e + 3..];
            } else {
                dictate.lines.push(line);
            }
        }
        dictates
    }
}

#[cfg(test)]
mod test {
    use super::{super::DICTATES_2, Dictate};
    #[test]
    fn test_1() {
        let dictates = Dictate::new_list(DICTATES_2);
        assert_eq!(dictates[25].title, "Im Bus");
        assert_eq!(dictates.len(), 50);
        assert_eq!(
            dictates[44].lines[5],
            "Zum Glück hat sich niemand verletzt."
        );
    }
}
