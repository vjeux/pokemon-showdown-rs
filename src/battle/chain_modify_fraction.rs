// NOTE: This method is NOT in JavaScript - Rust-specific implementation

use crate::*;

impl Battle {

    /// Chain modify the event modifier using a fraction
    /// Used for precise ratios: battle.chain_modify_fraction(3, 2) for 1.5x
    pub fn chain_modify_fraction(&mut self, numerator: i32, denominator: i32) -> i32 {
        if let Some(ref mut event) = self.event {
            // Extract modifier value first to avoid borrow checker issues
            let modifier = event.modifier;
            eprintln!("[CHAIN_MODIFY_FRACTION] Input: numerator={}, denominator={}, previous_modifier={}",
                      numerator, denominator, modifier);

            // JS: const previousMod = this.trunc(this.event.modifier * 4096);
            // Modifier is already in 4096 basis points
            let previous_mod = modifier;

            // JS: const nextMod = this.trunc(numerator * 4096 / denominator);
            let next_mod = ((numerator as i64 * 4096) / denominator as i64) as i32;
            eprintln!("[CHAIN_MODIFY_FRACTION] previous_mod={}, next_mod={}", previous_mod, next_mod);

            // JS: this.event.modifier = ((previousMod * nextMod + 2048) >> 12) / 4096;
            // Result stays in 4096 basis points
            let new_modifier = ((previous_mod as i64 * next_mod as i64 + 2048) >> 12) as i32;
            eprintln!("[CHAIN_MODIFY_FRACTION] new_modifier={}", new_modifier);
            event.modifier = new_modifier;
            new_modifier
        } else {
            4096 // Default modifier (1.0 in 4096 basis points)
        }
    }
}
