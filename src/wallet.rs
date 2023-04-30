use std::collections::HashMap;
use std::fmt;
use crate::category::Category;

pub(crate) struct Wallet {
    categories: HashMap<String, Category>,
}

impl Wallet {
    fn new() -> Wallet {
        Wallet {
            categories: HashMap::new(),
        }
    }

    fn size() -> usize {
        return 0;
    }

    fn empty() -> bool {
        return false;
    }

    fn new_category(&mut self, category_identifier: &String) -> Category {
        let category = Category::new(category_identifier.to_string());
        return category;
    }

    fn add_category(category: Category) -> bool {
        return false;
    }

    fn delete_category(category_identifier: &String) -> bool {
        return false;
    }

    fn load(filename: &String) -> bool {
        return false;
    }

    fn save(filename: &String) -> bool {
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
