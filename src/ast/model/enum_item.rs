use crate::ast::{Id, ItemType, ModelDefinition, RecordItem, TupleItem};

#[derive(Debug, PartialEq)]
pub enum EnumItem<'a> {
    Item(Id<'a>),
    Record(Id<'a>, ItemType<'a>),
    Tuple(Id<'a>, ItemType<'a>),
}

impl<'a> EnumItem<'a> {
    pub fn new_item(id: &'a str) -> Self {
        Self::Item(id.into())
    }

    pub fn new_record(id: &'a str, items: Vec<RecordItem<'a>>) -> Self {
        let def = ModelDefinition::new_record(Id::Inline, items);
        Self::Record(id.into(), ItemType::new_inline(def))
    }

    pub fn new_tuple(id: &'a str, items: Vec<TupleItem<'a>>) -> Self {
        let def = ModelDefinition::new_tuple(Id::Inline, items);
        Self::Tuple(id.into(), ItemType::new_inline(def))
    }
}