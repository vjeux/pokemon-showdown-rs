// NOTE: This method is NOT in JavaScript - Rust-specific implementation

use crate::*;

impl Pokemon {

    /// Check if Pokemon has a specific status
    pub fn has_status(&self, status: &str) -> bool {
        self.status.as_str() == status.to_lowercase()
    }
}
