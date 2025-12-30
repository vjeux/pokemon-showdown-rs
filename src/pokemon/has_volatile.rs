use crate::*;

impl Pokemon {

    /// Check if Pokemon has a specific volatile
    pub fn has_volatile(&self, id: &ID) -> bool {
        self.volatiles.contains_key(id)
    }
}
