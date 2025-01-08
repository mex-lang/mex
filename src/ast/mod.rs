mod source;
mod error;
mod scope;

pub use source::*;
pub use error::*;
pub use scope::*;

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

#[derive(Debug, PartialEq)]
pub enum ItemType<'a> {
    Model(Id<'a>),
    //Model(Box<Scope::Model>, Vec<ModelParam>),
    //Generic(Box<Scope::Model>, Box<ModelParamDefinition::Generic>)
}

#[derive(Debug, PartialEq)]
pub enum ModelItemDefinition<'a> {
    Item(Id<'a>, ItemType<'a>),
    Spread(ItemType<'a>)
}

impl<'a> ModelItemDefinition<'a> {
    pub fn new_item(id: &'a str, type_id: &'a str) -> Self {
        Self::Item(id.into(), ItemType::Model(type_id.into()))
    }

    pub fn new_spread(type_id: &'a str) -> Self {
        Self::Spread(ItemType::Model(type_id.into()))
    }
}

#[derive(Debug, PartialEq)]
pub enum TupleItemDefinition<'a> {
    Item(Id<'a>, ItemType<'a>),
}

#[derive(Debug, PartialEq)]
pub enum EnumItemDefinition<'a> {
    Item(ItemType<'a>)
}

// #[derive(Debug)]
// pub enum GenericConstraintDefinition {
//     Model(Box<Scope::Model>),
// }

#[derive(Debug, PartialEq)]
pub enum ModelParamDefinition {
    //Generic(Name::Common, Vec<GenericConstraintDefinition>),
    //Meta(Name::Common, Box<Scope::Model>, Option<Literal>)
}

// #[derive(Debug)]
// pub enum ModelParam {
//     GenericRef(Box<ModelParamDefinition::Generic>, Ref::Generic),
//     Generic(Box<ModelParamDefinition::Generic>, Ref::Model),
//     Meta(Box<ModelParamDefinition::Meta>, Literal)
// }

#[derive(Debug, PartialEq)]
pub enum ModelDefinition<'a> {
    Record(Id<'a>, Vec<ModelItemDefinition<'a>>, Vec<ModelParamDefinition>),
    Tuple(Id<'a>, Vec<TupleItemDefinition<'a>>, Vec<ModelParamDefinition>),
    Enum(Id<'a>, Vec<EnumItemDefinition<'a>>, Vec<ModelParamDefinition>),
    Fragment(Id<'a>, Vec<ModelItemDefinition<'a>>, Vec<EnumItemDefinition<'a>>),
    Scalar(Id<'a>)
}

impl<'a> ModelDefinition<'a> {
    pub fn new_record(id: Id<'a>, items: Vec<ModelItemDefinition<'a>>) -> Self {
        Self::Record(id, items, vec![])
    }

    pub fn new_fragment(id: Id<'a>, items: Vec<ModelItemDefinition<'a>>) -> Self {
        Self::Fragment(id, items, vec![])
    }

    pub fn new_scalar(id: Id<'a>) -> Self {
        Self::Scalar(id)
    }
}

