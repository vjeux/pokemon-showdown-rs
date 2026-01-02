// NOTE: This method is NOT in JavaScript - Rust-specific implementation

use crate::*;

impl Dex {

    /// Get total type effectiveness modifier against multiple types
    /// Returns sum of modifiers (matching TypeScript getEffectiveness with array)
    // TypeScript:
    // if (Array.isArray(targetTyping)) {
    //   for (const type of targetTyping) {
    //     totalTypeMod += this.getEffectiveness(sourceType, type);
    //   }
    //   return totalTypeMod;
    // }
    pub fn get_type_effectiveness(&self, attack_type: &str, defend_types: &[String]) -> i32 {
        let mut total_mod = 0;
        for defend_type in defend_types {
            total_mod += self.get_effectiveness(attack_type, defend_type);
        }
        total_mod
    }
}
