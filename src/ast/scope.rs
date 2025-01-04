use lalrpop_util::ErrorRecovery;
use crate::ast::Id;
use crate::lexer::{LexicalError, Token};

#[derive(Debug)]
pub enum Scope<'input> {
    Package(Id<'input>, Vec<Box<Scope<'input>>>),
    //Model(ModelDefinition),
    //Fragment(ModelDefinition),

    Error(ErrorRecovery<usize, Token<'input>, LexicalError>),

    Nop
}

impl<'input> Scope<'input> {
    pub fn new_package(name: &'input str) -> Box<Self> {
        Box::new(Scope::Package(Id::Name(name), vec![]))
    }

    pub fn add_into_root(mut root: Box<Self>, item: Box<Self>) -> Box<Self> {
        match *root {
            Scope::Package(_, ref mut children) => children.push(item),
            _ => {}
        };
        root
    }
}