use crate::ast::{Id, ModelDefinition};

#[derive(Debug, PartialEq)]
pub enum ItemType<'a> {
    Name(Id<'a>),
    Inline(ModelDefinition<'a>),
    //Model(Box<Scope::Model>, Vec<ModelParam>),
    //Generic(Box<Scope::Model>, Box<ModelParamDefinition::Generic>)
}

impl<'a> ItemType<'a> {
    pub fn new_name(name: &'a str) -> Self {
        ItemType::Name(name.into())
    }

    pub fn new_inline(inline: ModelDefinition<'a>) -> Self {
        ItemType::Inline(inline)
    }
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