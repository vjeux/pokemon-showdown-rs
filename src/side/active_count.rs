// NOTE: This method is NOT in JavaScript - Rust-specific implementation

use crate::side::*;

impl Side {

    /// Count active Pokemon
    pub fn active_count(&self) -> usize {
        self.active.iter().filter(|opt| opt.is_some()).count()
    }
}
