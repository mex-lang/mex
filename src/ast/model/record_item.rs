use crate::ast::{Id, ItemType};

#[derive(Debug, PartialEq)]
pub enum RecordItem<'a> {
    Item(Id<'a>, ItemType<'a>),
    Spread(ItemType<'a>)
}

impl<'a> RecordItem<'a> {
    pub fn new_item(id: Id<'a>, item_type: ItemType<'a>) -> Self {
        Self::Item(id, item_type)
    }

    pub fn new_spread(item_type: ItemType<'a>) -> Self {
        Self::Spread(item_type)
    }
}