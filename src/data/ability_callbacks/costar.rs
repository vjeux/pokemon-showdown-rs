//! Costar Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onStart(pokemon) {
///     const ally = pokemon.allies()[0];
///     if (!ally) return;
/// 
///     let i: BoostID;
///     for (i in ally.boosts) {
///         pokemon.boosts[i] = ally.boosts[i];
///     }
///     const volatilesToCopy = ['dragoncheer', 'focusenergy', 'gmaxchistrike', 'laserfocus'];
///     // we need to be sure to remove all the overlapping crit volatiles before trying to add any
///     for (const volatile of volatilesToCopy) pokemon.removeVolatile(volatile);
///     for (const volatile of volatilesToCopy) {
///         if (ally.volatiles[volatile]) {
///             pokemon.addVolatile(volatile);
///             if (volatile === 'gmaxchistrike') pokemon.volatiles[volatile].layers = ally.volatiles[volatile].layers;
///             if (volatile === 'dragoncheer') pokemon.volatiles[volatile].hasDragonType = ally.volatiles[volatile].hasDragonType;
///         }
///     }
///     this.add('-copyboost', pokemon, ally, '[from] ability: Costar');
/// }
pub fn on_start(_battle: &mut Battle, _pokemon_pos: (usize, usize)) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

