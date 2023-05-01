use std::collections::HashMap;
use std::fmt;
use crate::item::Item;

pub(crate) struct Category {
    identifier: String,
    items: HashMap<String, Item>,
}

impl Category {
    pub(crate) fn new(identifier: String) -> Category {
        Category {
            identifier,
            items: HashMap::new(),
        }
    }

    pub(crate) fn size(&self) -> usize {
        return 0;
    }

    pub(crate) fn empty(&self) -> bool {
        return false;
    }

    pub(crate) fn get_ident(&self) -> &String {
        return &self.identifier;
    }

    pub(crate) fn set_ident(&mut self, identifier: String) -> () {
        self.identifier = identifier;
    }

    pub(crate) fn new_item(&mut self, item_identifier: &String) -> Item {
        let item = Item::new(item_identifier.to_string());
        return item;
    }

    pub(crate) fn add_item(&mut self, item: Item) -> bool {
        return false;
    }

    fn merge_items(&mut self, other: &mut Category) -> () {}

    pub(crate) fn get_item(&self, item_identifier: &String) -> Option<&Item> {
        return None;
    }

    pub(crate) fn delete_item(&mut self, item_identifier: &String) -> bool {
        return false;
    }
}

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{}", "");
    }
}

impl PartialEq<Self> for Category {
    fn eq(&self, other: &Self) -> bool {
        self.items == other.items
    }
}

impl Eq for Category {}