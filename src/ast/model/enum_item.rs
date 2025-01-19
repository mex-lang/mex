use crate::ast::{Id, ItemType, ModelDefinition, RecordItem, TupleItem};

#[derive(Debug, PartialEq)]
pub enum EnumItem<'a> {
    Item(Id<'a>),
    Record(Id<'a>, ItemType<'a>),
    Tuple(Id<'a>, ItemType<'a>),
    Enum(Id<'a>, ItemType<'a>),
}

impl<'a> EnumItem<'a> {
    pub fn new_item(id: Id<'a>) -> Self {
        Self::Item(id)
    }

    pub fn new_record(id: Id<'a>, items: Vec<RecordItem<'a>>) -> Self {
        let def = ModelDefinition::new_record(Id::Inline, items.into(), vec![].into());
        Self::Record(id, ItemType::new_inline(def))
    }

    pub fn new_tuple(id: Id<'a>, items: Vec<TupleItem<'a>>) -> Self {
        let def = ModelDefinition::new_tuple(Id::Inline, items.into(), vec![].into());
        Self::Tuple(id, ItemType::new_inline(def))
    }

    pub fn new_enum(id: Id<'a>, items: Vec<EnumItem<'a>>) -> Self {
        let def = ModelDefinition::new_enum(Id::Inline, items.into(), vec![].into());
        Self::Enum(id, ItemType::new_inline(def))
    }
}