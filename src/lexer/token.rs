use std::fmt;
use logos::{Lexer, Logos, Skip};
use crate::lexer::LexicalError;

#[derive(Logos, Clone, Debug, PartialEq)]
#[logos(error = super::error::LexicalError)]
#[logos(extras = (usize, usize))]
pub enum Token<'input> {
    #[token("package")]
    KeywordPackage,
    #[token("model")]
    KeywordModel,
    #[token("enum")]
    KeywordEnum,
    #[token("fragment")]
    KeywordFragment,
    #[token("scalar")]
    KeywordScalar,

    #[regex("[_a-zA-Z][_0-9a-zA-Z]*", |lex| lex.slice())]
    Identifier(&'input str),
    #[regex("[1-9][0-9]*", |lex| lex.slice())]
    Number(&'input str),
    //#[regex("[1-9][0-9]*", |lex| lex.slice().parse())]
    //Integer(i64),

    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("{")]
    LBracket,
    #[token("}")]
    RBracket,
    #[token("<")]
    Lees,
    #[token(">")]
    Greater,
    #[token("[")]
    LSquare,
    #[token("]")]
    RSquare,

    #[token("?")]
    Question,
    #[token("=")]
    Assign,
    #[token(";")]
    Semicolon,
    #[token(":")]
    Colon,
    #[token(",")]
    Comma,
    #[token("...")]
    Spread,

    #[regex(r"\/\/.*\n?", comment_callback)]
    Comment,

    #[regex(r"\n", newline_callback)]
    Newline,

    #[regex(r"[ \t\f]+", whitespace_callback)]
    Whitespace,

    Error(LexicalError),
}

impl<'input> fmt::Display for Token<'input> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

fn newline_callback<'input>(lex: &mut Lexer<'input, Token<'input>>) -> Skip {
    lex.extras.0 += 1;               // line
    lex.extras.1 = lex.span().end;   // column
    Skip
}

fn comment_callback<'input>(lex: &mut Lexer<'input, Token<'input>>) -> Skip {
    lex.extras.0 += 1;               // line
    lex.extras.1 = lex.span().end;   // column
    Skip
}

fn whitespace_callback<'input>(lex: &mut Lexer<'input, Token<'input>>) -> Skip {
    lex.extras.1 = lex.span().end;   // column
    Skip
}