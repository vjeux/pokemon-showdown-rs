use crate::*;

impl Battle {

    /// Apply a floating-point modifier to a value
    /// Rust convenience method - JavaScript modify() accepts floats directly: modify(value, 1.5)
    /// This provides type-safe handling when the modifier is already a float
    pub fn modify_f(&self, value: i32, multiplier: f64) -> i32 {
        // JS: const modifier = tr(numerator * 4096 / denominator);
        let modifier = self.trunc(multiplier * 4096.0);
        // JS: return tr((tr(value * modifier) + 2048 - 1) / 4096);
        let inner = self.trunc((value * modifier) as f64);
        self.trunc((inner + 2048 - 1) as f64 / 4096.0)
    }
}
