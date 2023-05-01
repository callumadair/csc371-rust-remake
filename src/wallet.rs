use std::collections::HashMap;
use std::fmt;
use crate::category::Category;

pub(crate) struct Wallet {
    categories: HashMap<String, Category>,
}

impl Wallet {
    pub(crate) fn new() -> Wallet {
        Wallet {
            categories: HashMap::new(),
        }
    }

    pub(crate) fn size(&self) -> usize {
        return 0;
    }

    pub(crate) fn empty(&self) -> bool {
        return false;
    }

    pub(crate) fn new_category(&mut self, category_identifier: &String) -> Category {
        let category = Category::new(category_identifier.to_string());
        return category;
    }

    pub(crate) fn add_category(&mut self, category: Category) -> bool {
        return false;
    }

    pub(crate) fn delete_category(&mut self, category_identifier: &String) -> bool {
        return false;
    }

    pub(crate) fn load(&self, filename: &String) -> bool {
        return false;
    }

    pub(crate) fn save(&self, filename: &String) -> bool {
        return false;
    }
}

impl fmt::Display for Wallet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{}", "");
    }
}

impl PartialEq<Self> for Wallet {
    fn eq(&self, other: &Self) -> bool {
        self.categories == other.categories
    }
}

impl Eq for Wallet {}
