// NOTE: This method is NOT in JavaScript - Rust-specific implementation

use crate::*;
use crate::side::Side;

impl Battle {

    /// Get a mutable side by ID
    /// Rust-specific helper for mutable access (JavaScript doesn't need this due to no borrow checker)
    pub fn get_side_mut(&mut self, side_id: SideID) -> Option<&mut Side> {
        self.sides.get_mut(side_id.index())
    }
}
