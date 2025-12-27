//! Tidy Up Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onHit(pokemon) {
///     let success = false;
///     for (const active of this.getAllActive()) {
///         if (active.removeVolatile('substitute')) success = true;
///     }
///     const removeAll = ['spikes', 'toxicspikes', 'stealthrock', 'stickyweb', 'gmaxsteelsurge'];
///     const sides = [pokemon.side, ...pokemon.side.foeSidesWithConditions()];
///     for (const side of sides) {
///         for (const sideCondition of removeAll) {
///             if (side.removeSideCondition(sideCondition)) {
///                 this.add('-sideend', side, this.dex.conditions.get(sideCondition).name);
///                 success = true;
///             }
///         }
///     }
///     if (success) this.add('-activate', pokemon, 'move: Tidy Up');
///     return !!this.boost({ atk: 1, spe: 1 }, pokemon, pokemon, null, false, true) || success;
/// }
pub fn on_hit(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>, move_id: Option<&str>) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

