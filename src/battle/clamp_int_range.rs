// NOTE: This method is NOT in JavaScript - Rust-specific implementation

use crate::*;

impl Battle {

    /// Clamp an integer to a minimum value
    /// Equivalent to battle.ts clampIntRange(value, min, max)
    ///
    // TypeScript source:
    // 	clampIntRange(num: any, min?: number, max?: number) {
    // 		if (typeof num !== 'number') return 0;
    // 		num = Math.floor(num);
    // 		if (min !== undefined) num = Math.max(num, min);
    // 		if (max !== undefined) num = Math.min(num, max);
    // 		return num;
    // 	}
    //
    pub fn clamp_int_range(&self, value: i32, min: Option<i32>, max: Option<i32>) -> i32 {
        let mut result = value;
        if let Some(min_val) = min {
            result = result.max(min_val);
        }
        if let Some(max_val) = max {
            result = result.min(max_val);
        }
        result
    }
}
