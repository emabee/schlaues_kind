#[derive(Default, Debug)]
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
    ChangeTrickyWordCategory,

    ShowMathBasics,
    NextCalculation,
    ShowResult,

    ShowAbout,
    // Cancel,
    CloseModal,
}
