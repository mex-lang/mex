use crate::ast::{EnumItem, Id, ItemType, ModelParamDefinition, RecordItem, TupleItem};

pub mod enum_item;
pub mod record_item;
pub mod tuple_item;
pub mod item_type;

#[derive(Debug, PartialEq)]
pub enum ModelDefinition<'a> {
    Fragment(Id<'a>, Vec<RecordItem<'a>>, Vec<ModelParamDefinition<'a>>),
    Record(Id<'a>, Vec<RecordItem<'a>>, Vec<ModelParamDefinition<'a>>),
    Tuple(Id<'a>, Vec<TupleItem<'a>>, Vec<ModelParamDefinition<'a>>),
    Enum(Id<'a>, Vec<EnumItem<'a>>, Vec<ModelParamDefinition<'a>>),
    Alias(Id<'a>, Vec<ModelParamDefinition<'a>>, Box<ItemType<'a>>),
    Scalar(Id<'a>)
}

impl<'a> ModelDefinition<'a> {
    pub fn new_fragment(id: Id<'a>, items: Option<Vec<RecordItem<'a>>>, params: Option<Vec<ModelParamDefinition<'a>>>) -> Self {
        Self::Fragment(id, items.unwrap_or(vec![]), params.unwrap_or(vec![]))
    }

    pub fn new_record(id: Id<'a>, items: Option<Vec<RecordItem<'a>>>, params: Option<Vec<ModelParamDefinition<'a>>>) -> Self {
        Self::Record(id, items.unwrap_or(vec![]), params.unwrap_or(vec![]))
    }

    pub fn new_tuple(id: Id<'a>, items: Option<Vec<TupleItem<'a>>>, params: Option<Vec<ModelParamDefinition<'a>>>) -> Self {
        Self::Tuple(id, items.unwrap_or(vec![]), params.unwrap_or(vec![]))
    }

    pub fn new_enum(id: Id<'a>, items: Vec<EnumItem<'a>>, params: Option<Vec<ModelParamDefinition<'a>>>) -> Self {
        Self::Enum(id, items, params.unwrap_or(vec![]))
    }

    pub fn new_alias(id: Id<'a>, params: Option<Vec<ModelParamDefinition<'a>>>, item_type: ItemType<'a>) -> Self {
        Self::Alias(id, params.unwrap_or(vec![]), Box::new(item_type))
    }

    pub fn new_scalar(id: Id<'a>) -> Self {
        Self::Scalar(id)
    }
}