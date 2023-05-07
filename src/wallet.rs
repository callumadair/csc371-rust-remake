use std::collections::HashMap;
use std::{fmt, fs};
use serde::{Serialize, Deserialize};
use serde_json::Value;
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

    pub(crate) fn new_category(&mut self, category_identifier: &String) -> &mut Category {
        if self.categories.contains_key(category_identifier) {
            return self.categories.get_mut(category_identifier).unwrap();
        }
        self.categories.insert(category_identifier.clone(), Category::new(category_identifier.clone()));
        return self.categories.get_mut(category_identifier).unwrap();
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

    pub(crate) fn get_category(&mut self, category_identifier: &String) -> &mut Category {
        if self.categories.contains_key(category_identifier) {
            return self.categories.get_mut(category_identifier).unwrap();
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

    pub(crate) fn load(&mut self, filename: &String) -> bool {
        let file_contents = fs::read_to_string(filename).unwrap();
        let wallet_values: Value = serde_json::from_str(&file_contents).unwrap();

        for (cat_ident, category) in wallet_values.as_object().unwrap() {
            let mut new_category = Category::new(cat_ident.clone());

            for (item_ident, item) in category.as_object().unwrap(){
                let mut new_item = new_category.new_item(item_ident);

                for (entry_ident, entry_val) in item.as_object().unwrap() {
                    new_item.add_entry(entry_ident.clone(), entry_val.to_string());
                    println!("Entry: {}, contents: {}", entry_ident, entry_val);
                }
            }

            println!("Category: {}, contents: {}", cat_ident, category)
        }
        return true;
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

