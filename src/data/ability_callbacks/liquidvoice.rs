//! Liquid Voice Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onModifyType(move, pokemon) {
///     if (move.flags['sound'] && !pokemon.volatiles['dynamax']) { // hardcode
///         move.type = 'Water';
///     }
/// }
pub fn on_modify_type(battle: &mut Battle, move_id: &str, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

