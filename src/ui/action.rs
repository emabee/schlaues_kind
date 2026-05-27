#[derive(Default, Copy, Clone, Debug)]
pub(crate) enum Action {
    #[default]
    None,

    DeclineVerbs,
    NextDeclination,
    NextVerb,
    RestartVerb,
    PrintVerbs,

    ReadTrickyWords,
    NextTrickyWord,
    ChangeTrickyWordList(usize),

    ShowMathBasics,
    NextCalculation,
    ShowResult,

    Dictate(usize),
    NextDictate,

    ShowAbout,
    // Cancel,
    CloseModal,
}
