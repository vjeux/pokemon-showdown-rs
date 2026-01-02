// NOTE: This method is NOT in JavaScript - Rust-specific implementation

use crate::*;

impl Battle {

    /// Check if pokemon is fainted
    /// Rust helper method - JavaScript directly accesses pokemon.fainted or checks pokemon.hp <= 0
    /// This provides a safe accessor that handles the tuple position format used in Rust
    pub fn is_pokemon_fainted(&self, pos: (usize, usize)) -> bool {
        self.sides
            .get(pos.0)
            .and_then(|s| s.pokemon.get(pos.1))
            .is_none_or(|p| p.is_fainted())
    }
}
