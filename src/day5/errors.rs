use crate::util::lexer;

#[derive(Debug, PartialEq, Eq)]
pub enum CrateInputError {
    ParsingFailed(lexer::Error),
    EmptySpaceUnderCrate { line: String, stack_num: usize },
    NoEmptyLineAfterSeparator,
    IdenticalStackNumbers { line: String },
    StackNumbersTooBig { line: String },
    NotEnoughCrates { line: String },
}

impl std::fmt::Display for CrateInputError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for CrateInputError {}

impl From<lexer::Error> for CrateInputError {
    fn from(err: lexer::Error) -> Self {
        CrateInputError::ParsingFailed(err)
    }
}
