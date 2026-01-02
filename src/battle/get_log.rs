// NOTE: This method is NOT in JavaScript - Rust-specific implementation

use crate::*;

impl Battle {

    /// Get the battle log as a string
    /// Rust convenience method - JavaScript directly accesses this.log array
    pub fn get_log(&self) -> String {
        self.log.join("\n")
    }
}
