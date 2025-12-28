//! Rapid Spin Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onAfterHit(target, pokemon, move) {
///     if (!move.hasSheerForce) {
///         if (pokemon.hp && pokemon.removeVolatile('leechseed')) {
///             this.add('-end', pokemon, 'Leech Seed', '[from] move: Rapid Spin', `[of] ${pokemon}`);
///         }
///         const sideConditions = ['spikes', 'toxicspikes', 'stealthrock', 'stickyweb', 'gmaxsteelsurge'];
///         for (const condition of sideConditions) {
///             if (pokemon.hp && pokemon.side.removeSideCondition(condition)) {
///                 this.add('-sideend', pokemon.side, this.dex.conditions.get(condition).name, '[from] move: Rapid Spin', `[of] ${pokemon}`);
///             }
///         }
///         if (pokemon.hp && pokemon.volatiles['partiallytrapped']) {
///             pokemon.removeVolatile('partiallytrapped');
///         }
///     }
/// }
pub fn on_after_hit(_battle: &mut Battle, _source_pos: (usize, usize), _target_pos: (usize, usize)) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

/// onAfterSubDamage(damage, target, pokemon, move) {
///     if (!move.hasSheerForce) {
///         if (pokemon.hp && pokemon.removeVolatile('leechseed')) {
///             this.add('-end', pokemon, 'Leech Seed', '[from] move: Rapid Spin', `[of] ${pokemon}`);
///         }
///         const sideConditions = ['spikes', 'toxicspikes', 'stealthrock', 'stickyweb', 'gmaxsteelsurge'];
///         for (const condition of sideConditions) {
///             if (pokemon.hp && pokemon.side.removeSideCondition(condition)) {
///                 this.add('-sideend', pokemon.side, this.dex.conditions.get(condition).name, '[from] move: Rapid Spin', `[of] ${pokemon}`);
///             }
///         }
///         if (pokemon.hp && pokemon.volatiles['partiallytrapped']) {
///             pokemon.removeVolatile('partiallytrapped');
///         }
///     }
/// }
pub fn on_after_sub_damage(_battle: &mut Battle, _damage: i32, _target_pos: Option<(usize, usize)>, _pokemon_pos: (usize, usize), _move_id: &str) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

