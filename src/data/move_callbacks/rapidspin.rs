//! Rapid Spin Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{MoveHandlerResult, Status, Effect};

/// onAfterHit(target, pokemon, move) {            if (!move.hasSheerForce) {
///                 if (pokemon.hp && pokemon.removeVolatile('leechseed')) {
///                     this.add('-end', pokemon, 'Leech Seed', '[from] move: Rapid Spin', `[of] ${pokemon}`);
///                 }
///                 const sideConditions = ['spikes', 'toxicspikes', 'stealthrock', 'stickyweb', 'gmaxsteelsurge'];
///                 for (const condition of sideConditions) {
///                     if (pokemon.hp && pokemon.side.removeSideCondition(condition)) {
///                         this.add('-sideend', pokemon.side, this.dex.conditions.get(condition).name, '[from] move: Rapid Spin', `[of] ${pokemon}`);
///                     }
///                 }
///                 if (pokemon.hp && pokemon.volatiles['partiallytrapped']) {
///                     pokemon.removeVolatile('partiallytrapped');
///                 }
///             }
///         }
pub fn on_after_hit(battle: &mut Battle, /* TODO: Add parameters */) -> MoveHandlerResult {
    // TODO: Implement 1-to-1 from JS
    MoveHandlerResult::Undefined
}

/// onAfterSubDamage(damage, target, pokemon, move) {            if (!move.hasSheerForce) {
///                 if (pokemon.hp && pokemon.removeVolatile('leechseed')) {
///                     this.add('-end', pokemon, 'Leech Seed', '[from] move: Rapid Spin', `[of] ${pokemon}`);
///                 }
///                 const sideConditions = ['spikes', 'toxicspikes', 'stealthrock', 'stickyweb', 'gmaxsteelsurge'];
///                 for (const condition of sideConditions) {
///                     if (pokemon.hp && pokemon.side.removeSideCondition(condition)) {
///                         this.add('-sideend', pokemon.side, this.dex.conditions.get(condition).name, '[from] move: Rapid Spin', `[of] ${pokemon}`);
///                     }
///                 }
///                 if (pokemon.hp && pokemon.volatiles['partiallytrapped']) {
///                     pokemon.removeVolatile('partiallytrapped');
///                 }
///             }
///         }
pub fn on_after_sub_damage(battle: &mut Battle, /* TODO: Add parameters */) -> MoveHandlerResult {
    // TODO: Implement 1-to-1 from JS
    MoveHandlerResult::Undefined
}

