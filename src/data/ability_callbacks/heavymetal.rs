//! Heavy Metal Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onModifyWeight(weighthg) {
///     return weighthg * 2;
/// }
pub fn on_modify_weight(_battle: &mut Battle, weight: i32, _pokemon_pos: (usize, usize)) -> EventResult {
    EventResult::Number(weight * 2)
}

