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

            for (item_ident, item) in category.as_object().unwrap() {
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
        let json_val = serde_json::to_string(&self).unwrap();
        fs::write(filename, json_val).expect("Unable to write file");
        return true;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        let wallet = Wallet::new();
        assert_eq!(wallet.size(), 0);
        assert!(wallet.empty());
    }

    #[test]
    fn test_categories_add() {
        let mut wallet = Wallet::new();
        assert!(wallet.empty());

        let first_cat_ident = String::from("Test");
        let first_category = Category::new(first_cat_ident.clone());
        assert!(wallet.empty());
        assert!(wallet.add_category(first_category.clone()));
        assert_eq!(wallet.size(), 1);
        assert!(!wallet.empty());
        assert_eq!(wallet.get_category(&first_cat_ident), &first_category);

        let second_category = Category::new(first_cat_ident.clone());
        assert!(second_category.empty());
        assert!(!wallet.add_category(second_category.clone()));
        assert_eq!(wallet.size(), 1);
        assert!(!wallet.empty());

        let third_cat_ident = String::from("Test2");
        let third_category = Category::new(third_cat_ident.clone());
        assert!(third_category.empty());
        assert!(wallet.add_category(third_category.clone()));
        assert_eq!(wallet.size(), 2);
        assert!(!wallet.empty());
        assert_eq!(wallet.get_category(&third_cat_ident), &third_category);
    }

    #[test]
    fn test_categories_delete() {
        let mut wallet = Wallet::new();
        assert!(wallet.empty());
        let first_cat_ident = String::from("Test");
        let first_category = Category::new(first_cat_ident.clone());
        assert!(wallet.empty());
        assert!(wallet.add_category(first_category.clone()));
        assert_eq!(wallet.size(), 1);
        assert!(!wallet.empty());
        assert_eq!(wallet.get_category(&first_cat_ident), &first_category);
        assert!(wallet.delete_category(&first_cat_ident));
        //add exception check here
        assert_eq!(wallet.size(), 0);
    }
}