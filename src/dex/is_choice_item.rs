use crate::*;

impl Dex {

    /// Check if an item is a choice item
    pub fn is_choice_item(&self, item_name: &str) -> bool {
        self.items().get(item_name)
            .map(|i| i.is_choice)
            .unwrap_or(false)
    }
}
