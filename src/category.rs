use crate::item::Item;
use serde::{ser::SerializeMap, Deserialize, Serialize, Serializer};
use std::{collections::BTreeMap, fmt};

#[derive(Clone, Eq, Debug, Deserialize)]
pub(crate) struct Category {
    identifier: String,
    items: BTreeMap<String, Item>,
}

impl Category {
    pub(crate) fn new(identifier: String) -> Category {
        Category {
            identifier,
            items: BTreeMap::new(),
        }
    }

    pub(crate) fn size(&self) -> usize {
        self.items.len()
    }

    pub(crate) fn empty(&self) -> bool {
        self.items.is_empty()
    }

    pub(crate) fn get_ident(&self) -> &String {
        &self.identifier
    }

    pub(crate) fn set_ident(&mut self, identifier: &String) {
        self.identifier = identifier.clone();
    }

    pub(crate) fn new_item(&mut self, item_identifier: &String) -> &mut Item {
        if self.items.contains_key(item_identifier) {
            return self.items.get_mut(item_identifier).unwrap();
        }
        self.items
            .insert(item_identifier.clone(), Item::new(item_identifier.clone()));
        self.items.get_mut(item_identifier).unwrap()
    }

    pub(crate) fn add_item(&mut self, item: &Item) -> bool {
        self.items
            .insert(item.get_ident().clone(), item.clone())
            .is_none()
    }

    fn merge_items(&mut self, other: &mut Category) {
        for (key, value) in other.items.iter_mut() {
            if self.items.contains_key(key) {
                self.items.get_mut(key).unwrap().merge_entries(value);
            } else {
                self.items.insert(key.clone(), value.clone());
            }
        }
    }

    pub(crate) fn get_item(&mut self, item_identifier: &String) -> Option<&mut Item> {
        return self.items.get_mut(item_identifier);
    }

    pub(crate) fn delete_item(&mut self, item_identifier: &String) -> bool {
        self.items.remove(item_identifier).is_some()
    }
}

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for item in self.items.iter() {
            write!(f, "{}", serde_json::to_string(&item).unwrap())?;
        }
        write!(f, "{}", "")
    }
}

impl PartialEq<Self> for Category {
    fn eq(&self, other: &Self) -> bool {
        self.items == other.items
    }
}

impl Serialize for Category {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map: <S as Serializer>::SerializeMap =
            serializer.serialize_map(Some(self.items.len()))?;

        for (item_identifier, item_contents) in &self.items {
            map.serialize_entry(&item_identifier, &item_contents)?;
        }
        map.end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        let empty_category: Category = Category::new("Empty_Test".to_string());
        assert_eq!(empty_category.size(), 0);
        assert!(empty_category.empty());
    }

    #[test]
    fn test_items_add() {
        let mut category: Category = Category::new("Test_Cat".to_string());
        assert!(category.empty());

        let item_identifier: String = "Test_Item".to_string();
        let item: Item = Item::new(item_identifier.clone());
        assert!(item.empty());
        assert!(category.add_item(&item));
        assert_eq!(category.size(), 1);
        assert!(!category.empty());
        assert_eq!(category.get_item(&item_identifier).unwrap(), &item);
        assert!(!category.empty());

        //Now try to add a new item with the same identifier.
        let item2: Item = Item::new(item_identifier.clone());
        assert!(item2.empty());
        assert!(!category.add_item(&item2));
        assert_eq!(category.size(), 1);
        assert!(!category.empty());

        //Now add another item with a different identifier.
        let item_identifier2: String = "Test_Item2".to_string();
        let item3: Item = Item::new(item_identifier2.clone());
        assert!(item3.empty());
        assert!(category.add_item(&item3));
        assert_eq!(category.size(), 2);
        assert!(!category.empty());
        assert_eq!(category.get_item(&item_identifier2).unwrap(), &item3);
    }

    #[test]
    fn test_items_delete() {
        let mut category: Category = Category::new("Test_Cat".to_string());
        assert!(category.empty());

        let item_identifier: String = "Test_Item".to_string();
        let item: Item = Item::new(item_identifier.clone());
        assert!(item.empty());
        assert!(category.add_item(&item));
        assert_eq!(category.size(), 1);
        assert!(!category.empty());
        assert_eq!(category.get_item(&item_identifier).unwrap(), &item);
        assert!(!category.empty());

        //Now try to delete an item that doesn't exist.
        let item_identifier2: String = "Test_Item2".to_string();
        assert!(!category.delete_item(&item_identifier2));
        assert_eq!(category.get_item(&item_identifier).unwrap(), &item);
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
