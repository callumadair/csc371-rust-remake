use std::collections::HashMap;
use std::fmt;
use serde::{Serialize, Deserialize, Serializer, ser::SerializeMap};

#[derive(Clone, Eq, Debug, Deserialize)]
pub(crate) struct Item {
    #[serde(flatten)]
    identifier: String,
    entries: HashMap<String, String>,
}

impl Item {
    pub(crate) fn new(identifier: String) -> Item {
        Item {
            identifier,
            entries: HashMap::new(),
        }
    }

    pub(crate) fn size(&self) -> usize {
        return self.entries.len();
    }

    pub(crate) fn empty(&self) -> bool {
        return self.entries.is_empty();
    }

    pub(crate) fn get_ident(&self) -> &String {
        return &self.identifier;
    }

    pub(crate) fn set_ident(&mut self, identifier: String) {
        self.identifier = identifier;
    }

    pub(crate) fn add_entry(&mut self, key: String, value: String) -> bool {
        if !self.entries.contains_key(&key) {
            self.entries.insert(key, value);
            return true;
        }
        return false;
    }

    pub(crate) fn merge_entries(&mut self, other: &mut Item) -> () {
        for (key, value) in other.entries.iter() {
            self.add_entry(key.clone(), value.clone());
        }
    }

    pub(crate) fn get_entry(&mut self, key: String) -> Option<String> {
        return self.entries.get_mut(&key).cloned();
    }

    pub(crate) fn delete_entry(&mut self, key: String) -> bool {
        if self.entries.contains_key(&key) {
            self.entries.remove(&key);
            return !self.entries.contains_key(&key);
        }
        return false;
    }
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for entry in self.entries.iter() {
            write!(f, "{}", serde_json::to_string(&entry).unwrap())?;
        }

        return write!(f, "{}", "");
    }
}

impl PartialEq<Self> for Item {
    fn eq(&self, other: &Self) -> bool {
        self.identifier == self.identifier && self.entries == other.entries
    }
}


impl Serialize for Item {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let mut map:<S as Serializer>::SerializeMap = serializer.serialize_map(Some(self.entries.len()))?;
        for (key, value) in &self.entries {
            map.serialize_entry(&key, &value)?;
        }
        map.end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        let item: Item = Item::new("Empty_Test".to_string());
        assert_eq!(item.size(), 0);
        assert!(item.empty());
    }

    #[test]
    fn test_entries_add() {
        let mut item: Item = Item::new("Entries_Test".to_string());

        let first_key: String = String::from("url");
        let first_val: String = String::from("https://www.google.com");

        assert_eq!(item.add_entry(first_key.clone(), first_val.clone()), true);
        assert_eq!(item.size(), 1);
        assert_ne!(item.empty(), true);
        assert_eq!(item.get_entry(String::from("url")).unwrap(), first_val);

        //The add another entry with the same key.
        assert_eq!(item.add_entry(first_key.clone(), first_val.clone()), false);
        assert_eq!(item.size(), 1);
        assert_ne!(item.empty(), true);

        //Now try with new key value pair.
        let second_key: String = String::from("username");
        let second_val: String = String::from("myusername");

        assert!(item.add_entry(second_key.clone(), second_val.clone()));
        assert_eq!(item.size(), 2);
        assert_ne!(item.empty(), true);
        assert_eq!(item.get_entry(second_key.clone()).unwrap(), second_val);
    }

    #[test]
    fn test_entries_delete() {
        let mut item: Item = Item::new("Test".to_string());

        let first_key: String = String::from("url");
        let first_val: String = String::from("https://www.google.com");

        //Validate the details of the entry are correct
        assert!(item.add_entry(first_key.clone(), first_val.clone()));
        assert_eq!(item.size(), 1);
        assert_ne!(item.empty(), true);
        assert_eq!(item.get_entry(first_key.clone()).unwrap(), first_val);

        //Delete non-existent entry and validate nothing changed.
        assert_eq!(item.delete_entry(String::from("username")), false);
        assert_eq!(item.get_entry(first_key.clone()).unwrap(), first_val);
        assert_eq!(item.size(), 1);

        //Now delete the real entry and validate it is gone.
        assert!(item.delete_entry(first_key.clone()));
        assert_eq!(item.size(), 0);
    }
}