use std::collections::HashMap;
use std::fmt;

pub(crate) struct Item {
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
        return 0;
    }

    pub(crate) fn empty(&self) -> bool {
        return false;
    }

    pub(crate) fn get_ident(&self) -> &String {
        return &self.identifier;
    }

    pub(crate) fn set_ident(&mut self, identifier: String) {
        self.identifier = identifier;
    }

    pub(crate) fn add_entry(&mut self, key: String, value: String) -> bool {
        return false;
    }

    pub(crate) fn merge_entries(&mut self, other: &mut Item) -> () {}

    pub(crate) fn get_entry(&self, key: String) -> Option<String> {
        return None;
    }

    pub(crate) fn delete_entry(&mut self, key: String) -> bool {
        return false;
    }
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{}", "");
    }
}

impl PartialEq<Self> for Item {
    fn eq(&self, other: &Self) -> bool {
        self.entries == other.entries
    }
}

impl Eq for Item {}