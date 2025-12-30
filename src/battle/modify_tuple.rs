use crate::*;

impl Battle {

    /// Apply a modifier to a value using a tuple for numerator/denominator
    /// Rust convenience method - JavaScript modify() accepts arrays: modify(value, [numerator, denominator])
    /// This provides a type-safe alternative using Rust tuples
    pub fn modify_tuple(&self, value: i32, fraction: (i32, i32)) -> i32 {
        self.modify(value, fraction.0, fraction.1)
    }
}
