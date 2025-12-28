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
    // if (pokemon.useItem())
    // TODO: Need pokemon.useItem(), battle.debug(), battle.attrLastMove(), battle.addMove()
    // This callback skips the charge turn for charging moves like Solar Beam
    // return false to skip charge turn if item is used successfully
    EventResult::Continue
}
