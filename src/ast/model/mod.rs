use crate::ast::{EnumItem, Id, ModelParamDefinition, RecordItem, TupleItem};

pub mod enum_item;
pub mod record_item;
pub mod tuple_item;
pub mod item_type;

#[derive(Debug, PartialEq)]
pub enum ModelDefinition<'a> {
    Fragment(Id<'a>, Vec<RecordItem<'a>>, Vec<ModelParamDefinition>),
    Record(Id<'a>, Vec<RecordItem<'a>>, Vec<ModelParamDefinition>),
    Tuple(Id<'a>, Vec<TupleItem<'a>>, Vec<ModelParamDefinition>),
    Enum(Id<'a>, Vec<EnumItem<'a>>, Vec<ModelParamDefinition>),
    Scalar(Id<'a>)
}

impl<'a> ModelDefinition<'a> {
    pub fn new_fragment(id: Id<'a>, items: Vec<RecordItem<'a>>) -> Self {
        Self::Fragment(id, items, vec![])
    }

    pub fn new_record(id: Id<'a>, items: Vec<RecordItem<'a>>) -> Self {
        Self::Record(id, items, vec![])
    }

    pub fn new_tuple(id: Id<'a>, items: Vec<TupleItem<'a>>) -> Self {
        Self::Tuple(id, items, vec![])
    }

    pub fn new_enum(id: Id<'a>, items: Vec<EnumItem<'a>>) -> Self {
        Self::Enum(id, items, vec![])
    }

    pub fn new_scalar(id: Id<'a>) -> Self {
        Self::Scalar(id)
    }
}