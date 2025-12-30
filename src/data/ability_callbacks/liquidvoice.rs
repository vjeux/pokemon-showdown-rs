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
pub fn on_modify_type(_battle: &mut Battle, _move_id: &str, _pokemon_pos: (usize, usize), _target_pos: Option<(usize, usize)>) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

