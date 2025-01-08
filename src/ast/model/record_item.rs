use crate::ast::{Id, ItemType};

#[derive(Debug, PartialEq)]
pub enum RecordItem<'a> {
    Item(Id<'a>, ItemType<'a>),
    Spread(ItemType<'a>)
}

impl<'a> RecordItem<'a> {
    pub fn new_item(id: &'a str, type_id: &'a str) -> Self {
        Self::Item(id.into(), ItemType::Name(type_id.into()))
    }

    pub fn new_spread(type_id: &'a str) -> Self {
        Self::Spread(ItemType::Name(type_id.into()))
    }
}