use std::num::ParseIntError;

#[derive(Default, Debug, Clone, PartialEq)]
pub enum LexicalError {
    #[default]
    InvalidToken,
    InvalidInteger(ParseIntError),
}

impl From<ParseIntError> for LexicalError {
    fn from(err: ParseIntError) -> Self {
        LexicalError::InvalidInteger(err)
    }
}