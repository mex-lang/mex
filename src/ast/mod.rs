mod source;
mod error;
mod scope;

pub use source::*;
pub use error::*;
pub use scope::*;

#[derive(Debug)]
pub enum Branch<'input> {
    Version(&'input str)
}

#[derive(Debug)]
pub enum Id<'input> {
    Name(&'input str),
    Index(i32),
    Anonymous,
    Branch(&'input str, Branch<'input>)
}

// #[derive(Debug)]
// pub enum Literal {
//     String(String),
//     Number(String),
//     Integer(i64),
// }

// #[derive(Debug)]
// pub enum Ref {
//     Model(Box<Scope::Model>, Vec<ModelParam>),
//     Generic(Box<Scope::Model>, Box<ModelParamDefinition::Generic>)
// }

// #[derive(Debug)]
// pub enum DicItemDefinition {
//     Item(Name, Ref::Model),
//     Spread(Ref::Model)
// }

// #[derive(Debug)]
// pub enum TupleItemDefinition {
//     Item(Name, Ref::Model),
// }

// #[derive(Debug)]
// pub enum EnumItemDefinition {
//     Item(Ref::Model)
// }

// #[derive(Debug)]
// pub enum GenericConstraintDefinition {
//     Model(Box<Scope::Model>),
// }

// #[derive(Debug)]
// pub enum ModelParamDefinition {
//     Generic(Name::Common, Vec<GenericConstraintDefinition>),
//     Meta(Name::Common, Box<Scope::Model>, Option<Literal>)
// }

// #[derive(Debug)]
// pub enum ModelParam {
//     GenericRef(Box<ModelParamDefinition::Generic>, Ref::Generic),
//     Generic(Box<ModelParamDefinition::Generic>, Ref::Model),
//     Meta(Box<ModelParamDefinition::Meta>, Literal)
// }

// #[derive(Debug)]
// pub enum ModelDefinition {
//     Dic(Name, Vec<DicItemDefinition>, Vec<ModelParamDefinition>),
//     Tuple(Name, Vec<TupleItemDefinition>, Vec<ModelParamDefinition>),
//     Enum(Name, Vec<EnumItemDefinition>, Vec<ModelParamDefinition>),
//     Scalar(Name::Common)
// }

