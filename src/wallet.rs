use std::collections::HashMap;
use crate::category::Category;

pub(crate) struct Wallet {
    categories: HashMap<String, Category>,
}

impl Wallet {
    fn new() -> Wallet {
        Wallet {
            categories: HashMap::new(),
        }
    }

    fn size() -> usize {
        return 0;
    }

    fn empty() -> bool {
        return false;
    }

    fn new_category(&mut self, category_identifier: &String) -> Category {
        let category = Category::new(category_identifier.to_string());
        return category;
    }

}