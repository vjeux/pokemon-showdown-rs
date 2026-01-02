// NOTE: This method is NOT in JavaScript - Rust-specific implementation

use crate::*;
use crate::side::Side;

impl Battle {

    /// Get P2
    //
    // 	get p2() {
    // 		return this.sides[1];
    // 	}
    //
    pub fn p2(&self) -> Option<&Side> {
        self.sides.get(1)
    }
}
