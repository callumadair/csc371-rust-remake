use std::collections::HashMap;
use std::fmt;
use serde::{Serialize, Deserialize};
use crate::item::Item;

#[derive(Clone, Eq, Debug, Serialize, Deserialize)]
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
        return self.items.len();
    }

    pub(crate) fn empty(&self) -> bool {
        return self.items.is_empty();
    }

    pub(crate) fn get_ident(&self) -> &String {
        return &self.identifier;
    }

    pub(crate) fn set_ident(&mut self, identifier: String) -> () {
        self.identifier = identifier;
    }

    pub(crate) fn new_item(&mut self, item_identifier: &String) -> &mut Item {
        if self.items.contains_key(item_identifier) {
            return self.items.get_mut(item_identifier).unwrap();
        }
        self.items.insert(item_identifier.clone(), Item::new(item_identifier.clone()));
        return self.items.get_mut(item_identifier).unwrap();
    }

    pub(crate) fn add_item(&mut self, item: Item) -> bool {
        if self.items.contains_key(item.get_ident()) {
            return false;
        }
        self.items.insert(item.get_ident().clone(), item.clone());
        if self.items.contains_key(item.get_ident()) {
            return true;
        }
        return false;
    }

    fn merge_items(&mut self, other: &mut Category) -> () {
        for (key, value) in other.items.iter_mut() {
            if self.items.contains_key(key) {
                self.items.get_mut(key).unwrap().merge_entries(value);
            } else {
                self.items.insert(key.clone(), value.clone());
            }
        }
    }

    pub(crate) fn get_item(&mut self, item_identifier: &String) -> &mut Item {
        if self.items.contains_key(item_identifier) {
            return self.items.get_mut(item_identifier).unwrap();
        }
        panic!("Item {} not found in category {}", item_identifier, self.identifier);
    }

    pub(crate) fn delete_item(&mut self, item_identifier: &String) -> bool {
        if self.items.contains_key(item_identifier) {
            self.items.remove(item_identifier);
            return !self.items.contains_key(item_identifier);
        }
        return false;
    }
}

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for item in self.items.iter() {
            write!(f, "{}\n", serde_json::to_string(&item).unwrap())?;
        }
        return write!(f, "{}", "");
    }
}

impl PartialEq<Self> for Category {
    fn eq(&self, other: &Self) -> bool {
        self.items == other.items
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        let empty_category = Category::new("Empty_Test".to_string());
        assert_eq!(empty_category.size(), 0);
        assert!(empty_category.empty());
    }

    #[test]
    fn test_items_add() {
        let mut category = Category::new("Test_Cat".to_string());
        assert!(category.empty());

        let item_identifier = "Test_Item".to_string();
        let item = Item::new(item_identifier.clone());
        assert!(item.empty());
        assert!(category.add_item(item.clone()));
        assert_eq!(category.size(), 1);
        assert!(!category.empty());
        assert_eq!(category.get_item(&item_identifier), &item);
        assert!(!category.empty());

        //Now try to add a new item with the same identifier.
        let item2 = Item::new(item_identifier.clone());
        assert!(item2.empty());
        assert!(!category.add_item(item2.clone()));
        assert_eq!(category.size(), 1);
        assert!(!category.empty());

        //Now add another item with a different identifier.
        let item_identifier2 = "Test_Item2".to_string();
        let item3 = Item::new(item_identifier2.clone());
        assert!(item3.empty());
        assert!(category.add_item(item3.clone()));
        assert_eq!(category.size(), 2);
        assert!(!category.empty());
        assert_eq!(category.get_item(&item_identifier2), &item3);
    }

    #[test]
    fn test_items_delete() {
        let mut category = Category::new("Test_Cat".to_string());
        assert!(category.empty());

        let item_identifier = "Test_Item".to_string();
        let item = Item::new(item_identifier.clone());
        assert!(item.empty());
        assert!(category.add_item(item.clone()));
        assert_eq!(category.size(), 1);
        assert!(!category.empty());
        assert_eq!(category.get_item(&item_identifier), &item);
        assert!(!category.empty());

        //Now try to delete an item that doesn't exist.
        let item_identifier2 = "Test_Item2".to_string();
        assert!(!category.delete_item(&item_identifier2));
        assert_eq!(category.get_item(&item_identifier), &item);
        assert_eq!(category.size(), 1);
        assert!(!category.empty());

        //Now delete the item that does exist.
        assert!(category.delete_item(&item_identifier));
        //Check if an error is thrown when trying to get the item.
        //assert_eq!(category.get_item(&item_identifier), some error here);
        assert_eq!(category.size(), 0);
        assert!(category.empty());
    }
}