use std::cell::RefCell;
use std::rc::Rc;
use lalrpop_util::ErrorRecovery;
use crate::ast::*;
use crate::lexer::{LexicalError, Token};

#[derive(Debug, PartialEq)]
pub enum Scope<'input> {
    Global(Vec<RefScope<'input>>),
    Package(Id<'input>, Vec<RefScope<'input>>),
    Model(ModelDefinition<'input>),
    Error(ErrorRecovery<usize, Token<'input>, LexicalError>),
}

impl<'input> Scope<'input> {
    pub fn new_package(name: &'input str, items: Vec<RefScope<'input>>) -> RefScope<'input> {
        Scope::Package(Id::Name(name), items).into()
    }

    pub fn new_record(name: &'input str, items: Vec<RecordItem<'input>>) -> RefScope<'input> {
        let def = ModelDefinition::new_record(name.into(), items);
        Scope::Model(def).into()
    }

    pub fn new_tuple(name: &'input str, items: Vec<TupleItem<'input>>) -> RefScope<'input> {
        let def = ModelDefinition::new_tuple(name.into(), items);
        Scope::Model(def).into()
    }

    pub fn new_enum(name: &'input str, items: Vec<EnumItem<'input>>) -> RefScope<'input> {
        let def = ModelDefinition::new_enum(name.into(), items);
        Scope::Model(def).into()
    }

    pub fn new_fragment(name: &'input str, items: Vec<RecordItem<'input>>) -> RefScope<'input> {
        let def = ModelDefinition::new_fragment(name.into(), items);
        Scope::Model(def).into()
    }

    pub fn new_scalar(name: &'input str) -> RefScope<'input> {
        let def = ModelDefinition::new_scalar(name.into());
        Scope::Model(def).into()
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