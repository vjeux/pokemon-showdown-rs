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
pub fn on_start(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    use crate::battle::Arg;
    use crate::Pokemon;
    use crate::dex_data::ID;

    // const ally = pokemon.allies()[0];
    let allies = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.allies(battle, false)
    };

    // if (!ally) return;
    let ally_pos = match allies.get(0) {
        Some(&pos) => pos,
        None => return EventResult::Continue,
    };

    // for (i in ally.boosts) { pokemon.boosts[i] = ally.boosts[i]; }
    let ally_boosts = {
        let ally = match battle.pokemon_at(ally_pos.0, ally_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        ally.boosts.clone()
    };

    {
        let pokemon = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.boosts = ally_boosts;
    }

    // const volatilesToCopy = ['dragoncheer', 'focusenergy', 'gmaxchistrike', 'laserfocus'];
    let volatiles_to_copy = vec![
        ID::from("dragoncheer"),
        ID::from("focusenergy"),
        ID::from("gmaxchistrike"),
        ID::from("laserfocus"),
    ];

    // for (const volatile of volatilesToCopy) pokemon.removeVolatile(volatile);
    for volatile_id in &volatiles_to_copy {
        Pokemon::remove_volatile(battle, pokemon_pos, volatile_id);
    }

    // for (const volatile of volatilesToCopy) { if (ally.volatiles[volatile]) { ... } }
    for volatile_id in &volatiles_to_copy {
        let ally_has_volatile = {
            let ally = match battle.pokemon_at(ally_pos.0, ally_pos.1) {
                Some(p) => p,
                None => continue,
            };
            ally.has_volatile(volatile_id)
        };

        if ally_has_volatile {
            // pokemon.addVolatile(volatile);
            Pokemon::add_volatile(battle, pokemon_pos, volatile_id.clone(), None, None, None);

            // Note: Copying layers and hasDragonType properties requires accessing volatile state data
            // This would need EffectState.data HashMap access which is complex
            // For now, we're adding the volatile without copying those extra properties
            // TODO: Implement layers and hasDragonType copying when volatile state structure allows
        }
    }

    // this.add('-copyboost', pokemon, ally, '[from] ability: Costar');
    let (pokemon_slot, ally_slot) = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        let ally = match battle.pokemon_at(ally_pos.0, ally_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        (pokemon.get_slot(), ally.get_slot())
    };

    battle.add("-copyboost", &[
        Arg::String(pokemon_slot),
        Arg::String(ally_slot),
        Arg::Str("[from] ability: Costar"),
    ]);

    EventResult::Continue
}

