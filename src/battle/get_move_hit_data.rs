//! Battle::get_move_hit_data - Get or create move hit data for a Pokemon
//!
//! Equivalent to target.getMoveHitData(move) in JavaScript

use crate::*;

impl Battle {
    /// Get move hit data for a Pokemon
    /// Equivalent to target.getMoveHitData(move) in JavaScript
    ///
    /// Returns a mutable reference to MoveHitData for the given Pokemon's slot
    /// Creates a new entry if one doesn't exist
    ///
    /// JavaScript equivalent:
    /// ```javascript
    /// getMoveHitData(move: ActiveMove) {
    ///     if (!move.moveHitData) move.moveHitData = {};
    ///     const slot = this.getSlot();
    ///     return move.moveHitData[slot] || (move.moveHitData[slot] = {
    ///         crit: false,
    ///         typeMod: 0,
    ///         zBrokeProtect: false,
    ///     });
    /// }
    /// ```
    pub fn get_move_hit_data_mut(&mut self, pokemon_pos: (usize, usize)) -> Option<&mut pokemon::MoveHitData> {
        // Get the slot for the Pokemon
        let slot = {
            let pokemon = self.pokemon_at(pokemon_pos.0, pokemon_pos.1)?;
            pokemon.get_slot()
        };

        // Get or create move hit data for this slot
        if let Some(ref mut active_move) = self.active_move {
            Some(active_move.move_hit_data.entry(slot).or_insert_with(|| pokemon::MoveHitData {
                crit: false,
                type_mod: 0,
                damage: 0,
                hit_substitute: false,
            }))
        } else {
            None
        }
    }

    /// Get immutable reference to move hit data (if it exists)
    pub fn get_move_hit_data(&self, pokemon_pos: (usize, usize)) -> Option<&pokemon::MoveHitData> {
        let slot = {
            let pokemon = self.pokemon_at(pokemon_pos.0, pokemon_pos.1)?;
            pokemon.get_slot()
        };

        if let Some(ref active_move) = self.active_move {
            active_move.move_hit_data.get(&slot)
        } else {
            None
        }
    }
}
