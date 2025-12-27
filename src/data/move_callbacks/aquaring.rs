//! Aqua Ring Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;


pub mod condition {
    use super::*;

    /// onStart(pokemon) {
    ///     this.add('-start', pokemon, 'Aqua Ring');
    /// }
    pub fn on_start(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
        // TODO: battle.add('-start', pokemon, 'Aqua Ring');
        EventResult::Continue
    }

    /// onResidual(pokemon) {
    ///     this.heal(pokemon.baseMaxhp / 16);
    /// }
    pub fn on_residual(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
        let base_maxhp = if let Some(pokemon) = battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            pokemon.base_maxhp
        } else {
            return EventResult::Continue;
        };

        // Heal 1/16 of base max HP each turn
        let heal_amount = base_maxhp / 16;
        battle.heal(heal_amount, Some(pokemon_pos), None, None);

        EventResult::Continue
    }
}
