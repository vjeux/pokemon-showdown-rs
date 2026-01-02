// NOTE: This method is NOT in JavaScript - Rust-specific implementation

use crate::*;
use crate::side::Side;

impl Battle {

    /// Get P1
    //
    // 	get p1() {
    // 		return this.sides[0];
    // 	}
    //
    pub fn p1(&self) -> Option<&Side> {
        self.sides.first()
    }
}
