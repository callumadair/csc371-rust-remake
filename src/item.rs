use std::collections::HashMap;

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
}