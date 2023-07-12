pub mod app {
    use std::io::{Error, ErrorKind};
    use clap::{arg, Parser};
    use crate::{wallet::Wallet, category::Category, item::Item};

    const STUDENT_NUMBER: &str = "851784";

    #[derive(Debug, PartialEq)]
    pub(crate) enum Action {
        CREATE,
        READ,
        UPDATE,
        DELETE,
    }

    #[derive(Parser, Default, Debug, Clone)]
    #[command(author = "Callum Adair",
    version,
    about = "A remake of the Department of Computer Science at Swansea University's CSC371 Module Assignment written in Rust",
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

    pub fn run(args: Args) -> Result<(), Error> {
        let db_filename = args.database.clone();
        let mut w_obj: Wallet = Wallet::new();
        w_obj.load(&db_filename);

        let action: Action = parse_action_argument(args.clone()).unwrap();

        match action {
            Action::READ =>
                execute_read_action(args, &mut w_obj),
            Action::CREATE =>
                execute_create_action(args, &mut w_obj),
            Action::UPDATE =>
                execute_update_action(args, &mut w_obj),
            Action::DELETE =>
                execute_delete_action(args, &mut w_obj),
        }
    }

    pub(crate) fn parse_action_argument(args: Args) -> Result<Action, Error> {
        let action: String = args.action.unwrap().to_uppercase();

        return match Some(action.as_str()) {
            None =>
                Err(Error::new(ErrorKind::InvalidInput, "No action argument provided.")),
            Some("CREATE") =>
                Ok(Action::CREATE),
            Some("READ") =>
                Ok(Action::READ),
            Some("UPDATE") =>
                Ok(Action::UPDATE),
            Some("DELETE") =>
                Ok(Action::DELETE),
            Some(_) =>
                Err(Error::new(ErrorKind::InvalidInput, "Invalid action argument.")),
        };
    }

    fn execute_create_action(args: Args, w_obj: &mut Wallet) -> Result<(), Error> {
        if args.category.is_none() && (args.item.is_some() || args.entry.is_some()) {
            return Err(Error::new(ErrorKind::InvalidInput, "No category argument provided."));
        } else if args.category.is_none() {
            return Err(Error::new(ErrorKind::InvalidInput, "Error: missing category, item or entry argument(s)."));
        }

        let new_category: &mut Category = w_obj.new_category(&args.category.unwrap());

        if args.item.is_none() && args.entry.is_some() {
            return Err(Error::new(ErrorKind::InvalidInput, "No item argument provided."));
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
            let entry_identifier: String = entry_input
                .split(&entry_delimiter)
                .collect::<Vec<&str>>()[0]
                .to_string();
            let entry_value: String = entry_input
                .split(&entry_delimiter)
                .collect::<Vec<&str>>()[1]
                .to_string();

            new_item.add_entry(entry_identifier, entry_value);
        } else {
            new_item.add_entry(entry_input, String::from(""));
        }
        w_obj.save(&args.database);
        Ok(())
    }

    pub(crate) fn execute_read_action(args: Args, w_obj: &mut Wallet) -> Result<(), Error> {
        let result = generate_wallet_string(args, w_obj).unwrap();
        println!("{:?}", result);
        Ok(())
    }

    pub(crate) fn generate_wallet_string(args: Args, w_obj: &mut Wallet) -> Result<String, Error> {
        if args.category.is_none() && (args.item.is_some() || args.entry.is_some()) {
            return Err(Error::new(ErrorKind::InvalidInput, "No category argument provided."));
        }

        if args.category.is_none() {
            return Ok(get_wallet_json(w_obj));
        }

        if args.item.is_none() && args.entry.is_some() {
            return Err(Error::new(ErrorKind::InvalidInput, "No item argument provided."));
        } else if args.item.is_none() {
            return Ok(get_category_json(w_obj,
                                        &args.category.unwrap()));
        }

        if args.entry.is_none() {
            return Ok(get_item_json(w_obj,
                                    &args.category.unwrap(),
                                    &args.item.unwrap()));
        }

        return Ok(get_entry_json(w_obj,
                                 &args.category.unwrap(),
                                 &args.item.unwrap(),
                                 &args.entry.unwrap()));
    }


    fn execute_update_action(args: Args, w_obj: &mut Wallet) -> Result<(), Error> {
        if args.category.is_none() && args.item.is_none() && args.entry.is_none() {
            return Err(Error::new(ErrorKind::InvalidInput, "No category, item or entry argument provided."));
        }

        if args.category.is_none() && (args.item.is_some() || args.entry.is_some()) {
            return Err(Error::new(ErrorKind::InvalidInput, "Error: missing category argument(s)."));
        }

        let key_delimiter: String = String::from(":");
        let cat_input: String = args.clone().category.unwrap();

        let cur_cat_ident: String = if cat_input.contains(",") {
            cat_input
                .split(&key_delimiter)
                .collect::<Vec<&str>>()[0]
                .to_string()
        } else { cat_input.clone() };

        if args.item.is_none() && args.entry.is_some() {
            return Err(Error::new(ErrorKind::InvalidInput, "No item argument provided."));
        }

        let cur_cat: &mut Category = w_obj.get_category(&cur_cat_ident).unwrap();
        let item_input: String = args.clone().item.unwrap();
        let cur_item_ident: String = if item_input.contains(&key_delimiter) {
            item_input
                .split(&key_delimiter)
                .collect::<Vec<&str>>()[0]
                .to_string()
        } else { item_input.clone() };

        if args.entry.is_some() { process_entry_update(&args, &key_delimiter, cur_cat, &cur_item_ident).expect("TODO: panic message"); }

        if args.item.is_some() { process_item_update(&key_delimiter, cur_cat, &item_input, &cur_item_ident).expect("TODO: panic message"); }

        process_category_update(w_obj, &key_delimiter, &cat_input, &cur_cat_ident).expect("TODO: panic message");

        Ok(())
    }

    fn process_category_update(w_obj: &mut Wallet,
                               key_delimiter: &String,
                               cat_input: &String,
                               cur_cat_ident: &String) -> Result<(), Error> {
        if cat_input.contains(key_delimiter) {
            let new_cat_ident: String = cat_input
                .split(key_delimiter)
                .collect::<Vec<&str>>()[1]
                .to_string();

            let cur_cat: &mut Category = w_obj.get_category(&cur_cat_ident).unwrap();

            if new_cat_ident.is_empty() {
                return Err(Error::new(ErrorKind::InvalidInput, "Error: new category identifier cannot be empty."));
            }

            cur_cat.set_ident(new_cat_ident);
            let new_cat = cur_cat.clone();
            w_obj.add_category(new_cat);
            w_obj.delete_category(&cur_cat_ident);
        }
        Ok(())
    }

    fn process_item_update(key_delimiter: &String,
                           cur_cat: &mut Category,
                           item_input: &String,
                           cur_item_ident: &String) -> Result<(), Error> {
        if item_input.contains(key_delimiter) {
            let new_item_ident: String = item_input
                .split(key_delimiter)
                .collect::<Vec<&str>>()[1]
                .to_string();
            let cur_item: &mut Item = cur_cat.get_item(&cur_item_ident).unwrap();

            if new_item_ident.is_empty() {
                return Err(Error::new(ErrorKind::InvalidInput, "Error: new item identifier cannot be empty."));
            }
            cur_item.set_ident(new_item_ident);
            let new_item = cur_item.clone();
            cur_cat.add_item(new_item);
            cur_cat.delete_item(&cur_item_ident);
        }
        Ok(())
    }

    fn process_entry_update(args: &Args,
                            key_delimiter: &String,
                            cur_cat: &mut Category,
                            cur_item_ident: &String) -> Result<(), Error> {
        let cur_item: &mut Item = cur_cat.get_item(&cur_item_ident).unwrap();
        let entry_input: String = args.clone().entry.unwrap();
        let value_delimiter: String = String::from(",");

        if entry_input.contains(key_delimiter)
            && entry_input.contains(&value_delimiter) {
            let input_vec = entry_input
                .split(key_delimiter)
                .collect::<Vec<&str>>();

            let old_entry_ident: String = input_vec[0].to_string();
            let update_vals: String = input_vec[1].to_string();

            let update_vec = update_vals
                .split(&value_delimiter)
                .collect::<Vec<&str>>();
        }

        Ok(())
    }

    fn execute_delete_action(args: Args, w_obj: &mut Wallet) -> Result<(), Error> {
        if args.category.is_none() {
            return Err(Error::new(ErrorKind::InvalidInput, "No category argument provided."));
        }

        let cat_str = args.category.unwrap();

        if args.item.is_none() && args.entry.is_some() {
            return Err(Error::new(ErrorKind::InvalidInput, "No item argument provided."));
        }

        if args.item.is_none() {
            w_obj.delete_category(&cat_str);
            return Ok(());
        }

        let item_str = args.item.unwrap();

        if args.entry.is_none() {
            w_obj.get_category(&cat_str).unwrap()
                .delete_item(&item_str);
            return Ok(());
        }

        w_obj.get_category(&cat_str).unwrap()
            .get_item(&item_str).unwrap()
            .delete_entry(args.entry.unwrap());

        Ok(())
    }

    fn get_wallet_json(w: &Wallet) -> String {
        String::from(serde_json::to_string(w).unwrap())
    }

    fn get_category_json(w: &mut Wallet, c: &String) -> String {
        String::from(
            serde_json::to_string(
                w.get_category(c)
                    .unwrap()).unwrap())
    }

    fn get_item_json(w: &mut Wallet, c: &String, i: &String) -> String {
        String::from(serde_json::to_string(w.get_category(c).unwrap()
            .get_item(i).unwrap()).unwrap())
    }

    fn get_entry_json(w: &Wallet, c: &String, i: &String, e: &String) -> String {
        return String::from("");
    }
}

#[cfg(test)]
mod tests {
    use std::{
        io::{Error, ErrorKind},
        path::Path,
        fs,
        io::Write,
    };
    use crate::{
        _371pass::{app},
        wallet::Wallet,
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
        let result = app::parse_action_argument(args.clone());

        assert!(result.is_err());
        assert_eq!(result.as_ref()
                       .unwrap_err()
                       .kind(),
                   expected_error.kind());
        assert_eq!(result.as_ref()
                       .unwrap_err()
                       .to_string(),
                   expected_error.to_string());

        args.action = Some(String::from("create"));
        let result = app::parse_action_argument(args.clone());
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), app::Action::CREATE);

        args.action = Some(String::from("read"));
        let result = app::parse_action_argument(args.clone());
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), app::Action::READ);

        args.action = Some(String::from("update"));
        let result = app::parse_action_argument(args.clone());
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), app::Action::UPDATE);

        args.action = Some(String::from("delete"));
        let result = app::parse_action_argument(args.clone());
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), app::Action::DELETE);
    }


    #[test]
    fn test_create_action() {
        let file_path: String = String::from("./tests/testcreate.json");
        assert!(Path::new(&file_path).exists());
        let data = String::from(r#"{"Bank Accounts":{"Starling":{"Account Number":"12345678","Name":"Mr John Doe","Sort Code":"12-34-56"}},"Websites":{"Facebook":{"password":"pass1234fb","url":"https://www.facebook.com/","username":"example@gmail.com"},"Google":{"password":"pass1234","url":"https://www.google.com/","username":"example@gmail.com"},"Twitter":{"password":"r43rfsffdsfdsf","url":"https://www.twitter.com/","username":"example@gmail.com"}}}"#);
        let mut file: fs::File = fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&file_path)
            .expect("Unable to open file");
        assert!(file.write(data.as_bytes()).is_ok());

        let test_category: String = String::from("Test Category");
        let test_item: String = String::from("Test Item");
        let test_entry_key: String = String::from("Test Entry Key");
        let test_entry_value: String = String::from("Test Entry Value");
        let test_entry: String = format!("{},{}", test_entry_key, test_entry_value);

        let mut w_obj = Wallet::new();
        assert!(w_obj.empty());
        assert!(w_obj.load(&file_path));

        let args = app::Args {
            database: file_path.clone(),
            action: Some(String::from("create")),
            category: Some(test_category.clone()),
            item: Some(test_item.clone()),
            entry: None,
        };

        assert!(app::run(args).is_ok());
        let mut w_obj2 = Wallet::new();
        assert!(w_obj2.empty());
        assert!(w_obj2.load(&file_path));

        assert!(w_obj2.get_category(&test_category).is_some());
        assert_eq!(w_obj2.get_category(&test_category).unwrap().size(), 1);
        assert!(w_obj2.get_category(&test_category).unwrap()
            .get_item(&test_item).is_some());
        assert_eq!(w_obj2.get_category(&test_category).unwrap()
                       .get_item(&test_item).unwrap().size(), 0);
    }

    #[test]
    fn test_read_action() {
        let file_path: String = String::from("./tests/testdatabase.json");
        assert!(Path::new(&file_path).exists());
        let data = String::from(r#"{"Bank Accounts":{"Starling":{"Account Number":"12345678","Name":"Mr John Doe","Sort Code":"12-34-56"}},"Websites":{"Facebook":{"password":"pass1234fb","url":"https://www.facebook.com/","username":"example@gmail.com"},"Google":{"password":"pass1234","url":"https://www.google.com/","username":"example@gmail.com"},"Twitter":{"password":"r43rfsffdsfdsf","url":"https://www.twitter.com/","username":"example@gmail.com"}}}"#);
        let mut file: fs::File = fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&file_path)
            .expect("Unable to open file");
        assert!(file.write(data.as_bytes()).is_ok());

        let args = app::Args {
            database: String::from(file_path.clone()),
            action: Some(String::from("read")),
            category: None,
            item: None,
            entry: None,
        };

        // app::run(args).expect("Unable to run app.");
        let mut wallet = Wallet::new();
        wallet.load(&file_path);
        assert_eq!(data, app::generate_wallet_string(args, &mut wallet).unwrap());
    }

    #[test]
    fn test_delete_action() {}

    #[test]
    fn test_update_action() {}
}