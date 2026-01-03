use crate::*;

impl Battle {
    /// Decrement a Pokemon's active move actions counter
    /// JavaScript equivalent: pokemon.activeMoveActions--
    ///
    /// This field tracks how many move actions a Pokemon has left in the current turn.
    /// It's used by moves like Sky Drop to nullify the attacker's move action.
    ///
    /// In JavaScript: pokemon.activeMoveActions--
    /// In Rust: battle.decrement_active_move_actions(pokemon_pos)
    pub fn decrement_active_move_actions(&mut self, pokemon_pos: (usize, usize)) {
        if let Some(pokemon) = self.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
            pokemon.active_move_actions -= 1;
        }
    }
}
