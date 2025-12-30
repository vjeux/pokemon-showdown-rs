use crate::*;

impl Battle {

    /// Internal modifier calculation with 4096 fixed-point arithmetic
    /// Rust helper method - JavaScript uses inline calculation or this.modify()
    /// This method implements 4096-based fixed-point multiplication
    /// Formula: result = ((value * modifier + 2048) >> 12).max(1)
    /// modifier is in 4096 basis points (e.g., 6144 = 1.5x, 2048 = 0.5x)
    pub fn modify_internal(&self, value: i32, modifier: i32) -> i32 {
        // 4096-based fixed-point multiplication
        // modifier is already in 4096 basis (e.g., 6144 = 1.5x)
        let result = value as i64 * modifier as i64;
        ((result + 2048) >> 12).max(1) as i32
    }
}
