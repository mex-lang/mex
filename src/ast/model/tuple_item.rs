use crate::ast::{Id, ItemType};

#[derive(Debug, PartialEq)]
pub enum TupleItem<'a> {
    Item(ItemType<'a>),
    NamedItem(Id<'a>, ItemType<'a>),
}

impl<'a> TupleItem<'a> {
    pub fn new_item(type_id: &'a str) -> Self {
        Self::Item(ItemType::Name(type_id.into()))
    }

    pub fn new_named_item(id: &'a str, type_id: &'a str) -> Self {
        Self::NamedItem(id.into(), ItemType::Name(type_id.into()))
    }
}