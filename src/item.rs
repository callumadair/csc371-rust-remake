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

    fn size() -> usize {
        return 0;
    }

    fn empty() -> bool {
        return false;
    }

    fn get_ident() -> String {
        return self::identifier;
    }

    fn set_ident(identifier: String) {
        self::identifier = identifier;
    }

    fn add_entry(key: String, value: String) -> bool {
        return false;
    }

    fn merge_entries(other: &mut Item) -> () {}

    fn get_entry(key: String) -> Option<String> {
        return None;
    }

    fn delete_entry(key: String) -> bool {
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