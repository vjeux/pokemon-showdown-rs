//! Sparkling Aria Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onAfterMove(source, target, move) {
///     if (source.fainted || !move.hitTargets || move.hasSheerForce) {
///         // make sure the volatiles are cleared
///         for (const pokemon of this.getAllActive()) delete pokemon.volatiles['sparklingaria'];
///         return;
///     }
///     const numberTargets = move.hitTargets.length;
///     for (const pokemon of move.hitTargets) {
///         // bypasses Shield Dust when hitting multiple targets
///         if (pokemon !== source && pokemon.isActive && (pokemon.removeVolatile('sparklingaria') || numberTargets > 1) &&
///             pokemon.status === 'brn') {
///             pokemon.cureStatus();
///         }
///     }
/// }
pub fn on_after_move(battle: &mut Battle, source_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

