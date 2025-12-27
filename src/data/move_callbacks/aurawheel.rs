//! Aura Wheel Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onTry(source) {
///     if (source.species.baseSpecies === 'Morpeko') {
///         return;
///     }
///     this.attrLastMove('[still]');
///     this.add('-fail', source, 'move: Aura Wheel');
///     this.hint("Only a Pokemon whose form is Morpeko or Morpeko-Hangry can use this move.");
///     return null;
/// }
pub fn on_try(battle: &mut Battle, source_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

/// onModifyType(move, pokemon) {
///     if (pokemon.species.name === 'Morpeko-Hangry') {
///         move.type = 'Dark';
///     } else {
///         move.type = 'Electric';
///     }
/// }
pub fn on_modify_type(battle: &mut Battle, move_id: &str, pokemon_pos: (usize, usize)) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

