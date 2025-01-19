mod source;
mod error;
mod scope;
mod model;

pub use source::*;
pub use error::*;
pub use scope::*;
pub use model::*;
pub use model::record_item::*;
pub use model::tuple_item::*;
pub use model::enum_item::*;
pub use model::item_type::*;

#[derive(Debug, PartialEq)]
pub enum Branch<'input> {
    Version(&'input str)
}

#[derive(Debug, PartialEq)]
pub enum Id<'input> {
    Name(&'input str),
    Index(i32),
    Branch(&'input str, Branch<'input>),
    Inline,
}

impl<'input> From<Option<Id<'input>>> for Id<'input> {
    fn from(value: Option<Id<'input>>) -> Self {
        match value {
            Some(value) => value,
            None => Id::Inline,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Literal<'input> {
    String(&'input str),
    Number(&'input str)
}

