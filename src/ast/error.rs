use lalrpop_util::ParseError;
use crate::lexer::{LexicalError, Token};

#[derive(Debug)]
pub enum Error<'input> {
    Io(std::io::Error),
    Parsing(ParseError<usize, Token<'input>, LexicalError>),
}

impl<'input> From<std::io::Error> for Error<'input> {
    fn from(value: std::io::Error) -> Self {
        Error::Io(value)
    }
}

impl<'input> From<ParseError<usize, Token<'input>, LexicalError>> for Error<'input> {
    fn from(value: ParseError<usize, Token<'input>, LexicalError>) -> Self {
        Error::Parsing(value)
    }
}