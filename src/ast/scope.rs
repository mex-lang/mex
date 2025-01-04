use std::cell::RefCell;
use std::rc::Rc;
use lalrpop_util::ErrorRecovery;
use crate::ast::Id;
use crate::lexer::{LexicalError, Token};

#[derive(Debug, PartialEq)]
pub enum Scope<'input> {
    Global(Vec<RefScope<'input>>),
    Package(Id<'input>, Vec<RefScope<'input>>),
    //Model(ModelDefinition),
    //Fragment(ModelDefinition),

    Error(ErrorRecovery<usize, Token<'input>, LexicalError>),

    Nop
}

impl<'input> Scope<'input> {
    pub fn new_package(name: &'input str) -> RefScope<'input> {
        Scope::Package(Id::Name(name), vec![]).into()
    }

    pub fn add_space(root: RefScope<'input>, item: RefScope<'input>) -> RefScope<'input> {
        match **root.borrow_mut() {
            Scope::Package(_, ref mut children) => children.push(item.clone()),
            Scope::Global(ref mut children) => children.push(item.clone()),
            _ => {}
        };
        item
    }
}

pub type RefScope<'input> = Rc<RefCell<Box<Scope<'input>>>>;

impl<'input> Into<RefScope<'input>> for Scope<'input> {
    fn into(self) -> RefScope<'input> {
        Rc::new(RefCell::new(Box::new(self)))
    }
}