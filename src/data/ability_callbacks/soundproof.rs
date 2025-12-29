//! Soundproof Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onTryHit(target, source, move) {
///     if (target !== source && move.flags['sound']) {
///         this.add('-immune', target, '[from] ability: Soundproof');
///         return null;
///     }
/// }
pub fn on_try_hit(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

/// onAllyTryHitSide(target, source, move) {
///     if (move.flags['sound']) {
///         this.add('-immune', this.effectState.target, '[from] ability: Soundproof');
///     }
/// }
pub fn on_ally_try_hit_side(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

