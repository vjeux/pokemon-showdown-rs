// NOTE: This method is NOT in JavaScript - Rust-specific implementation

use crate::*;

impl Pokemon {

    /// Copy volatiles from another Pokemon (for Baton Pass)
    pub fn copy_volatile_from_full(&mut self, source: &Pokemon, is_shed_tail: bool) {
        // Note: This is similar to copy_volatile_from.rs but includes boosts
        // Note: JavaScript has copyVolatileFrom which this implements

        // Copy boosts unless Shed Tail
        // Note: Shed Tail only passes Substitute, not boosts
        if !is_shed_tail {
            self.boosts = source.boosts;
        }

        // List of volatiles that can be copied
        // Note: Hardcoded list instead of checking condition.noCopy flag
        // Note: Should loop through source volatiles and check noCopy dynamically
        let copyable_volatiles = [
            "aquaring",
            "confusion",
            "curse",
            "embargo",
            "focusenergy",
            "gmaxchistrike",
            "healblock",
            "ingrain",
            "laserfocus",
            "leechseed",
            "magnetrise",
            "perishsong",
            "powertrick",
            "substitute",
            "telekinesis",
            "torment",
        ];

        for volatile_name in copyable_volatiles {
            // Shed Tail only copies Substitute
            if is_shed_tail && volatile_name != "substitute" {
                continue;
            }
            let id = ID::new(volatile_name);
            if let Some(state) = source.get_volatile(&id) {
                self.volatiles.insert(id, state.clone());
            }
        }

        // Note: Missing linkedPokemon bidirectional link updating
        // Note: Missing source.clearVolatile() call (would need &mut source)
        // Note: Missing singleEvent('Copy') calls for each copied volatile
    }
}
