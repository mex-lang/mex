use crate::ast::{Id, ItemType};

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

    pub fn new_record(id: &'a str, item_type: ItemType<'a>) -> Self {
        Self::Record(id.into(), item_type)
    }

    pub fn new_tuple(id: &'a str, item_type: ItemType<'a>) -> Self {
        Self::Tuple(id.into(), item_type)
    }
}