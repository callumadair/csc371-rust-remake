pub mod app {
    use getopts::Options;
    use crate::category::Category;
    use crate::wallet::Wallet;

    const STUDENT_NUMBER: &str = "851784";

    enum Action {
        CREATE,
        READ,
        UPDATE,
        DELETE,
    }

    fn print_usage(program: &str, opts: Options) {
        let brief = format!("Usage: {} FILE [options]", program);
        print!("{}", opts.usage(&brief));
    }

    pub fn run() {
        let options = opts_setup();
        let args: Vec<String> = std::env::args().collect();

        let program = args[0].clone();

        let matches = match options.parse(&args[1..]) {
            Ok(m) => { m }
            Err(f) => { panic!("{}", f.to_string()) }
        };

        if matches.opt_present("h") {
            print_usage(&program, options);
            return;
        }

        let db_filename: String = matches.opt_str("d").unwrap();
        let w_obj = Wallet::new();
        w_obj.load(&db_filename);

        let action: Action = parse_action_argument(args);
    }

    fn opts_setup() -> Options {
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

    fn parse_action_argument(args: Vec<String>) -> Action {

        let action_index = args.iter().position(|r| r == "-a" || r == "--action");
        let input: String = args[action_index.unwrap() + 1].clone().to_uppercase();

        match input.as_str() {
            "CREATE" => return Action::CREATE,
            "READ" => return Action::READ,
            "UPDATE" => return Action::UPDATE,
            "DELETE" => return Action::DELETE,
            _ => panic!("Invalid action argument: {}", input),
        }
    }

    fn execute_delete_action(args: Vec<String>, w_obj: &mut Wallet) {}

    fn execute_update_action(args: Vec<String>, w_obj: &mut Wallet) {}

    fn execute_read_action(args: Vec<String>, w_obj: &mut Wallet) {}

    fn execute_create_action(args: Vec<String>, w_obj: &mut Wallet) {}

    fn get_wallet_json(w: &Wallet) -> String {
        return String::from("");
    }

    fn get_category_json(w: &Wallet, c: &String) -> String {
        return String::from("");
    }

    fn get_item_json(w: &Wallet, c: &String, i: &String) -> String {
        return String::from("");
    }

    fn get_entry_json(w: &Wallet, c: &String, i: &String, e: &String) -> String {
        return String::from("");
    }

    fn process_entry_update(args: Vec<String>, key_delimiter: &String, cur_cat: &Category, cur_item_ident: &String) {}

    fn process_item_update(key_delimiter: &String, cur_cat: &Category, item_input: &String, cur_item_ident: &String) {}

    fn process_category_update(w_obj: &mut Wallet, key_delimiter: &String, cat_input: &String, cur_cat_ident: &String) {}
}

