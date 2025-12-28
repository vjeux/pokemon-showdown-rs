//! Power Herb Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onChargeMove(pokemon, target, move) {
///     if (pokemon.useItem()) {
///         this.debug('power herb - remove charge turn for ' + move.id);
///         this.attrLastMove('[still]');
///         this.addMove('-anim', pokemon, move.name, target);
///         return false; // skip charge turn
///     }
/// }
pub fn on_charge_move(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>, move_id: &str) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}
