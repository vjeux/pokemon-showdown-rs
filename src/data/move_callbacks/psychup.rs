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
pub fn on_hit(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    use crate::dex_data::ID;

    let source = pokemon_pos;
    let target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // let i: BoostID;
    // for (i in target.boosts) {
    //     source.boosts[i] = target.boosts[i];
    // }
    let target_boosts = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target_pokemon.boosts.clone()
    };

    let source_pokemon = match battle.pokemon_at_mut(source.0, source.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };
    source_pokemon.boosts = target_boosts;

    // const volatilesToCopy = ['dragoncheer', 'focusenergy', 'gmaxchistrike', 'laserfocus'];
    // we need to remove all crit stage volatiles first; otherwise copying e.g. dragoncheer onto a mon with focusenergy
    // will crash the server (since addVolatile fails due to overlap, leaving the source mon with no hasDragonType to set)
    // for (const volatile of volatilesToCopy) source.removeVolatile(volatile);
    let volatiles_to_copy = vec![
        ID::from("dragoncheer"),
        ID::from("focusenergy"),
        ID::from("gmaxchistrike"),
        ID::from("laserfocus"),
    ];

    for volatile in &volatiles_to_copy {
        battle.remove_volatile(volatile, source);
    }

    // for (const volatile of volatilesToCopy) {
    //     if (target.volatiles[volatile]) {
    //         source.addVolatile(volatile);
    //         if (volatile === 'gmaxchistrike') source.volatiles[volatile].layers = target.volatiles[volatile].layers;
    //         if (volatile === 'dragoncheer') source.volatiles[volatile].hasDragonType = target.volatiles[volatile].hasDragonType;
    //     }
    // }
    for volatile in &volatiles_to_copy {
        let has_volatile = {
            let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            target_pokemon.volatiles.contains_key(volatile)
        };

        if has_volatile {
            battle.add_volatile(volatile, source, None, None);

            if volatile == &ID::from("gmaxchistrike") {
                let layers = {
                    let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                        Some(p) => p,
                        None => return EventResult::Continue,
                    };
                    target_pokemon.volatiles.get(volatile)
                        .and_then(|v| v.layers)
                        .unwrap_or(0)
                };

                let source_pokemon = match battle.pokemon_at_mut(source.0, source.1) {
                    Some(p) => p,
                    None => return EventResult::Continue,
                };
                if let Some(volatile_effect) = source_pokemon.volatiles.get_mut(volatile) {
                    volatile_effect.layers = Some(layers);
                }
            }

            if volatile == &ID::from("dragoncheer") {
                let has_dragon_type = {
                    let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                        Some(p) => p,
                        None => return EventResult::Continue,
                    };
                    target_pokemon.volatiles.get(volatile)
                        .and_then(|v| v.has_dragon_type)
                        .unwrap_or(false)
                };

                let source_pokemon = match battle.pokemon_at_mut(source.0, source.1) {
                    Some(p) => p,
                    None => return EventResult::Continue,
                };
                if let Some(volatile_effect) = source_pokemon.volatiles.get_mut(volatile) {
                    volatile_effect.has_dragon_type = Some(has_dragon_type);
                }
            }
        }
    }

    // this.add('-copyboost', source, target, '[from] move: Psych Up');
    let (source_arg, target_arg) = {
        let source_pokemon = match battle.pokemon_at(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        (crate::battle::Arg::from(source_pokemon), crate::battle::Arg::from(target_pokemon))
    };

    battle.add("-copyboost", &[
        source_arg,
        target_arg,
        "[from] move: Psych Up".into(),
    ]);

    EventResult::Continue
}

