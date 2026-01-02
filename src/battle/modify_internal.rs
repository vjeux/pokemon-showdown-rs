// NOTE: This method is NOT in JavaScript - Rust-specific implementation

use crate::*;

impl Battle {

    /// Internal modifier calculation with 4096 fixed-point arithmetic
    /// Equivalent to JavaScript's modify() function in battle.ts
    ///
    /// JavaScript:
    /// modify(value, numerator, denominator = 1) {
    ///     const tr = this.trunc;
    ///     const modifier = tr(numerator * 4096 / denominator);
    ///     return tr((tr(value * modifier) + 2048 - 1) / 4096);
    /// }
    ///
    /// When calling from run_event with event.modifier (which is already in 4096 basis),
    /// we skip the conversion and apply the formula directly.
    pub fn modify_internal(&self, value: i32, modifier: i32) -> i32 {
        // JavaScript: return tr((tr(value * modifier) + 2048 - 1) / 4096);
        // Note: modifier is already in 4096 basis (e.g., 6144 = 1.5x)

        let product = value as i64 * modifier as i64;
        let truncated_product = self.trunc(product as f64, None) as i64;
        let adjusted = truncated_product + 2048 - 1;  // JavaScript uses + 2048 - 1, not + 2048
        let result = self.trunc(adjusted as f64 / 4096.0, None) as i32;

        result.max(1)
    }
}
