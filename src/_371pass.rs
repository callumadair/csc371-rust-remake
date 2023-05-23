pub mod app {
    use std::{env::Args,
              io::{Error, ErrorKind},
    };
    use getopts::Options;
    use crate::{category::Category, wallet::Wallet};

    const STUDENT_NUMBER: &str = "851784";

    #[derive(Debug, PartialEq)]
    pub(crate) enum Action {
        CREATE,
        READ,
        UPDATE,
        DELETE,
    }

    fn print_usage(program: &str, opts: Options) {
        let brief = format!("Usage: {} FILE [options]", program);
        print!("{}", opts.usage(&brief));
    }

    pub fn run(args: Vec<String>) -> Result<(), Error> {
        let options = opts_setup();
        dbg!(&args);
        let program = args[0].clone();

        let matches = match options.parse(&args[1..]) {
            Ok(m) =>
                { m }
            Err(f) =>
                { panic!("{}", f.to_string()) }
        };

        if matches.opt_present("h") {
            print_usage(&program, options);
            return Ok(());
        }

        let db_filename: String = matches.opt_str("d").unwrap();
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

    pub(crate) fn opts_setup() -> Options {
        let mut opts = Options::new();

        opts.optopt("d",
                    "database",
                    "Filename of the 371pass database",
                    "DATABASE");

        opts.optopt("a",
                    "action",
                    "Action to take, can be: 'create', 'read', 'update', 'delete'.",
                    "ACTION");

        opts.optopt("c",
                    "category",
                    "Apply action to a category (e.g., if you want to add a category, set the \
                    action argument to 'add' and the category argument to your chosen category \
                    identifier).",
                    "CATEGORY");

        opts.optopt("i",
                    "item",
                    "Apply action to an item (e.g., if you want to add an item, set the \
                    action argument to 'add' and the item argument to your chosen item identifier).",
                    "ITEM");

        opts.optopt("e",
                    "entry",
                    "Apply action to an entry (e.g., if you want to add an entry, set the \
                    action argument to 'add', the category argument to your chosen category \
                    identifier, the item argument to your chosen item identifier, and the entry \
                    argument to the string 'key,value'). If there is no comma, an empty entry is \
                    inserted. If you are simply retrieving an entry, set the entry argument to the \
                    'key'. If you are updating an entry key, use a : e.g., oldkey:newkey,newvalue.",
                    "ENTRY");

        opts.optflag("h", "help", "Prints this help menu.");

        return opts;
    }

    pub(crate) fn parse_action_argument(args: Vec<String>) -> Result<Action, Error> {
        let action_index = args
            .iter()
            .position(|r| r == "-a" || r == "--action")
            .expect("Invalid action argument.");
        let input: String = args[action_index + 1]
            .clone()
            .to_uppercase();

        return match Some(input.as_str()) {
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

    pub(crate) fn execute_read_action(args: Vec<String>, w_obj: &mut Wallet) -> Result<(), Error> {
        Ok(())
    }

    fn execute_create_action(args: Vec<String>, w_obj: &mut Wallet) -> Result<(), Error> {
        Ok(())
    }

    fn execute_update_action(args: Vec<String>, w_obj: &mut Wallet) -> Result<(), Error> {
        Ok(())
    }

    fn process_category_update(w_obj: &mut Wallet,
                               key_delimiter: &String,
                               cat_input: &String,
                               cur_cat_ident: &String) -> () {}

    fn process_item_update(key_delimiter: &String,
                           cur_cat: &Category,
                           item_input: &String,
                           cur_item_ident: &String) -> () {}

    fn process_entry_update(args: Vec<String>,
                            key_delimiter: &String,
                            cur_cat: &Category,
                            cur_item_ident: &String) -> () {}

    fn execute_delete_action(args: Vec<String>, w_obj: &mut Wallet) -> Result<(), Error> {
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
        _371pass::app,
        wallet::Wallet,
    };

    #[test]
    fn test_args_parsing() {
        let mut args_vec: Vec<String> = Vec::new();
        args_vec.push(String::from("target/debug/csc371_remake"));
        args_vec.push(String::from("--action"));
        args_vec.push(String::from("invalid"));

        let opts = app::opts_setup();
        let expected_error = Error::new(ErrorKind::InvalidInput, "Invalid action argument.");
        let result = app::parse_action_argument(args_vec.clone());

        assert!(result.is_err());
        assert_eq!(result.as_ref()
                       .unwrap_err()
                       .kind(),
                   expected_error.kind());
        assert_eq!(result.as_ref()
                       .unwrap_err()
                       .to_string(),
                   expected_error.to_string());

        args_vec.pop();
        args_vec.push(String::from("create"));
        let result = app::parse_action_argument(args_vec.clone());
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), app::Action::CREATE);

        args_vec.pop();
        args_vec.push(String::from("read"));
        let result = app::parse_action_argument(args_vec.clone());
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), app::Action::READ);

        args_vec.pop();
        args_vec.push(String::from("update"));
        let result = app::parse_action_argument(args_vec.clone());
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), app::Action::UPDATE);

        args_vec.pop();
        args_vec.push(String::from("delete"));
        let result = app::parse_action_argument(args_vec.clone());
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), app::Action::DELETE);
    }

    #[test]
    fn test_read_action() {
        let file_path: String = String::from("./tests/testdatabase.json");
        assert!(Path::new(&file_path).exists());
        let data = String::from(r#"{
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

        let mut args_vec: Vec<String> = Vec::new();
        args_vec.push(String::from("target/debug/csc371_remake"));
        args_vec.push(String::from("--database"));
        args_vec.push(String::from(file_path.clone()));
        args_vec.push(String::from("--action"));
        args_vec.push(String::from("read"));

        app::run(args_vec.clone()).expect("Unable to run app.");
    }
}