// NOTE: This method is NOT in JavaScript - Rust-specific implementation

use crate::*;

impl Battle {

    /// Get the next effect order number
    /// Rust helper - JavaScript uses this.effectOrder++ inline in initEffectState
    /// Returns current value and then increments (post-increment behavior)
    pub fn next_effect_order(&mut self) -> i32 {
        let old = self.effect_order;
        self.effect_order += 1;
        old
    }
}
