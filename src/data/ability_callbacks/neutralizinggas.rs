//! Neutralizing Gas Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onSwitchIn(pokemon) {
///     this.add('-ability', pokemon, 'Neutralizing Gas');
///     pokemon.abilityState.ending = false;
///     const strongWeathers = ['desolateland', 'primordialsea', 'deltastream'];
///     for (const target of this.getAllActive()) {
///         if (target.hasItem('Ability Shield')) {
///             this.add('-block', target, 'item: Ability Shield');
///             continue;
///         }
///         // Can't suppress a Tatsugiri inside of Dondozo already
///         if (target.volatiles['commanding']) {
///             continue;
///         }
///         if (target.illusion) {
///             this.singleEvent('End', this.dex.abilities.get('Illusion'), target.abilityState, target, pokemon, 'neutralizinggas');
///         }
///         if (target.volatiles['slowstart']) {
///             delete target.volatiles['slowstart'];
///             this.add('-end', target, 'Slow Start', '[silent]');
///         }
///         if (strongWeathers.includes(target.getAbility().id)) {
///             this.singleEvent('End', this.dex.abilities.get(target.getAbility().id), target.abilityState, target, pokemon, 'neutralizinggas');
///         }
///     }
/// }
pub fn on_switch_in(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    use crate::dex_data::ID;
    use crate::pokemon::Pokemon;

    // this.add('-ability', pokemon, 'Neutralizing Gas');
    let pokemon_slot = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.get_slot()
    };
    battle.add("-ability", &[pokemon_slot.into(), "Neutralizing Gas".into()]);

    // pokemon.abilityState.ending = false;
    {
        let pokemon = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.ability_state.data.insert("ending".to_string(), serde_json::json!(false));
    }

    // const strongWeathers = ['desolateland', 'primordialsea', 'deltastream'];
    let strong_weathers = ["desolateland", "primordialsea", "deltastream"];

    // for (const target of this.getAllActive()) {
    let active_positions: Vec<(usize, usize)> = battle.get_all_active(false).iter().map(|p| (p.0, p.1)).collect();
    for target_pos in active_positions {
        // if (target.hasItem('Ability Shield')) {
        let has_ability_shield = {
            let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
                Some(p) => p,
                None => continue,
            };
            target.has_item(battle, &["abilityshield"])
        };

        if has_ability_shield {
            // this.add('-block', target, 'item: Ability Shield');
            let target_slot = {
                let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
                    Some(p) => p,
                    None => continue,
                };
                target.get_slot()
            };
            battle.add("-block", &[target_slot.into(), "item: Ability Shield".into()]);
            continue;
        }

        // if (target.volatiles['commanding']) {
        let has_commanding = {
            let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
                Some(p) => p,
                None => continue,
            };
            target.has_volatile(&ID::from("commanding"))
        };

        if has_commanding {
            continue;
        }

        // if (target.illusion) {
        let has_illusion = {
            let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
                Some(p) => p,
                None => continue,
            };
            target.illusion.is_some()
        };

        if has_illusion {
            // this.singleEvent('End', this.dex.abilities.get('Illusion'), target.abilityState, target, pokemon, 'neutralizinggas');
            battle.single_event("End", &ID::from("illusion"), Some(target_pos), Some(pokemon_pos), Some(&ID::from("neutralizinggas")));
        }

        // if (target.volatiles['slowstart']) {
        let has_slowstart = {
            let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
                Some(p) => p,
                None => continue,
            };
            target.has_volatile(&ID::from("slowstart"))
        };

        if has_slowstart {
            // delete target.volatiles['slowstart'];
            Pokemon::remove_volatile(battle, target_pos, &ID::from("slowstart"));

            // this.add('-end', target, 'Slow Start', '[silent]');
            let target_slot = {
                let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
                    Some(p) => p,
                    None => continue,
                };
                target.get_slot()
            };
            battle.add("-end", &[target_slot.into(), "Slow Start".into(), "[silent]".into()]);
        }

        // if (strongWeathers.includes(target.getAbility().id)) {
        let target_ability_id = {
            let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
                Some(p) => p,
                None => continue,
            };
            target.get_ability().to_string()
        };

        if strong_weathers.contains(&target_ability_id.as_str()) {
            // this.singleEvent('End', this.dex.abilities.get(target.getAbility().id), target.abilityState, target, pokemon, 'neutralizinggas');
            battle.single_event("End", &ID::from(target_ability_id.as_str()), Some(target_pos), Some(pokemon_pos), Some(&ID::from("neutralizinggas")));
        }
    }

    EventResult::Continue
}

/// onEnd(source) {
///     if (source.transformed) return;
///     for (const pokemon of this.getAllActive()) {
///         if (pokemon !== source && pokemon.hasAbility('Neutralizing Gas')) {
///             return;
///         }
///     }
///     this.add('-end', source, 'ability: Neutralizing Gas');
///
///     // FIXME this happens before the pokemon switches out, should be the opposite order.
///     // Not an easy fix since we cant use a supported event. Would need some kind of special event that
///     // gathers events to run after the switch and then runs them when the ability is no longer accessible.
///     // (If you're tackling this, do note extreme weathers have the same issue)
///
///     // Mark this pokemon's ability as ending so Pokemon#ignoringAbility skips it
///     if (source.abilityState.ending) return;
///     source.abilityState.ending = true;
///     const sortedActive = this.getAllActive();
///     this.speedSort(sortedActive);
///     for (const pokemon of sortedActive) {
///         if (pokemon !== source) {
///             if (pokemon.getAbility().flags['cantsuppress']) continue; // does not interact with e.g Ice Face, Zen Mode
///             if (pokemon.hasItem('abilityshield')) continue; // don't restart abilities that weren't suppressed
///
///             // Will be suppressed by Pokemon#ignoringAbility if needed
///             this.singleEvent('Start', pokemon.getAbility(), pokemon.abilityState, pokemon);
///             if (pokemon.ability === "gluttony") {
///                 pokemon.abilityState.gluttony = false;
///             }
///         }
///     }
/// }
pub fn on_end(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    use crate::dex_data::ID;

    // if (source.transformed) return;
    let is_transformed = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.transformed
    };

    if is_transformed {
        return EventResult::Continue;
    }

    // for (const pokemon of this.getAllActive()) {
    //     if (pokemon !== source && pokemon.hasAbility('Neutralizing Gas')) {
    //         return;
    //     }
    // }
    let has_other_neutralizing_gas = {
        let active = battle.get_all_active(false);
        active.iter().any(|&pos| {
            if pos == pokemon_pos {
                return false;
            }
            if let Some(p) = battle.pokemon_at(pos.0, pos.1) {
                p.has_ability(battle, &["neutralizinggas"])
            } else {
                false
            }
        })
    };

    if has_other_neutralizing_gas {
        return EventResult::Continue;
    }

    // this.add('-end', source, 'ability: Neutralizing Gas');
    let pokemon_slot = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.get_slot()
    };
    battle.add("-end", &[pokemon_slot.into(), "ability: Neutralizing Gas".into()]);

    // if (source.abilityState.ending) return;
    let is_ending = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.ability_state.data.get("ending")
            .and_then(|v| v.as_bool())
            .unwrap_or(false)
    };

    if is_ending {
        return EventResult::Continue;
    }

    // source.abilityState.ending = true;
    {
        let pokemon = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.ability_state.data.insert("ending".to_string(), serde_json::json!(true));
    }

    // const sortedActive = this.getAllActive();
    // this.speedSort(sortedActive);
    let mut sorted_active: Vec<(usize, usize)> = battle.get_all_active(false).iter().map(|p| (p.0, p.1)).collect();

    // Extract speeds before sorting to avoid borrow checker issues
    let speeds: Vec<((usize, usize), f64)> = sorted_active.iter().map(|&pos| {
        use crate::dex_data::StatID;
        let speed = battle.get_pokemon_stat(pos, StatID::Spe, false, false) as f64;
        (pos, speed)
    }).collect();

    // Create a HashMap for quick speed lookup in the closure
    let speed_map: std::collections::HashMap<(usize, usize), f64> = speeds.into_iter().collect();

    // Speed sort using pre-calculated speeds
    battle.speed_sort(&mut sorted_active, |&pos| {
        use crate::battle::PriorityItem;

        let speed = speed_map.get(&pos).copied().unwrap_or(0.0);

        PriorityItem {
            order: None,
            priority: 0,
            speed,
            sub_order: 0,
            effect_order: 0,
            index: 0,
        }
    });

    // for (const pokemon of sortedActive) {
    for target_pos in sorted_active {
        // if (pokemon !== source) {
        if target_pos == pokemon_pos {
            continue;
        }

        // if (pokemon.getAbility().flags['cantsuppress']) continue;
        let has_cantsuppress = {
            let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
                Some(p) => p,
                None => continue,
            };
            let ability_id = target.get_ability();
            if let Some(ability_data) = battle.dex.abilities().get(ability_id.as_str()) {
                if let Some(&flag) = ability_data.flags.get("cantsuppress") {
                    flag != 0
                } else {
                    false
                }
            } else {
                false
            }
        };

        if has_cantsuppress {
            continue;
        }

        // if (pokemon.hasItem('abilityshield')) continue;
        let has_ability_shield = {
            let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
                Some(p) => p,
                None => continue,
            };
            target.has_item(battle, &["abilityshield"])
        };

        if has_ability_shield {
            continue;
        }

        // this.singleEvent('Start', pokemon.getAbility(), pokemon.abilityState, pokemon);
        let target_ability_id = {
            let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
                Some(p) => p,
                None => continue,
            };
            target.get_ability().to_string()
        };
        battle.single_event("Start", &ID::from(target_ability_id.as_str()), Some(target_pos), None, None);

        // if (pokemon.ability === "gluttony") {
        //     pokemon.abilityState.gluttony = false;
        // }
        let is_gluttony = {
            let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
                Some(p) => p,
                None => continue,
            };
            target.ability == ID::from("gluttony")
        };

        if is_gluttony {
            let target = match battle.pokemon_at_mut(target_pos.0, target_pos.1) {
                Some(p) => p,
                None => continue,
            };
            target.ability_state.data.insert("gluttony".to_string(), serde_json::json!(false));
        }
    }

    EventResult::Continue
}

