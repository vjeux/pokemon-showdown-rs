// NOTE: This method is NOT in JavaScript - Rust-specific implementation

use crate::side::*;

impl Side {

    /// Check if side has lost
    pub fn has_lost(&self) -> bool {
        self.count_unfainted() == 0
    }
}
