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

    fn size() -> usize {
        return 0;
    }

    fn empty() -> bool {
        return false;
    }

    fn get_ident() -> String {
        return self::identifier;
    }

    fn set_ident(identifier: String) -> () {
        self::identifier = identifier;
    }

    fn new_item(&mut self, item_identifier: &String) -> Item {
        let item = Item::new(item_identifier.to_string());
        return item;
    }

    fn add_item(item: Item) -> bool {
        return false;
    }

    fn merge_items(other: &mut Category) -> () {}

    fn get_item(item_identifier: &String) -> Option<&Item> {
        return None;
    }

    fn delete_item(item_identifier: &String) -> bool {
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