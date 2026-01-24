//! Battle::get_move_hit_data - Get or create move hit data for a Pokemon
//!
//! Equivalent to target.getMoveHitData(move) in JavaScript

use crate::*;

impl Battle {
    /// Get move hit data for a Pokemon with a callback for mutation
    /// Equivalent to target.getMoveHitData(move) in JavaScript
    ///
    /// Takes a closure that receives a mutable reference to the MoveHitData
    /// Returns true if the callback was invoked, false if no active move
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
    pub fn with_move_hit_data_mut<F>(&mut self, pokemon_pos: (usize, usize), f: F) -> bool
    where
        F: FnOnce(&mut pokemon::MoveHitData),
    {
        // Get the slot for the Pokemon
        let slot = match self.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(pokemon) => pokemon.get_slot(),
            None => return false,
        };

        // Get or create move hit data for this slot
        if let Some(ref active_move) = self.active_move {
            let mut borrowed = active_move.borrow_mut();
            let entry = borrowed.move_hit_data.entry(slot).or_insert_with(|| pokemon::MoveHitData {
                crit: false,
                type_mod: 0,
                damage: 0,
                hit_substitute: false,
                z_broke_protect: false,
            });
            f(entry);
            true
        } else {
            false
        }
    }

    /// Legacy function that returns Option<&mut MoveHitData>
    /// Note: This creates a temporary copy that needs to be set back with set_move_hit_data
    pub fn get_move_hit_data_mut(&mut self, pokemon_pos: (usize, usize)) -> Option<pokemon::MoveHitData> {
        let slot = match self.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(pokemon) => pokemon.get_slot(),
            None => return None,
        };

        if let Some(ref active_move) = self.active_move {
            let mut borrowed = active_move.borrow_mut();
            let data = borrowed.move_hit_data.entry(slot).or_insert_with(|| pokemon::MoveHitData {
                crit: false,
                type_mod: 0,
                damage: 0,
                hit_substitute: false,
                z_broke_protect: false,
            }).clone();
            Some(data)
        } else {
            None
        }
    }

    /// Set move hit data for a Pokemon slot
    pub fn set_move_hit_data(&mut self, pokemon_pos: (usize, usize), data: pokemon::MoveHitData) {
        let slot = match self.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(pokemon) => pokemon.get_slot(),
            None => return,
        };

        if let Some(ref active_move) = self.active_move {
            active_move.borrow_mut().move_hit_data.insert(slot, data);
        }
    }

    /// Get immutable reference to move hit data (if it exists)
    pub fn get_move_hit_data(&self, pokemon_pos: (usize, usize)) -> Option<pokemon::MoveHitData> {
        let slot = {
            let pokemon = self.pokemon_at(pokemon_pos.0, pokemon_pos.1)?;
            pokemon.get_slot()
        };

        if let Some(ref active_move) = self.active_move {
            active_move.borrow().move_hit_data.get(&slot).cloned()
        } else {
            None
        }
    }
}
