use std::collections::HashMap;
use std::{fmt, fs};
use serde::{Serialize, Deserialize};
use crate::category::Category;

#[derive(Clone, Eq, Debug, Serialize, Deserialize)]
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
        return self.categories.len();
    }

    pub(crate) fn empty(&self) -> bool {
        return self.categories.is_empty();
    }

    pub(crate) fn new_category(&mut self, category_identifier: &String) -> &Category {
        if self.categories.contains_key(category_identifier) {
            return self.categories.get(category_identifier).unwrap();
        }
        self.categories.insert(category_identifier.clone(), Category::new(category_identifier.clone()));
        return self.categories.get(category_identifier).unwrap();
    }

    pub(crate) fn add_category(&mut self, category: Category) -> bool {
        if self.categories.contains_key(category.get_ident()) {
            return false;
        }
        self.categories.insert(category.get_ident().clone(), category.clone());
        if self.categories.contains_key(category.get_ident()) {
            return true;
        }
        return false;
    }

    pub(crate) fn get_category(&self, category_identifier: &String) -> &Category {
        if self.categories.contains_key(category_identifier) {
            return self.categories.get(category_identifier).unwrap();
        }
        panic!("Category {} does not exist", category_identifier);
    }

    pub(crate) fn delete_category(&mut self, category_identifier: &String) -> bool {
        if self.categories.contains_key(category_identifier) {
            self.categories.remove(category_identifier);
            return !self.categories.contains_key(category_identifier);
        }
        return false;
    }

    pub(crate) fn load(&self, filename: &String) -> bool {
        let file_contents = fs::read_to_string(filename).unwrap();

        let json: serde_json::Value = serde_json::from_str(&file_contents).unwrap();
        println!("JSON: {}", json);

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

