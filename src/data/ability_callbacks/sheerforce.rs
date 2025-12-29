//! Sheer Force Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onModifyMove(move, pokemon) {
///     if (move.secondaries) {
///         delete move.secondaries;
///         // Technically not a secondary effect, but it is negated
///         delete move.self;
///         if (move.id === 'clangoroussoulblaze') delete move.selfBoost;
///         // Actual negation of `AfterMoveSecondary` effects implemented in scripts.js
///         move.hasSheerForce = true;
///     }
/// }
pub fn on_modify_move(battle: &mut Battle, pokemon_pos: (usize, usize), _move_id: &str) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

/// onBasePower(basePower, pokemon, target, move) {
///     if (move.hasSheerForce) return this.chainModify([5325, 4096]);
/// }
pub fn on_base_power(battle: &mut Battle, pokemon_pos: (usize, usize), _move_id: &str) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

