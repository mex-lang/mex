use crate::ast::Id;

#[derive(Debug, PartialEq)]
pub enum ItemType<'a> {
    Name(Id<'a>),
    //Inline(ModelDefinition<'a>),
    //Model(Box<Scope::Model>, Vec<ModelParam>),
    //Generic(Box<Scope::Model>, Box<ModelParamDefinition::Generic>)
}

impl<'a> ItemType<'a> {
    pub fn from_name(name: &'a str) -> Self {
        ItemType::Name(name.into())
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