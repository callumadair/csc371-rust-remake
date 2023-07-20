use std::{collections::BTreeMap, fmt, fs};
use serde::{Serialize, Deserialize, Serializer, ser::SerializeMap};
use serde_json::Value;
use crate::category::Category;

#[derive(Clone, Eq, Debug, Deserialize)]
pub(crate) struct Wallet {
    categories: BTreeMap<String, Category>,
}

impl Wallet {
    pub(crate) fn new() -> Wallet {
        Wallet {
            categories: BTreeMap::new(),
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
        return self.categories.insert(category.get_ident().clone(), category.clone()).is_none();
    }

    pub(crate) fn get_category(&mut self, category_identifier: &String) -> Option<&mut Category> {
        if self.categories.contains_key(category_identifier) {
            return self.categories.get_mut(category_identifier);
        }
        panic!("Category {} does not exist", category_identifier);
    }

    pub(crate) fn delete_category(&mut self, category_identifier: &String) -> bool {
        return self.categories.remove(category_identifier).is_some();
    }

    pub(crate) fn load(&mut self, filename: &String) -> bool {
        let file_contents: String = fs::read_to_string(filename).unwrap();
        let wallet_values: Value = serde_json::from_str(&file_contents).unwrap();

        for (cat_ident, category) in wallet_values.as_object().unwrap() {
            let mut new_category = self.new_category(cat_ident);

            for (item_ident, item) in category.as_object().unwrap() {
                let mut new_item = new_category.new_item(item_ident);

                for (entry_ident, entry_val) in item.as_object().unwrap() {
                    new_item.add_entry(&entry_ident, &entry_val.to_string());
                }
            }
        }
        return true;
    }

    pub(crate) fn save(&self, filename: &String) -> bool {
        let json_val: String = serde_json::to_string(&self).unwrap();
        fs::write(filename, json_val).expect("Unable to write file");
        return true;
    }
}

impl fmt::Display for Wallet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for category in self.categories.iter() {
            write!(f, "{}", serde_json::to_string(&category).unwrap())?;
        }
        return write!(f, "{}", "");
    }
}

impl PartialEq<Self> for Wallet {
    fn eq(&self, other: &Self) -> bool {
        self.categories == other.categories
    }
}

impl Serialize for Wallet {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let mut map: <S as Serializer>::SerializeMap = serializer.serialize_map(Some(self.categories.len()))?;

        for (category_identifier, category_contents) in &self.categories {
            map.serialize_entry(&category_identifier, &category_contents)?;
        }
        map.end()
    }
}

#[cfg(test)]
mod tests {
    use std::{path::Path, fs, io::Write};
    use crate::item::Item;
    use super::*;

    #[test]
    fn test_empty() {
        let wallet: Wallet = Wallet::new();
        assert_eq!(wallet.size(), 0);
        assert!(wallet.empty());
    }

    #[test]
    fn test_categories_add() {
        let mut wallet: Wallet = Wallet::new();
        assert!(wallet.empty());

        let first_cat_ident: String = String::from("Test");
        let first_category: Category = Category::new(first_cat_ident.clone());
        assert!(wallet.empty());
        assert!(wallet.add_category(first_category.clone()));
        assert_eq!(wallet.size(), 1);
        assert!(!wallet.empty());
        assert_eq!(wallet.get_category(&first_cat_ident).unwrap(), &first_category);

        let second_category: Category = Category::new(first_cat_ident.clone());
        assert!(second_category.empty());
        assert!(!wallet.add_category(second_category.clone()));
        assert_eq!(wallet.size(), 1);
        assert!(!wallet.empty());

        let third_cat_ident: String = String::from("Test2");
        let third_category: Category = Category::new(third_cat_ident.clone());
        assert!(third_category.empty());
        assert!(wallet.add_category(third_category.clone()));
        assert_eq!(wallet.size(), 2);
        assert!(!wallet.empty());
        assert_eq!(wallet.get_category(&third_cat_ident).unwrap(), &third_category);
    }

    #[test]
    fn test_categories_delete() {
        let mut wallet: Wallet = Wallet::new();
        assert!(wallet.empty());
        let first_cat_ident: String = String::from("Test");
        let first_category: Category = Category::new(first_cat_ident.clone());
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
        let file_path = String::from("./tests/testload.json");
        assert!(Path::new(&file_path).exists());

        let data: String = String::from(r#"{
            "Bank Accounts":{
                "Starling":{
                    "Account Number":"12345678",
                    "Name":"Mr John Doe",
                    "Sort Code":"12-34-56"
                }
            },
            "Websites":{
                "Facebook":{
                    "password":"pass1234fb",
                    "url":"https://www.facebook.com/",
                    "username":"example@gmail.com"
                    },
                "Google":{
                    "password":"pass1234",
                    "url":"https://www.google.com/",
                    "username":"example@gmail.com"
                },
                "Twitter":{
                    "password":"r43rfsffdsfdsf",
                    "url":"https://www.twitter.com/",
                    "username":"example@gmail.com"
                    }
                }
            }"#);

        let mut file: fs::File = fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&file_path)
            .expect("Unable to open file");
        assert!(file.write(data.as_bytes()).is_ok());

        let mut wallet: Wallet = Wallet::new();
        assert!(wallet.empty());
        //do error checking here instead of boolean checks.
        assert!(wallet.load(&file_path));
        assert_eq!(wallet.size(), 2);

        let web: String = String::from("Websites");
        assert!(wallet.get_category(&web).is_some());
        assert_eq!(wallet.get_category(&web).unwrap().size(), 3);

        let google: String = String::from("Google");
        assert!(wallet.get_category(&web).unwrap()
            .get_item(&google).is_some());
        assert_eq!(wallet.get_category(&web).unwrap()
                       .get_item(&google).unwrap().size(), 3);
        assert_eq!(wallet.get_category(&web).unwrap()
                       .get_item(&google).unwrap()
                       .get_entry(&String::from("url")).unwrap(),
                   &String::from("https://www.google.com/"));
        assert_eq!(wallet.get_category(&web).unwrap()
                       .get_item(&google).unwrap()
                       .get_entry(&String::from("username")).unwrap(),
                   &String::from("example@gmail.com"));
        assert_eq!(wallet.get_category(&web).unwrap()
                       .get_item(&google).unwrap()
                       .get_entry(&String::from("password")).unwrap(),
                   &String::from("pass1234"));

        let facebook: String = String::from("Facebook");
        assert!(wallet.get_category(&web).unwrap()
            .get_item(&facebook).is_some());
        assert_eq!(wallet.get_category(&web).unwrap()
                       .get_item(&facebook).unwrap().size(), 3);
        assert_eq!(wallet.get_category(&web).unwrap()
                       .get_item(&facebook).unwrap()
                       .get_entry(&String::from("url")).unwrap(),
                   &String::from("https://www.facebook.com/"));
        assert_eq!(wallet.get_category(&web).unwrap()
                       .get_item(&facebook).unwrap()
                       .get_entry(&String::from("username")).unwrap(),
                   &String::from("example@gmail.com"));
        assert_eq!(wallet.get_category(&web).unwrap()
                       .get_item(&facebook).unwrap()
                       .get_entry(&String::from("password")).unwrap(),
                   &String::from("pass1234fb"));

        let twitter: String = String::from("Twitter");
        assert!(wallet.get_category(&web).unwrap()
            .get_item(&twitter).is_some());
        assert_eq!(wallet.get_category(&web).unwrap()
                       .get_item(&twitter).unwrap().size(), 3);
        assert_eq!(wallet.get_category(&web).unwrap()
                       .get_item(&twitter).unwrap()
                       .get_entry(&String::from("url")).unwrap(),
                   &String::from("https://www.twitter.com/"));
        assert_eq!(wallet.get_category(&web).unwrap()
                       .get_item(&twitter).unwrap()
                       .get_entry(&String::from("username")).unwrap(),
                   &String::from("example@gmail.com"));
        assert_eq!(wallet.get_category(&web).unwrap()
                       .get_item(&twitter).unwrap()
                       .get_entry(&String::from("password")).unwrap(),
                   &String::from("r43rfsffdsfdsf"));

        let bank: String = String::from("Bank Accounts");
        assert!(wallet.get_category(&bank).is_some());
        assert_eq!(wallet.get_category(&bank).unwrap().size(), 1);


        let starling: String = String::from("Starling");
        assert!(wallet.get_category(&bank).unwrap()
            .get_item(&starling).is_some());
        assert_eq!(wallet.get_category(&bank).unwrap()
                       .get_item(&starling).unwrap().size(), 3);
        assert_eq!(wallet.get_category(&bank).unwrap()
                       .get_item(&starling).unwrap()
                       .get_entry(&String::from("Name")).unwrap(),
                   &String::from("Mr John Doe"));
        assert_eq!(wallet.get_category(&bank).unwrap()
                       .get_item(&starling).unwrap()
                       .get_entry(&String::from("Account Number")).unwrap(), &String::from("12345678"));
        assert_eq!(wallet.get_category(&bank).unwrap()
                       .get_item(&starling).unwrap()
                       .get_entry(&String::from("Sort Code")).unwrap(), &String::from("12-34-56"));
    }

    #[test]
    fn test_save_json_file() {
        let file_path: String = String::from("./tests/testsave.json");
        assert!(Path::new(&file_path).exists());
        let mut file: fs::File = fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&file_path)
            .expect("Unable to open file");
        assert!(file.write_all("{}".as_bytes()).is_ok());

        let mut wallet = Wallet::new();
        assert!(wallet.empty());

        let ident_1: String = String::from("ident_1");
        let ident_2: String = String::from("ident_2");

        let entry_key_1 = String::from("key_1");
        let entry_key_2 = String::from("key_2");

        let entry_value_1: String = String::from("value_1");
        let entry_value_2: String = String::from("value_2");

        let mut item_1: Item = Item::new(ident_1.clone());
        let mut item_2: Item = Item::new(ident_2.clone());
        item_1.add_entry(&entry_key_1, &entry_value_1);
        item_1.add_entry(&entry_key_2, &entry_value_2);
        item_2.add_entry(&entry_key_1, &entry_value_1);

        assert_eq!(item_1.size(), 2);
        assert_eq!(item_2.size(), 1);

        let mut cat_1: Category = Category::new(ident_1.clone());
        let mut cat_2: Category = Category::new(ident_2.clone());
        cat_1.add_item(&item_1);
        cat_1.add_item(&item_2);
        cat_2.add_item(&item_1);

        assert_eq!(cat_1.size(), 2);
        assert_eq!(cat_2.size(), 1);

        wallet.add_category(cat_1);
        wallet.add_category(cat_2);

        assert_eq!(wallet.size(), 2);
        assert!(wallet.save(&file_path));
        let file_contents: String = fs::read_to_string(&file_path).expect("Unable to read file");
        let expected_contents: &str = r#"{"ident_1":{"ident_1":{"key_1":"value_1","key_2":"value_2"},"ident_2":{"key_1":"value_1"}},"ident_2":{"ident_1":{"key_1":"value_1","key_2":"value_2"}}}"#;
        assert_eq!(file_contents, expected_contents);
    }
}