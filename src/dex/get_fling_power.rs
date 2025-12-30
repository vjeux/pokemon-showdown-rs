use crate::*;

impl Dex {

    /// Get fling base power for an item
    pub fn get_fling_power(&self, item_name: &str) -> i32 {
        self.items().get(item_name)
            .and_then(|i| i.fling.as_ref())
            .map(|f| f.base_power)
            .unwrap_or(0)
    }
}
