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
    Anonymous,
    Branch(&'input str, Branch<'input>)
}

impl<'input> From<&'input str> for Id<'input> {
    fn from(value: &'input str) -> Self {
        Id::Name(value)
    }
}

// #[derive(Debug)]
// pub enum Literal {
//     String(String),
//     Number(String),
//     Integer(i64),
// }

