pub mod app {
    use crate::{category::Category, item::Item, wallet::Wallet};
    use clap::{arg, Parser};
    use std::io::{Error, ErrorKind};

    const STUDENT_NUMBER: &str = "851784";

    #[derive(Debug, PartialEq)]
    pub(crate) enum Action {
        Create,
        Read,
        Update,
        Delete,
    }

    #[derive(Parser, Default, Debug, Clone)]
    #[command(author = "Callum Adair",
    version,
    about = "A remake of the CSC371 Module Assignment from the Department of Computer Science at Swansea University written in Rust",
    long_about = None)]
    pub struct Args {
        /// Path of the database file
        #[arg(short, long)]
        pub(crate) database: String,

        /// Action to be performed
        #[arg(short, long)]
        pub(crate) action: Option<String>,

        /// Name of the category if present
        #[arg(short, long)]
        pub(crate) category: Option<String>,

        /// Name of the item if present
        #[arg(short, long)]
        pub(crate) item: Option<String>,

        /// Name of the entry if present
        #[arg(short, long)]
        pub(crate) entry: Option<String>,
    }

    pub fn run(args: &Args) -> Result<(), Error> {
        let db_filename = args.database.clone();
        let mut w_obj: Wallet = Wallet::new();
        w_obj.load(&db_filename);

        let action: Action = parse_action_argument(args).unwrap();

        match action {
            Action::Read => execute_read_action(args, &mut w_obj),
            Action::Create => execute_create_action(args, &mut w_obj),
            Action::Update => execute_update_action(args, &mut w_obj),
            Action::Delete => execute_delete_action(args, &mut w_obj),
        }
    }

    pub(crate) fn parse_action_argument(args: &Args) -> Result<Action, Error> {
        let action: String = args.action.clone().unwrap().to_uppercase();

        match Some(action.as_str()) {
            None => Err(Error::new(
                ErrorKind::InvalidInput,
                "No action argument provided.",
            )),
            Some("CREATE") => Ok(Action::Create),
            Some("READ") => Ok(Action::Read),
            Some("UPDATE") => Ok(Action::Update),
            Some("DELETE") => Ok(Action::Delete),
            Some(_) => Err(Error::new(
                ErrorKind::InvalidInput,
                "Invalid action argument.",
            )),
        }
    }

    fn execute_create_action(args: &Args, w_obj: &mut Wallet) -> Result<(), Error> {
        let args = args.clone();

        if args.category.is_none() && (args.item.is_some() || args.entry.is_some()) {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "No category argument provided.",
            ));
        } else if args.category.is_none() {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "Error: missing category, item or entry argument(s).",
            ));
        }

        let new_category: &mut Category = w_obj.new_category(&args.category.unwrap());

        if args.item.is_none() && args.entry.is_some() {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "No item argument provided.",
            ));
        } else if args.item.is_none() {
            return Ok(());
        }

        let new_item: &mut Item = new_category.new_item(&args.item.unwrap());

        if args.entry.is_none() {
            return Ok(());
        }

        let entry_input: String = args.entry.unwrap();
        let entry_delimiter: String = String::from(",");

        if entry_input.contains(&entry_delimiter) {
            let entry_identifier: String =
                entry_input.split(&entry_delimiter).collect::<Vec<&str>>()[0].to_string();
            let entry_value: String =
                entry_input.split(&entry_delimiter).collect::<Vec<&str>>()[1].to_string();

            new_item.add_entry(&entry_identifier, &entry_value);
        } else {
            new_item.add_entry(&entry_input, &String::from(""));
        }
        w_obj.save(&args.database);
        Ok(())
    }

    pub(crate) fn execute_read_action(args: &Args, w_obj: &mut Wallet) -> Result<(), Error> {
        let result = generate_wallet_string(args, w_obj).unwrap();
        println!("{:?}", result);
        Ok(())
    }

    pub(crate) fn generate_wallet_string(args: &Args, w_obj: &mut Wallet) -> Result<String, Error> {
        let args = args.clone();

        if args.category.is_none() && (args.item.is_some() || args.entry.is_some()) {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "No category argument provided.",
            ));
        }

        if args.category.is_none() {
            return Ok(get_wallet_json(w_obj));
        }

        if args.item.is_none() && args.entry.is_some() {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "No item argument provided.",
            ));
        } else if args.item.is_none() {
            return Ok(get_category_json(w_obj, &args.category.unwrap()));
        }

        if args.entry.is_none() {
            return Ok(get_item_json(
                w_obj,
                &args.category.unwrap(),
                &args.item.unwrap(),
            ));
        }

        Ok(get_entry_json(
            w_obj,
            &args.category.unwrap(),
            &args.item.unwrap(),
            &args.entry.unwrap(),
        ))
    }

    fn execute_update_action(args: &Args, w_obj: &mut Wallet) -> Result<(), Error> {
        if args.category.is_none() && args.item.is_none() && args.entry.is_none() {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "No category, item or entry argument provided.",
            ));
        }

        if args.category.is_none() && (args.item.is_some() || args.entry.is_some()) {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "Error: missing category argument(s).",
            ));
        }

        let key_delimiter: char = ':';
        let cat_input: String = args.clone().category.unwrap();

        let cur_cat_ident: String = if cat_input.contains(",") {
            cat_input.split(key_delimiter).collect::<Vec<&str>>()[0].to_string()
        } else {
            cat_input.clone()
        };

        if args.item.is_none() && args.entry.is_some() {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "No item argument provided.",
            ));
        }

        let cur_cat: &mut Category = w_obj.get_category(&cur_cat_ident).unwrap();
        let item_input: String = args.clone().item.unwrap();
        let cur_item_ident: String = if item_input.contains(key_delimiter) {
            item_input.split(key_delimiter).collect::<Vec<&str>>()[0].to_string()
        } else {
            item_input.clone()
        };

        if args.entry.is_some() {
            process_entry_update(&args, key_delimiter, cur_cat, &cur_item_ident)
                .expect("TODO: panic message");
        }

        if args.item.is_some() {
            process_item_update(key_delimiter, cur_cat, &item_input, &cur_item_ident)
                .expect("TODO: panic message");
        }

        process_category_update(w_obj, key_delimiter, &cat_input, &cur_cat_ident)
            .expect("TODO: panic message");

        Ok(())
    }

    fn process_category_update(
        w_obj: &mut Wallet,
        key_delimiter: char,
        cat_input: &String,
        cur_cat_ident: &String,
    ) -> Result<(), Error> {
        if cat_input.contains(key_delimiter) {
            let new_cat_ident: String =
                cat_input.split(key_delimiter).collect::<Vec<&str>>()[1].to_string();

            let cur_cat: &mut Category = w_obj.get_category(&cur_cat_ident).unwrap();

            if new_cat_ident.is_empty() {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    "Error: new category identifier cannot be empty.",
                ));
            }

            cur_cat.set_ident(&new_cat_ident);
            let new_cat = cur_cat.clone();
            w_obj.add_category(new_cat);
            w_obj.delete_category(&cur_cat_ident);
        }
        Ok(())
    }

    fn process_item_update(
        key_delimiter: char,
        cur_cat: &mut Category,
        item_input: &String,
        cur_item_ident: &String,
    ) -> Result<(), Error> {
        if item_input.contains(key_delimiter) {
            let new_item_ident: String =
                item_input.split(key_delimiter).collect::<Vec<&str>>()[1].to_string();
            let cur_item: &mut Item = cur_cat.get_item(&cur_item_ident).unwrap();

            if new_item_ident.is_empty() {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    "Error: new item identifier cannot be empty.",
                ));
            }
            cur_item.set_ident(&new_item_ident);
            let new_item = cur_item.clone();
            cur_cat.add_item(&new_item);
            cur_cat.delete_item(&cur_item_ident);
        }
        Ok(())
    }

    fn process_entry_update(
        args: &Args,
        key_delimiter: char,
        cur_cat: &mut Category,
        cur_item_ident: &String,
    ) -> Result<(), Error> {
        let cur_item: &mut Item = cur_cat.get_item(&cur_item_ident).unwrap();
        let entry_input: String = args.clone().entry.unwrap();
        let value_delimiter: char = ',';

        let input_vec: Vec<&str> = entry_input
            .split([key_delimiter, value_delimiter])
            .collect::<Vec<&str>>();

        let entry_ident: String = input_vec[0].to_string();

        // if !entry_input.contains([key_delimiter, value_delimiter]) {}

        if entry_input.contains(key_delimiter) && entry_input.contains(value_delimiter) {
            let new_entry_ident = input_vec[1].to_string();
            let new_entry_val = input_vec[2].to_string();

            cur_item.add_entry(&new_entry_ident, &new_entry_val);
            cur_item.delete_entry(&entry_ident);
        } else if entry_input.contains(key_delimiter) {
            let new_entry_ident: String = input_vec[1].to_string();

            if new_entry_ident.is_empty() {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    "No replacement entry argument provided",
                ));
            }

            let new_entry_val = cur_item.get_entry(&entry_ident).unwrap().clone();
            cur_item.add_entry(&new_entry_ident, &new_entry_val);
            cur_item.delete_entry(&entry_ident);
        } else if entry_input.contains(value_delimiter) {
            let new_entry_val = input_vec[1].to_string();
            cur_item.delete_entry(&entry_ident);
            cur_item.add_entry(&entry_ident, &new_entry_val);
        }

        Ok(())
    }

    fn execute_delete_action(args: &Args, w_obj: &mut Wallet) -> Result<(), Error> {
        if args.category.is_none() {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "No category argument provided.",
            ));
        }

        let cat_str = args.category.clone().unwrap();

        if args.item.is_none() {
            if args.entry.is_some() {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    "No item argument provided.",
                ));
            }

            w_obj.delete_category(&cat_str);
            return Ok(());
        }

        let item_str = args.item.clone().unwrap();

        if args.entry.is_none() {
            w_obj.get_category(&cat_str).unwrap().delete_item(&item_str);
            return Ok(());
        }

        w_obj
            .get_category(&cat_str)
            .unwrap()
            .get_item(&item_str)
            .unwrap()
            .delete_entry(&args.entry.clone().unwrap());

        Ok(())
    }

    fn get_wallet_json(w: &Wallet) -> String {
        serde_json::to_string(w).unwrap()
    }

    fn get_category_json(w: &mut Wallet, c: &String) -> String {
        serde_json::to_string(w.get_category(c).unwrap()).unwrap()
    }

    fn get_item_json(w: &mut Wallet, c: &String, i: &String) -> String {
        serde_json::to_string(w.get_category(c).unwrap().get_item(i).unwrap()).unwrap()
    }

    fn get_entry_json(w: &mut Wallet, c: &String, i: &String, e: &String) -> String {
        serde_json::to_string(
            w.get_category(c)
                .unwrap()
                .get_item(i)
                .unwrap()
                .get_entry(e)
                .unwrap(),
        )
        .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::{_371pass::app, wallet::Wallet};
    use std::{
        fs,
        io::Write,
        io::{Error, ErrorKind},
        path::Path,
    };

    #[test]
    fn test_args_parsing() {
        let mut args = app::Args {
            database: String::from("test"),
            action: Some(String::from("invalid")),
            category: None,
            item: None,
            entry: None,
        };

        let expected_error = Error::new(ErrorKind::InvalidInput, "Invalid action argument.");
        let result = app::parse_action_argument(&args);

        assert!(result.is_err());
        assert_eq!(result.as_ref().unwrap_err().kind(), expected_error.kind());
        assert_eq!(
            result.as_ref().unwrap_err().to_string(),
            expected_error.to_string()
        );

        args.action = Some(String::from("create"));
        let result = app::parse_action_argument(&args);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), app::Action::Create);

        args.action = Some(String::from("read"));
        let result = app::parse_action_argument(&args);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), app::Action::Read);

        args.action = Some(String::from("update"));
        let result = app::parse_action_argument(&args);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), app::Action::Update);

        args.action = Some(String::from("delete"));
        let result = app::parse_action_argument(&args);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), app::Action::Delete);
    }

    #[test]
    fn test_create_action() {
        let file_path: String = String::from("./tests/testcreate.json");
        assert!(Path::new(&file_path).exists());
        let data = String::from(
            r#"{"Bank Accounts":{"Starling":{"Account Number":"12345678","Name":"Mr John Doe","Sort Code":"12-34-56"}},"Websites":{"Facebook":{"password":"pass1234fb","url":"https://www.facebook.com/","username":"example@gmail.com"},"Google":{"password":"pass1234","url":"https://www.google.com/","username":"example@gmail.com"},"Twitter":{"password":"r43rfsffdsfdsf","url":"https://www.twitter.com/","username":"example@gmail.com"}}}"#,
        );
        let mut file: fs::File = fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&file_path)
            .expect("Unable to open file");
        assert!(file.write(data.as_bytes()).is_ok());

        let test_category_ident: String = String::from("Test Category");
        let test_item_ident: String = String::from("Test Item");
        let test_entry_key: String = String::from("Test Entry Key");
        let test_entry_value: String = String::from("Test Entry Value");
        let test_entry_arg: String = format!("{},{}", test_entry_key, test_entry_value);

        let args = app::Args {
            database: file_path.clone(),
            action: Some(String::from("create")),
            category: Some(test_category_ident.clone()),
            item: Some(test_item_ident.clone()),
            entry: None,
        };

        assert!(app::run(&args).is_ok());
        let mut w_obj1 = Wallet::new();
        assert!(w_obj1.empty());
        assert!(w_obj1.load(&file_path));

        assert!(w_obj1.get_category(&test_category_ident).is_some());
        assert_eq!(w_obj1.get_category(&test_category_ident).unwrap().size(), 1);
        assert!(w_obj1
            .get_category(&test_category_ident)
            .unwrap()
            .get_item(&test_item_ident)
            .is_some());
        assert_eq!(
            w_obj1
                .get_category(&test_category_ident)
                .unwrap()
                .get_item(&test_item_ident)
                .unwrap()
                .size(),
            0
        );

        let args = app::Args {
            database: file_path.clone(),
            action: Some(String::from("create")),
            category: Some(test_category_ident.clone()),
            item: Some(test_item_ident.clone()),
            entry: Some(test_entry_arg.clone()),
        };

        assert!(app::run(&args).is_ok());
        let mut w_obj3 = Wallet::new();
        assert!(w_obj3.empty());
        assert!(w_obj3.load(&file_path));

        assert!(w_obj3.get_category(&test_category_ident).is_some());
        assert_eq!(w_obj3.get_category(&test_category_ident).unwrap().size(), 1);
        assert!(w_obj3
            .get_category(&test_category_ident)
            .unwrap()
            .get_item(&test_item_ident)
            .is_some());
        assert_eq!(
            w_obj3
                .get_category(&test_category_ident)
                .unwrap()
                .get_item(&test_item_ident)
                .unwrap()
                .size(),
            1
        );
        assert_eq!(
            w_obj3
                .get_category(&test_category_ident)
                .unwrap()
                .get_item(&test_item_ident)
                .unwrap()
                .get_entry(&test_entry_key)
                .unwrap(),
            &test_entry_value
        );
    }

    #[test]
    fn test_read_action() {
        let file_path: String = String::from("./tests/testdatabase.json");
        assert!(Path::new(&file_path).exists());
        let data = String::from(
            r#"{"Bank Accounts":{"Starling":{"Account Number":"12345678","Name":"Mr John Doe","Sort Code":"12-34-56"}},"Websites":{"Facebook":{"password":"pass1234fb","url":"https://www.facebook.com/","username":"example@gmail.com"},"Google":{"password":"pass1234","url":"https://www.google.com/","username":"example@gmail.com"},"Twitter":{"password":"r43rfsffdsfdsf","url":"https://www.twitter.com/","username":"example@gmail.com"}}}"#,
        );
        let mut file: fs::File = fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&file_path)
            .expect("Unable to open file");
        assert!(file.write(data.as_bytes()).is_ok());

        let args = app::Args {
            database: file_path.clone(),
            action: Some(String::from("read")),
            category: None,
            item: None,
            entry: None,
        };

        assert!(app::run(&args).is_ok());
        let mut wallet = Wallet::new();
        wallet.load(&file_path);
        assert_eq!(
            data,
            app::generate_wallet_string(&args, &mut wallet).unwrap()
        );
    }

    #[test]
    fn test_delete_action() {
        let file_path: String = String::from("./tests/testdelete.json");
        assert!(Path::new(&file_path).exists());
        let data = String::from(
            r#"{"Bank Accounts":{"Starling":{"Account Number":"12345678","Name":"Mr John Doe","Sort Code":"12-34-56"}},"Websites":{"Facebook":{"password":"pass1234fb","url":"https://www.facebook.com/","username":"example@gmail.com"},"Google":{"password":"pass1234","url":"https://www.google.com/","username":"example@gmail.com"},"Twitter":{"password":"r43rfsffdsfdsf","url":"https://www.twitter.com/","username":"example@gmail.com"}}}"#,
        );
        let mut file: fs::File = fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&file_path)
            .expect("Unable to open file");
        assert!(file.write(data.as_bytes()).is_ok());

        let test_category: String = String::from("Bank Accounts");
        let test_item: String = String::from("Starling");
        let test_entry_key: String = String::from("Account Number");

        let mut args = app::Args {
            database: file_path.clone(),
            action: Some(String::from("delete")),
            category: Some(test_category.clone()),
            item: Some(test_item.clone()),
            entry: Some(test_entry_key.clone()),
        };

        assert!(app::run(&args).is_ok());
        let mut w_obj = Wallet::new();
        assert!(w_obj.empty());
        assert!(w_obj.load(&file_path));
        assert!(w_obj
            .get_category(&test_category)
            .unwrap()
            .get_item(&test_item)
            .unwrap()
            .get_entry(&test_entry_key)
            .is_none());

        args.entry = None;
        assert!(app::run(&args).is_ok());
        let mut w_obj: Wallet = Wallet::new();
        assert!(w_obj.empty());
        assert!(w_obj.load(&file_path));
        assert!(w_obj
            .get_category(&test_category)
            .unwrap()
            .get_item(&test_item)
            .is_none());

        args.item = None;
        assert!(app::run(&args).is_ok());
        let mut w_obj: Wallet = Wallet::new();
        assert!(w_obj.empty());
        assert!(w_obj.load(&file_path));
        assert!(w_obj.get_category(&test_category).is_none());
    }

    #[test]
    fn test_update_action() {
        let file_path: String = String::from("./tests/testupdate.json");
        assert!(Path::new(&file_path).exists());
        let data = String::from(
            r#"{"Bank Accounts":{"Starling":{"Account Number":"12345678","Name":"Mr John Doe","Sort Code":"12-34-56"}},"Websites":{"Facebook":{"password":"pass1234fb","url":"https://www.facebook.com/","username":"example@gmail.com"},"Google":{"password":"pass1234","url":"https://www.google.com/","username":"example@gmail.com"},"Twitter":{"password":"r43rfsffdsfdsf","url":"https://www.twitter.com/","username":"example@gmail.com"}}}"#,
        );
        let mut file: fs::File = fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&file_path)
            .expect("Unable to open file");
        assert!(file.write(data.as_bytes()).is_ok());

        let old_test_category: String = String::from("Bank Accounts");
        let old_test_item: String = String::from("Starling");
        let old_test_entry_key: String = String::from("Account Number");
        let old_test_entry_value: String = String::from("12345678");

        let new_test_category: String = String::from("Current Accounts");
        let new_test_item: String = String::from("Santander");
        let new_test_entry_key: String = String::from("Account");
        let new_test_entry_value: String = String::from("87654321");

        let mut w_obj: Wallet = Wallet::new();
        assert!(w_obj.load(&file_path));

        assert!(w_obj
            .get_category(&old_test_category)
            .unwrap()
            .get_item(&old_test_item)
            .unwrap()
            .get_entry(&old_test_entry_key)
            .is_some());
        assert_eq!(
            w_obj
                .get_category(&old_test_category)
                .unwrap()
                .get_item(&old_test_item)
                .unwrap()
                .get_entry(&old_test_entry_key)
                .unwrap(),
            &old_test_entry_value
        );
        assert!(w_obj
            .get_category(&new_test_category)
            .unwrap()
            .get_item(&new_test_item)
            .unwrap()
            .get_entry(&new_test_entry_key)
            .is_none());

        let args = app::Args {
            database: String::from("./tests/testdelete.json"),
            action: Some(String::from("update")),
            category: Some(format!("{}:{}", old_test_category, new_test_category)),
            item: Some(format!("{}:{}", old_test_item, new_test_item)),
            entry: Some(format!(
                "{}:{},{}",
                old_test_entry_key, new_test_entry_key, new_test_entry_value
            )),
        };

        assert!(app::run(&args).is_ok());
        let mut w_obj: Wallet = Wallet::new();
        assert!(w_obj.empty());
        assert!(w_obj.load(&file_path));
        assert!(w_obj
            .get_category(&old_test_category)
            .unwrap()
            .get_item(&old_test_item)
            .unwrap()
            .get_entry(&old_test_entry_key)
            .is_none());
        assert!(w_obj
            .get_category(&new_test_category)
            .unwrap()
            .get_item(&new_test_item)
            .unwrap()
            .get_entry(&new_test_entry_key)
            .is_some());
        assert_eq!(
            w_obj
                .get_category(&new_test_category)
                .unwrap()
                .get_item(&new_test_item)
                .unwrap()
                .get_entry(&new_test_entry_key)
                .unwrap(),
            &new_test_entry_value
        );
    }
}
