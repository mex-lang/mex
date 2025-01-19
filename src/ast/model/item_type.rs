use crate::ast::{Id, Literal, ModelDefinition, TupleItem};

#[derive(Debug, PartialEq)]
pub enum ItemType<'input> {
    Model(Id<'input>, Vec<ModelParam<'input>>),
    Inline(ModelDefinition<'input>),
}

impl<'input> ItemType<'input> {
    pub fn new_name(name: Id<'input>, params: Option<Vec<ModelParam<'input>>>) -> Self {
        ItemType::Model(name, params.unwrap_or(vec!()))
    }

    pub fn new_inline(inline: ModelDefinition<'input>) -> Self {
        ItemType::Inline(inline)
    }

    pub fn new_inline_tuple(items: Vec<TupleItem<'input>>) -> Self {
        ItemType::Inline(
            ModelDefinition::new_tuple(Id::Inline, items.into(), vec![].into())
        )
    }
}

#[derive(Debug, PartialEq)]
pub enum GenericConstraintDefinition<'input> {
    Contains(ItemType<'input>),
}

#[derive(Debug, PartialEq)]
pub enum ModelParamDefinition<'input> {
    Generic {
        id: Id<'input>,
        constraint_type: Option<ItemType<'input>>,
    },
    Metadata {
        id: Id<'input>,
        type_id: ItemType<'input>,
        def_value: Option<Literal<'input>>,
    },
    Constraint {
        id: Id<'input>,
        constraint: GenericConstraintDefinition<'input>,
    }
}

impl<'input> ModelParamDefinition<'input> {
    pub fn new_generic(id: Id<'input>, constraint_type: Option<ItemType<'input>>) -> Self {
        ModelParamDefinition::Generic { id, constraint_type }
    }

    pub fn new_metadata(id: Id<'input>, type_id: ItemType<'input>, def_value: Option<Literal<'input>>) -> Self {
        ModelParamDefinition::Metadata { id, type_id, def_value }
    }
}

#[derive(Debug, PartialEq)]
pub enum ModelParam<'input> {
    Generic(ItemType<'input>),
    Metadata(Id<'input>, Literal<'input>)
}

impl<'input> ModelParam<'input> {
    pub fn new_generic(item_type: ItemType<'input>) -> Self {
        ModelParam::Generic(item_type)
    }

    pub fn new_metadata(id: Id<'input>, value: Literal<'input>) -> Self {
        ModelParam::Metadata(id, value)
    }
}