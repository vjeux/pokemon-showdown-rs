// NOTE: This method is NOT in JavaScript - Rust-specific implementation

use crate::side::*;

impl Side {

    /// Get the Side ID as a string
    pub fn id_str(&self) -> &'static str {
        self.id.to_str()
    }
}
