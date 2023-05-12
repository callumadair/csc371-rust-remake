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

    pub(crate) fn get_category(&mut self, category_identifier: &String) -> Option<&mut Category> {
        if self.categories.contains_key(category_identifier) {
            return self.categories.get_mut(category_identifier);
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
    use std::{string::String, path::Path, fs};
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
        assert_eq!(wallet.get_category(&first_cat_ident).unwrap(), &first_category);

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
        assert_eq!(wallet.get_category(&third_cat_ident).unwrap(), &third_category);
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
        assert_eq!(wallet.get_category(&first_cat_ident).unwrap(), &first_category);
        assert!(wallet.delete_category(&first_cat_ident));
        //add exception check here
        assert_eq!(wallet.size(), 0);
    }

    #[test]
    fn test_load_json_file() {
        let file_path = String::from("./tests/testdatabasealt.json");
        assert!(Path::new(&file_path).exists());

        let data = String::from("{\"Bank Accounts\":{\"Starling\":{\"Account
                  Number\":\"12345678\",\"Name\":\"Mr John Doe\",\"Sort
                  Code\":\"12-34-56\"}},\"Websites\":{\"Facebook\":{
                  \"password\":\"pass1234fb\",\"url\":\"https://
                  www.facebook.com/
                  \",\"username\":\"example@gmail.com\"},\"Google\":{
                  \"password\":\"pass1234\",\"url\":\"https://www.google.com/
                  \",\"username\":\"example@gmail.com\"},\"Twitter\":{
                  \"password\":\"r43rfsffdsfdsf\",\"url\":\"https://
                  www.twitter.com/\",\"username\":\"example@gmail.com\"}}}");
        fs::write(&file_path, data).expect("Unable to write file");

        let mut wallet = Wallet::new();
        assert!(wallet.empty());
        //do error checking here instead of boolean checks.
        assert!(wallet.load(&file_path));
        assert_eq!(wallet.size(), 2);

        assert!(wallet.get_category(&String::from("Websites")).is_some());
        assert_eq!(wallet.get_category(&String::from("Websites")).unwrap().size(), 3);

        assert!(wallet.get_category(&String::from("Websites")).unwrap()
            .get_item(&String::from("Google")).is_some());
        assert_eq!(wallet.get_category(&String::from("Websites")).unwrap()
                       .get_item(&String::from("Google")).unwrap().size(), 3);
        assert_eq!(wallet.get_category(&String::from("Websites")).unwrap()
                       .get_item(&String::from("Google")).unwrap()
                       .get_entry(String::from("url")).unwrap(),
                   String::from("https://www.google.com/"));
        assert_eq!(wallet.get_category(&String::from("Websites")).unwrap()
                       .get_item(&String::from("Google")).unwrap()
                       .get_entry(String::from("username")).unwrap(),
                   String::from("example@gmail.com"));
        assert_eq!(wallet.get_category(&String::from("Websites")).unwrap()
                       .get_item(&String::from("Google")).unwrap()
                       .get_entry(String::from("password")).unwrap(),
                   String::from("pass1234"));

        assert!(wallet.get_category(&String::from("Websites")).unwrap()
            .get_item(&String::from("Facebook")).is_some());
        assert_eq!(wallet.get_category(&String::from("Websites")).unwrap()
                       .get_item(&String::from("Facebook")).unwrap().size(), 3);
        assert_eq!(wallet.get_category(&String::from("Websites")).unwrap()
                       .get_item(&String::from("Facebook")).unwrap()
                       .get_entry(String::from("url")).unwrap(),
                   String::from("https://www.facebook.com/"));
        assert_eq!(wallet.get_category(&String::from("Websites")).unwrap()
                       .get_item(&String::from("Facebook")).unwrap()
                       .get_entry(String::from("username")).unwrap(),
                   String::from("example@gmail.com"));
        assert_eq!(wallet.get_category(&String::from("Websites")).unwrap()
                       .get_item(&String::from("Facebook")).unwrap()
                       .get_entry(String::from("password")).unwrap(),
                   String::from("pass1234fb"));

        assert!(wallet.get_category(&String::from("Websites")).unwrap()
            .get_item(&String::from("Twitter")).is_some());
        assert_eq!(wallet.get_category(&String::from("Websites")).unwrap()
                       .get_item(&String::from("Twitter")).unwrap().size(), 3);
        assert_eq!(wallet.get_category(&String::from("Websites")).unwrap()
                       .get_item(&String::from("Twitter")).unwrap()
                       .get_entry(String::from("url")).unwrap(),
                   String::from("https://www.twitter.com/"));
        assert_eq!(wallet.get_category(&String::from("Websites")).unwrap()
                       .get_item(&String::from("Twitter")).unwrap()
                       .get_entry(String::from("username")).unwrap(),
                   String::from("example@gmail.com"));
        assert_eq!(wallet.get_category(&String::from("Websites")).unwrap()
                       .get_item(&String::from("Twitter")).unwrap()
                       .get_entry(String::from("password")).unwrap(),
                   String::from("r43rfsffdsfdsf"));

        assert!(wallet.get_category(&String::from("Bank Accounts")).is_some());
        assert_eq!(wallet.get_category(&String::from("Bank Accounts")).unwrap().size(), 1);
        assert!(wallet.get_category(&String::from("Bank Accounts")).unwrap()
            .get_item(&String::from("Starling")).is_some());
        assert_eq!(wallet.get_category(&String::from("Bank Accounts")).unwrap()
                       .get_item(&String::from("Starling")).unwrap().size(), 3);
        assert_eq!(wallet.get_category(&String::from("Bank Accounts")).unwrap()
                       .get_item(&String::from("Starling")).unwrap()
                       .get_entry(String::from("Name")).unwrap(),
                   String::from("Mr John Doe"));
        assert_eq!(wallet.get_category(&String::from("Bank Accounts")).unwrap()
                       .get_item(&String::from("Starling")).unwrap()
                       .get_entry(String::from("Account Number")).unwrap(), String::from("12345678"));
        assert_eq!(wallet.get_category(&String::from("Bank Accounts")).unwrap()
                       .get_item(&String::from("Starling")).unwrap()
                       .get_entry(String::from("Sort Code")).unwrap(), String::from("12-34-56"));
    }
}