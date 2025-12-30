use crate::*;

impl Battle {

    /// Chain two modifiers together (number variant)
    /// Rust convenience method - JavaScript chain() accepts both numbers and arrays via dynamic typing
    /// When both modifiers are simple multipliers (not fractions), this avoids tuple construction
    pub fn chain_f(&self, previous_mod: f64, next_mod: f64) -> f64 {
        // JS: previousMod = this.trunc(previousMod * 4096);
        let prev = self.trunc(previous_mod * 4096.0, None) as i32;
        // JS: nextMod = this.trunc(nextMod * 4096);
        let next = self.trunc(next_mod * 4096.0, None) as i32;
        // JS: return ((previousMod * nextMod + 2048) >> 12) / 4096;
        let result = ((prev * next) + 2048) >> 12;
        result as f64 / 4096.0
    }
}
