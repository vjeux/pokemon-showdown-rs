// NOTE: This method is NOT in JavaScript - Rust-specific implementation

use crate::*;

impl Pokemon {

    /// Get base moves as string list
    /// Equivalent to baseMoves getter in pokemon.ts
    pub fn get_base_moves(&self) -> Vec<String> {
        self.base_move_slots
            .iter()
            .map(|slot| slot.id.as_str().to_string())
            .collect()
    }
}
