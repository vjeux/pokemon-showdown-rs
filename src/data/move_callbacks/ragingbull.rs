//! Raging Bull Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onTryHit(pokemon) {
///     // will shatter screens through sub, before you hit
///     pokemon.side.removeSideCondition('reflect');
///     pokemon.side.removeSideCondition('lightscreen');
///     pokemon.side.removeSideCondition('auroraveil');
/// }
pub fn on_try_hit(battle: &mut Battle, source_pos: (usize, usize), target_pos: (usize, usize)) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

/// onModifyType(move, pokemon) {
///     switch (pokemon.species_id.name) {
///     case 'Tauros-Paldea-Combat':
///         move.type = 'Fighting';
///         break;
///     case 'Tauros-Paldea-Blaze':
///         move.type = 'Fire';
///         break;
///     case 'Tauros-Paldea-Aqua':
///         move.type = 'Water';
///         break;
///     }
/// }
pub fn on_modify_type(battle: &mut Battle, move_id: &str, pokemon_pos: (usize, usize)) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

