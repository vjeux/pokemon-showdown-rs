//! Mortal Spin Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onAfterHit(target, pokemon, move) {
///     if (!move.hasSheerForce) {
///         if (pokemon.hp && pokemon.removeVolatile('leechseed')) {
///             this.add('-end', pokemon, 'Leech Seed', '[from] move: Mortal Spin', `[of] ${pokemon}`);
///         }
///         const sideConditions = ['spikes', 'toxicspikes', 'stealthrock', 'stickyweb', 'gmaxsteelsurge'];
///         for (const condition of sideConditions) {
///             if (pokemon.hp && pokemon.side.removeSideCondition(condition)) {
///                 this.add('-sideend', pokemon.side, this.dex.conditions.get(condition).name, '[from] move: Mortal Spin', `[of] ${pokemon}`);
///             }
///         }
///         if (pokemon.hp && pokemon.volatiles['partiallytrapped']) {
///             pokemon.removeVolatile('partiallytrapped');
///         }
///     }
/// }
pub fn on_after_hit(battle: &mut Battle, target_pos: Option<(usize, usize)>, pokemon_pos: (usize, usize), move_id: &str) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

/// onAfterSubDamage(damage, target, pokemon, move) {
///     if (!move.hasSheerForce) {
///         if (pokemon.hp && pokemon.removeVolatile('leechseed')) {
///             this.add('-end', pokemon, 'Leech Seed', '[from] move: Mortal Spin', `[of] ${pokemon}`);
///         }
///         const sideConditions = ['spikes', 'toxicspikes', 'stealthrock', 'stickyweb', 'gmaxsteelsurge'];
///         for (const condition of sideConditions) {
///             if (pokemon.hp && pokemon.side.removeSideCondition(condition)) {
///                 this.add('-sideend', pokemon.side, this.dex.conditions.get(condition).name, '[from] move: Mortal Spin', `[of] ${pokemon}`);
///             }
///         }
///         if (pokemon.hp && pokemon.volatiles['partiallytrapped']) {
///             pokemon.removeVolatile('partiallytrapped');
///         }
///     }
/// }
pub fn on_after_sub_damage(battle: &mut Battle, damage: i32, target_pos: Option<(usize, usize)>, pokemon_pos: (usize, usize), move_id: &str) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

