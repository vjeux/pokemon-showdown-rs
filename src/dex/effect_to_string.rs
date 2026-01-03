// Dex::effectToString - Convert effect to string representation
//
// JS Source:
//
// 	effectToString() {
// 		return this.name;
// 	}

use crate::*;

impl Dex {
    /// Convert effect to string representation
    /// Equivalent to effectToString() in dex.ts
    ///
    /// JavaScript (dex.ts):
    ///   effectToString() {
    ///     return this.name;
    ///   }
    ///
    /// Note: In JavaScript this is a method on Effect objects, not Dex
    /// This is a placeholder - actual implementation would be on Effect trait/struct
    pub fn effect_to_string(name: &str) -> String {
        // return this.name;
        name.to_string()
    }
}
