//! Tera Starstorm Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onModifyType(move, pokemon) {
///     if (pokemon.species_id.as_str() === 'Terapagos-Stellar') {
///         move.type = 'Stellar';
///         if (pokemon.terastallized && pokemon.getStat('atk', false, true) > pokemon.getStat('spa', false, true)) {
///             move.category = 'Physical';
///         }
///     }
/// }
pub fn on_modify_type(_battle: &mut Battle, _move_id: &str, _pokemon_pos: (usize, usize)) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

/// onModifyMove(move, pokemon) {
///     if (pokemon.species_id.as_str() === 'Terapagos-Stellar') {
///         move.target = 'allAdjacentFoes';
///     }
/// }
pub fn on_modify_move(_battle: &mut Battle, _pokemon_pos: (usize, usize), _target_pos: Option<(usize, usize)>) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

