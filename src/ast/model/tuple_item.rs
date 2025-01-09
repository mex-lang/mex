use crate::ast::{Id, ItemType};

#[derive(Debug, PartialEq)]
pub enum TupleItem<'a> {
    Item(ItemType<'a>),
    NamedItem(Id<'a>, ItemType<'a>),
}

impl<'a> TupleItem<'a> {
    pub fn new_item(item_type: ItemType<'a>) -> Self {
        Self::Item(item_type)
    }

    pub fn new_named_item(id: &'a str, item_type: ItemType<'a>) -> Self {
        Self::NamedItem(id.into(), item_type)
    }
}