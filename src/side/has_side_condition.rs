// NOTE: This method is NOT in JavaScript - Rust-specific implementation

use crate::side::*;

impl Side {

    /// Check if a side condition is active
    pub fn has_side_condition(&self, id: &ID) -> bool {
        self.side_conditions.contains_key(id)
    }
}
