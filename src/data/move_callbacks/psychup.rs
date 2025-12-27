//! Psych Up Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onHit(target, source) {
///     let i: BoostID;
///     for (i in target.boosts) {
///         source.boosts[i] = target.boosts[i];
///     }
/// 
///     const volatilesToCopy = ['dragoncheer', 'focusenergy', 'gmaxchistrike', 'laserfocus'];
///     // we need to remove all crit stage volatiles first; otherwise copying e.g. dragoncheer onto a mon with focusenergy
///     // will crash the server (since addVolatile fails due to overlap, leaving the source mon with no hasDragonType to set)
///     for (const volatile of volatilesToCopy) source.removeVolatile(volatile);
///     for (const volatile of volatilesToCopy) {
///         if (target.volatiles[volatile]) {
///             source.addVolatile(volatile);
///             if (volatile === 'gmaxchistrike') source.volatiles[volatile].layers = target.volatiles[volatile].layers;
///             if (volatile === 'dragoncheer') source.volatiles[volatile].hasDragonType = target.volatiles[volatile].hasDragonType;
///         }
///     }
///     this.add('-copyboost', source, target, '[from] move: Psych Up');
/// }
pub fn on_hit(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>, move_id: Option<&str>) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

