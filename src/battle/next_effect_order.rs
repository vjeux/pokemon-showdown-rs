use crate::*;

impl Battle {

    /// Get the next effect order number
    /// Rust helper - JavaScript uses this.effectOrder++ inline in initEffectState
    pub fn next_effect_order(&mut self) -> i32 {
        self.effect_order += 1;
        self.effect_order
    }
}
