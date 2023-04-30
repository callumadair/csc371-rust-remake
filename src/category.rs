use std::collections::HashMap;
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

    fn new_item(&mut self, item_identifier: &String) -> Item {
        let item = Item::new(item_identifier.to_string());
        return item;
    }
}