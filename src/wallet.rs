use crate::{category::Category, error::WalletError};
use serde::{ser::SerializeMap, Deserialize, Serialize, Serializer};
use serde_json::Value;
use std::{collections::BTreeMap, fmt, fs};
use unescape::unescape;

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
        self.categories.len()
    }

    pub(crate) fn empty(&self) -> bool {
        self.categories.is_empty()
    }

    pub(crate) fn new_category(
        &mut self,
        category_identifier: &String,
    ) -> Result<&mut Category, WalletError> {
        if self.categories.contains_key(category_identifier) {
            return Ok(self.categories.get_mut(category_identifier).unwrap());
        }
        let result = self.categories.insert(
            category_identifier.clone(),
            Category::new(category_identifier.clone()),
        );

        if result.is_some() {
            return Ok(self.categories.get_mut(category_identifier).unwrap());
        }
        Err(WalletError::CreationError)
    }

    pub(crate) fn add_category(&mut self, category: Category) -> Result<bool, WalletError> {
        let result = self
            .categories
            .insert(category.get_ident().clone(), category.clone());

        if result.is_none() {
            return Ok(result.is_none());
        }

        Err(WalletError::InsertionError)
    }

    pub(crate) fn get_category(
        &mut self,
        category_identifier: &String,
    ) -> Result<&mut Category, WalletError> {
        let category_option = self.categories.get_mut(category_identifier);

        if let Some(category) = category_option {
            return Ok(category);
        }
        Err(WalletError::RetrievalError)
    }

    pub(crate) fn delete_category(
        &mut self,
        category_identifier: &String,
    ) -> Result<bool, WalletError> {
        let result = self.categories.remove(category_identifier);

        if result.is_some() {
            return Ok(result.is_some());
        }
        Err(WalletError::DeletionError)
    }

    pub(crate) fn load(&mut self, filename: &String) -> Result<bool, WalletError> {
        let file_contents: String = fs::read_to_string(filename).unwrap();
        let wallet_values: Value = serde_json::from_str(&file_contents).unwrap();

        for (cat_ident, category) in wallet_values.as_object().unwrap() {
            let new_category = self.new_category(cat_ident)?;

            for (item_ident, item) in category.as_object().unwrap() {
                let new_item = new_category.new_item(item_ident)?;

                for (entry_ident, entry_val) in item.as_object().unwrap() {
                    let entry_val = unescape(entry_val.as_str().unwrap());
                    new_item.add_entry(entry_ident, &entry_val.unwrap())?;
                }
            }
        }
        Ok(true)
    }

    pub(crate) fn save(&self, filename: &String) -> Result<bool, WalletError> {
        let json_val = serde_json::to_string(&self);
        if json_val.is_err() {
            return Err(WalletError::SaveError);
        }

        let write_result = fs::write(filename, json_val.unwrap());
        if write_result.is_err() {
            return Err(WalletError::SaveError);
        }

        Ok(true)
    }
}

impl fmt::Display for Wallet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for category in self.categories.iter() {
            write!(f, "{}", serde_json::to_string(&category).unwrap())?;
        }
        write!(f, "{}", "")
    }
}

impl PartialEq<Self> for Wallet {
    fn eq(&self, other: &Self) -> bool {
        self.categories == other.categories
    }
}

impl Serialize for Wallet {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map: <S as Serializer>::SerializeMap =
            serializer.serialize_map(Some(self.categories.len()))?;

        for (category_identifier, category_contents) in &self.categories {
            map.serialize_entry(&category_identifier, &category_contents)?;
        }
        map.end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::item::Item;
    use std::{fs, io::Write, path::Path};

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
        assert!(wallet.add_category(first_category.clone()).unwrap());
        assert_eq!(wallet.size(), 1);
        assert!(!wallet.empty());
        assert_eq!(
            wallet.get_category(&first_cat_ident).unwrap(),
            &first_category
        );

        let second_category: Category = Category::new(first_cat_ident.clone());
        assert!(second_category.empty());
        assert!(wallet.add_category(second_category.clone()).is_err());
        assert_eq!(wallet.size(), 1);
        assert!(!wallet.empty());

        let third_cat_ident: String = String::from("Test2");
        let third_category: Category = Category::new(third_cat_ident.clone());
        assert!(third_category.empty());
        assert!(wallet.add_category(third_category.clone()).unwrap());
        assert_eq!(wallet.size(), 2);
        assert!(!wallet.empty());
        assert_eq!(
            wallet.get_category(&third_cat_ident).unwrap(),
            &third_category
        );
    }

    #[test]
    fn test_categories_delete() {
        let mut wallet: Wallet = Wallet::new();
        assert!(wallet.empty());
        let first_cat_ident: String = String::from("Test");
        let first_category: Category = Category::new(first_cat_ident.clone());
        assert!(wallet.empty());
        assert!(wallet.add_category(first_category.clone()).unwrap());
        assert_eq!(wallet.size(), 1);
        assert!(!wallet.empty());
        assert_eq!(
            wallet.get_category(&first_cat_ident).unwrap(),
            &first_category
        );
        assert!(wallet.delete_category(&first_cat_ident).unwrap());
        //add exception check here
        assert_eq!(wallet.size(), 0);
    }

    #[test]
    fn test_load_json_file() {
        let file_path = String::from("./tests/testload.json");
        assert!(Path::new(&file_path).exists());

        let data: String = String::from(
            r#"{
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
            }"#,
        );

        let mut file: fs::File = fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&file_path)
            .expect("Unable to open file");
        assert!(file.write(data.as_bytes()).is_ok());

        let mut wallet: Wallet = Wallet::new();
        assert!(wallet.empty());
        //do error checking here instead of boolean checks.
        assert!(wallet.load(&file_path).unwrap());
        assert_eq!(wallet.size(), 2);

        let web: String = String::from("Websites");
        assert!(wallet.get_category(&web).is_ok());
        assert_eq!(wallet.get_category(&web).unwrap().size(), 3);

        let google: String = String::from("Google");
        assert!(wallet.get_category(&web).unwrap().get_item(&google).is_ok());
        assert_eq!(
            wallet
                .get_category(&web)
                .unwrap()
                .get_item(&google)
                .unwrap()
                .size(),
            3
        );
        assert_eq!(
            wallet
                .get_category(&web)
                .unwrap()
                .get_item(&google)
                .unwrap()
                .get_entry(&String::from("url"))
                .unwrap(),
            &String::from("https://www.google.com/")
        );
        assert_eq!(
            wallet
                .get_category(&web)
                .unwrap()
                .get_item(&google)
                .unwrap()
                .get_entry(&String::from("username"))
                .unwrap(),
            &String::from("example@gmail.com")
        );
        assert_eq!(
            wallet
                .get_category(&web)
                .unwrap()
                .get_item(&google)
                .unwrap()
                .get_entry(&String::from("password"))
                .unwrap(),
            &String::from("pass1234")
        );

        let facebook: String = String::from("Facebook");
        assert!(wallet
            .get_category(&web)
            .unwrap()
            .get_item(&facebook)
            .is_ok());
        assert_eq!(
            wallet
                .get_category(&web)
                .unwrap()
                .get_item(&facebook)
                .unwrap()
                .size(),
            3
        );
        assert_eq!(
            wallet
                .get_category(&web)
                .unwrap()
                .get_item(&facebook)
                .unwrap()
                .get_entry(&String::from("url"))
                .unwrap(),
            &String::from("https://www.facebook.com/")
        );
        assert_eq!(
            wallet
                .get_category(&web)
                .unwrap()
                .get_item(&facebook)
                .unwrap()
                .get_entry(&String::from("username"))
                .unwrap(),
            &String::from("example@gmail.com")
        );
        assert_eq!(
            wallet
                .get_category(&web)
                .unwrap()
                .get_item(&facebook)
                .unwrap()
                .get_entry(&String::from("password"))
                .unwrap(),
            &String::from("pass1234fb")
        );

        let twitter: String = String::from("Twitter");
        assert!(wallet
            .get_category(&web)
            .unwrap()
            .get_item(&twitter)
            .is_ok());
        assert_eq!(
            wallet
                .get_category(&web)
                .unwrap()
                .get_item(&twitter)
                .unwrap()
                .size(),
            3
        );
        assert_eq!(
            wallet
                .get_category(&web)
                .unwrap()
                .get_item(&twitter)
                .unwrap()
                .get_entry(&String::from("url"))
                .unwrap(),
            &String::from("https://www.twitter.com/")
        );
        assert_eq!(
            wallet
                .get_category(&web)
                .unwrap()
                .get_item(&twitter)
                .unwrap()
                .get_entry(&String::from("username"))
                .unwrap(),
            &String::from("example@gmail.com")
        );
        assert_eq!(
            wallet
                .get_category(&web)
                .unwrap()
                .get_item(&twitter)
                .unwrap()
                .get_entry(&String::from("password"))
                .unwrap(),
            &String::from("r43rfsffdsfdsf")
        );

        let bank: String = String::from("Bank Accounts");
        assert!(wallet.get_category(&bank).is_ok());
        assert_eq!(wallet.get_category(&bank).unwrap().size(), 1);

        let starling: String = String::from("Starling");
        assert!(wallet
            .get_category(&bank)
            .unwrap()
            .get_item(&starling)
            .is_ok());
        assert_eq!(
            wallet
                .get_category(&bank)
                .unwrap()
                .get_item(&starling)
                .unwrap()
                .size(),
            3
        );
        assert_eq!(
            wallet
                .get_category(&bank)
                .unwrap()
                .get_item(&starling)
                .unwrap()
                .get_entry(&String::from("Name"))
                .unwrap(),
            &String::from("Mr John Doe")
        );
        assert_eq!(
            wallet
                .get_category(&bank)
                .unwrap()
                .get_item(&starling)
                .unwrap()
                .get_entry(&String::from("Account Number"))
                .unwrap(),
            &String::from("12345678")
        );
        assert_eq!(
            wallet
                .get_category(&bank)
                .unwrap()
                .get_item(&starling)
                .unwrap()
                .get_entry(&String::from("Sort Code"))
                .unwrap(),
            &String::from("12-34-56")
        );
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
        item_1
            .add_entry(&entry_key_1, &entry_value_1)
            .expect("error adding entry");
        item_1
            .add_entry(&entry_key_2, &entry_value_2)
            .expect("error adding entry");
        item_2
            .add_entry(&entry_key_1, &entry_value_1)
            .expect("error adding entry");

        assert_eq!(item_1.size(), 2);
        assert_eq!(item_2.size(), 1);

        let mut cat_1: Category = Category::new(ident_1.clone());
        let mut cat_2: Category = Category::new(ident_2.clone());
        cat_1.add_item(&item_1).expect("error adding item");
        cat_1.add_item(&item_2).expect("error adding item");
        cat_2.add_item(&item_1).expect("error adding item");

        assert_eq!(cat_1.size(), 2);
        assert_eq!(cat_2.size(), 1);

        wallet.add_category(cat_1).expect("error adding category");
        wallet.add_category(cat_2).expect("error adding category");

        assert_eq!(wallet.size(), 2);
        assert!(wallet.save(&file_path).is_ok());
        let file_contents: String = fs::read_to_string(&file_path).expect("Unable to read file");
        let expected_contents: &str = r#"{"ident_1":{"ident_1":{"key_1":"value_1","key_2":"value_2"},"ident_2":{"key_1":"value_1"}},"ident_2":{"ident_1":{"key_1":"value_1","key_2":"value_2"}}}"#;
        assert_eq!(file_contents, expected_contents);
    }
}
