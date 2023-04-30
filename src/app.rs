pub mod app {
    use crate::category::Category;
    use crate::wallet::Wallet;

    const STUDENT_NUMBER: &str = "851784";

    enum Action {
        CREATE,
        READ,
        UPDATE,
        DELETE,
    }

    pub fn run() {}

    fn parse_action_argument(args: Vec<String>) {

    }

    fn execute_delete_action(args: Vec<String>, w_obj: &mut Wallet) {

    }

    fn execute_update_action(args: Vec<String>, w_obj: &mut Wallet) {

    }

    fn execute_read_action(args: Vec<String>, w_obj: &mut Wallet) {

    }

    fn execute_create_action(args: Vec<String>, w_obj: &mut Wallet) {

    }

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

    fn process_entry_update(args: Vec<String>, key_delimiter: &String, cur_cat: &Category, cur_item_ident: &String) {

    }

    fn process_item_update(key_delimiter: &String, cur_cat: &Category, item_input: &String, cur_item_ident: &String) {

    }

    fn process_category_update(w_obj: &mut Wallet, key_delimiter: &String, cat_input: &String,  cur_cat_ident: &String) {

    }
}

