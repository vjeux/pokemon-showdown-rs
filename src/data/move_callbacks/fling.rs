//! Fling Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::{Battle, Effect};
use crate::dex_data::ID;
use crate::event::EventResult;
use crate::Pokemon;

/// Dispatch to item-specific fling effect callbacks
/// Each item with a fling.effect in JavaScript has its own implementation here
fn dispatch_fling_effect(battle: &mut Battle, item_id: &ID, target: (usize, usize)) {
    debug_elog!("[FLING_EFFECT] Dispatching fling effect for item: {}", item_id.as_str());
    match item_id.as_str() {
        "whiteherb" => {
            // White Herb's fling.effect: clear negative boosts
            // JavaScript:
            // effect(pokemon) {
            //     let activate = false;
            //     const boosts: SparseBoostsTable = {};
            //     for (i in pokemon.boosts) {
            //         if (pokemon.boosts[i] < 0) {
            //             activate = true;
            //             boosts[i] = 0;
            //         }
            //     }
            //     if (activate) {
            //         pokemon.setBoost(boosts);
            //         this.add('-clearnegativeboost', pokemon, '[silent]');
            //     }
            // }
            use crate::dex_data::BoostID;

            let (activate, boosts_to_clear) = {
                let pokemon = match battle.pokemon_at(target.0, target.1) {
                    Some(p) => p,
                    None => return,
                };

                let mut activate = false;
                let mut boosts_map = std::collections::HashMap::new();

                for boost_id in BoostID::all() {
                    if pokemon.boosts.get(*boost_id) < 0 {
                        activate = true;
                        boosts_map.insert(*boost_id, 0);
                    }
                }

                (activate, boosts_map)
            };

            if activate {
                // pokemon.setBoost(boosts);
                if let Some(pokemon) = battle.pokemon_at_mut(target.0, target.1) {
                    pokemon.set_boost(boosts_to_clear);
                }

                // this.add('-clearnegativeboost', pokemon, '[silent]');
                let pokemon_slot = {
                    let pokemon = match battle.pokemon_at(target.0, target.1) {
                        Some(p) => p,
                        None => return,
                    };
                    pokemon.get_slot()
                };

                battle.add(
                    "-clearnegativeboost",
                    &[
                        crate::battle::Arg::from(pokemon_slot),
                        crate::battle::Arg::from("[silent]"),
                    ],
                );
            }
        }
        // TODO: Add other items with fling.effect as needed
        // Common ones: Mental Herb (cure infatuation/taunt/etc),
        // Luminous Moss (+1 SpD if hit by Water), etc.
        _ => {
            // Unknown fling effect - log for debugging
            debug_elog!("[FLING] Unknown fling.effect for item: {}", item_id.as_str());
        }
    }
}

/// onPrepareHit(target, source, move) {
///     if (source.ignoringItem(true)) return false;
///     const item = source.getItem();
///     if (!this.singleEvent('TakeItem', item, source.itemState, source, source, move, item)) return false;
///     if (!item.fling) return false;
///     move.basePower = item.fling.basePower;
///     this.debug(`BP: ${move.basePower}`);
///     if (item.isBerry) {
///         if (source.hasAbility('cudchew')) {
///             this.singleEvent('EatItem', source.getAbility(), source.abilityState, source, source, move, item);
///         }
///         move.onHit = function (foe) {
///             if (this.singleEvent('Eat', item, source.itemState, foe, source, move)) {
///                 this.runEvent('EatItem', foe, source, move, item);
///                 if (item.id === 'leppaberry') foe.staleness = 'external';
///             }
///             if (item.onEat) foe.ateBerry = true;
///         };
///     } else if (item.fling.effect) {
///         move.onHit = item.fling.effect;
///     } else {
///         if (!move.secondaries) move.secondaries = [];
///         if (item.fling.status) {
///             move.secondaries.push({ status: item.fling.status });
///         } else if (item.fling.volatileStatus) {
///             move.secondaries.push({ volatileStatus: item.fling.volatileStatus });
///         }
///     }
///     source.addVolatile('fling');
/// }
pub fn on_prepare_hit(
    battle: &mut Battle,
    _target_pos: (usize, usize),           // target (not used by fling)
    source_pos: Option<(usize, usize)>,     // source - the attacker using fling
) -> EventResult {
    // JavaScript uses "source" to get the item, not "target"
    let pokemon = match source_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // if (source.ignoringItem(true)) return false;
    let ignoring_item = {
        let pokemon_ref = match battle.pokemon_at(pokemon.0, pokemon.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon_ref.ignoring_item(battle, true)
    };

    if ignoring_item {
        return EventResult::Boolean(false);
    }

    // const item = source.getItem();
    let item_id = {
        let pokemon_ref = match battle.pokemon_at(pokemon.0, pokemon.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon_ref.get_item().clone()
    };

    // if (!this.singleEvent('TakeItem', item, source.itemState, source, source, move, item)) return false;
    let take_item_result = battle.single_event(
        "TakeItem",
        &crate::battle::Effect::item(item_id.clone()),
        None,
        Some(pokemon),
        Some(pokemon),
        Some(&Effect::move_(ID::new("fling"))),
        None,
    );
    if take_item_result.boolean() == Some(false) {
        return EventResult::Boolean(false);
    }

    // if (!item.fling) return false;
    let fling_data = {
        battle.dex.items().get_by_id(&item_id)
            .and_then(|item| item.fling.clone())
    };

    let fling = match fling_data {
        Some(f) => f,
        None => return EventResult::Boolean(false),
    };

    // move.basePower = item.fling.basePower;
    if let Some(ref mut active_move) = battle.active_move {
        active_move.base_power = fling.base_power;
    }

    // this.debug(`BP: ${move.basePower}`);
    battle.debug(&format!("BP: {}", fling.base_power));

    // if (item.isBerry) {
    //     if (source.hasAbility('cudchew')) {
    //         this.singleEvent('EatItem', source.getAbility(), source.abilityState, source, source, move, item);
    //     }
    //     move.onHit = function (foe) { ... };
    // } else if (item.fling.effect) {
    //     move.onHit = item.fling.effect;
    // } else {
    //     ... secondaries logic ...
    // }
    //
    // Store the item ID in the active_move so on_hit can access it
    // (battle.effect gets overwritten, but active_move persists)
    if let Some(ref mut active_move) = battle.active_move {
        active_move.flung_item = Some(item_id.clone());
    }

    // Get item.isBerry to handle berry case
    let is_berry = {
        battle.dex.items().get_by_id(&item_id)
            .map(|item| item.is_berry)
            .unwrap_or(false)
    };

    // if (item.isBerry) {
    if is_berry {
        // if (source.hasAbility('cudchew')) {
        //     this.singleEvent('EatItem', source.getAbility(), source.abilityState, source, source, move, item);
        // }
        let has_cudchew = {
            let pokemon_ref = match battle.pokemon_at(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon_ref.has_ability(battle, &["cudchew"])
        };

        if has_cudchew {
            let ability_id = {
                let pokemon_ref = match battle.pokemon_at(pokemon.0, pokemon.1) {
                    Some(p) => p,
                    None => return EventResult::Continue,
                };
                pokemon_ref.ability.clone()
            };

            battle.single_event(
                "EatItem",
                &crate::battle::Effect::ability(ability_id),
                None,
                Some(pokemon),
                Some(pokemon),
                Some(&Effect::item(item_id.clone())),
                None,
            );
        }

        // The berry eating logic will be handled in on_hit callback
    }
    // else if (item.fling.effect) {
    //     move.onHit = item.fling.effect;
    // }
    // The fling effect logic will be handled in on_hit callback via item.fling.effect
    else if fling.effect.is_none() {
        // else case - only add secondaries if no berry and no effect
        if let Some(ref mut active_move) = battle.active_move {
            // if (!move.secondaries) move.secondaries = [];
            // (secondaries already exists as Vec, so we just push to it)

            // if (item.fling.status) { move.secondaries.push({ status: item.fling.status }); }
            if let Some(status) = fling.status {
                active_move.secondaries.push(crate::dex::MoveSecondary {
                    status: Some(status),
                    ..Default::default()
                });
            }
            // else if (item.fling.volatileStatus) { move.secondaries.push({ volatileStatus: item.fling.volatileStatus }); }
            else if let Some(volatile_status) = fling.volatile_status {
                active_move.secondaries.push(crate::dex::MoveSecondary {
                    volatile_status: Some(volatile_status),
                    ..Default::default()
                });
            }
        }
    }

    // source.addVolatile('fling');
    Pokemon::add_volatile(battle, pokemon, ID::from("fling"), None, None, None, None);

    EventResult::Continue
}

/// onHit(target, source) {
///     // Handle berry or fling effect logic
///     const item = source.lastItem;
///     if (item.isBerry) {
///         if (this.singleEvent('Eat', item, source.itemState, target, source, move)) {
///             this.runEvent('EatItem', target, source, move, item);
///             if (item.id === 'leppaberry') target.staleness = 'external';
///         }
///         if (item.onEat) target.ateBerry = true;
///     }
/// }
pub fn on_hit(
    battle: &mut Battle,
    target_pos: (usize, usize),
    source_pos: Option<(usize, usize)>,
) -> EventResult {
    debug_elog!("[FLING_ON_HIT] Called with target={:?}, source={:?}", target_pos, source_pos);
    let target = target_pos;

    let source = match source_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // Get the flung item ID from active_move.flung_item (set in on_prepare_hit)
    let item_id = match &battle.active_move {
        Some(active_move) => match &active_move.flung_item {
            Some(id) => {
                debug_elog!("[FLING_ON_HIT] Flung item: {}", id.as_str());
                id.clone()
            },
            None => {
                debug_elog!("[FLING_ON_HIT] No flung_item in active_move");
                return EventResult::Continue;
            },
        },
        None => {
            debug_elog!("[FLING_ON_HIT] No active_move");
            return EventResult::Continue;
        },
    };

    let item_effect = Effect::item(item_id.clone());

    // Get item data
    let (is_berry, has_on_eat) = {
        let item_data = match battle.dex.items().get_by_id(&item_id) {
            Some(data) => data,
            None => return EventResult::Continue,
        };
        // JavaScript checks `if (item.onEat)` which is truthy check
        // `onEat: false` is falsy, so we need to check the value, not just key existence
        let on_eat_is_truthy = item_data.extra.get("onEat").map_or(false, |v| {
            !v.is_null() && v != &serde_json::Value::Bool(false)
        });
        (item_data.is_berry, on_eat_is_truthy)
    };

    // if (item.isBerry) {
    if is_berry {
        // if (this.singleEvent('Eat', item, source.itemState, target, source, move)) {
        let eat_result = battle.single_event(
            "Eat",
            &item_effect,
            None,
            Some(target),
            Some(source),
            Some(&item_effect),
            None,
        );

        if eat_result.boolean() != Some(false) {
            // this.runEvent('EatItem', target, source, move, item);
            battle.run_event("EatItem", Some(crate::event::EventTarget::Pokemon(target)), Some(source), Some(&item_effect), EventResult::Continue, false, false);

            // if (item.id === 'leppaberry') target.staleness = 'external';
            if item_id.as_str() == "leppaberry" {
                if let Some(target_pokemon) = battle.pokemon_at_mut(target.0, target.1) {
                    target_pokemon.staleness = Some("external".to_string());
                }
            }
        }

        // if (item.onEat) target.ateBerry = true;
        if has_on_eat {
            if let Some(target_pokemon) = battle.pokemon_at_mut(target.0, target.1) {
                target_pokemon.ate_berry = true;
            }
        }
    } else {
        // else if (item.fling.effect) { move.onHit = item.fling.effect; }
        // Note: In JavaScript, fling.effect is a function, which doesn't serialize to JSON.
        // We dispatch to hardcoded implementations based on item ID instead.
        debug_elog!("[FLING_ON_HIT] Item is not a berry, dispatching fling effect");
        dispatch_fling_effect(battle, &item_id, target);
    }

    EventResult::Continue
}

pub mod condition {
    use super::*;

    /// onUpdate(pokemon) {
    ///     const item = pokemon.getItem();
    ///     pokemon.setItem('');
    ///     pokemon.lastItem = item.id;
    ///     pokemon.usedItemThisTurn = true;
    ///     this.add('-enditem', pokemon, item.name, '[from] move: Fling');
    ///     this.runEvent('AfterUseItem', pokemon, null, null, item);
    ///     pokemon.removeVolatile('fling');
    /// }
    pub fn on_update(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
        use crate::dex_data::ID;

        let pokemon = pokemon_pos;

        // const item = pokemon.getItem();
        let item_id = {
            let pokemon_ref = match battle.pokemon_at(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon_ref.get_item().clone()
        };

        // If no item, nothing to do
        if item_id.is_empty() {
            return EventResult::Continue;
        }

        // Get item name for the battle log
        let item_name = {
            let item_data = battle.dex.items().get_by_id(&item_id);
            item_data
                .map(|i| i.name.clone())
                .unwrap_or_else(|| item_id.to_string())
        };

        // Get pokemon slot for battle log
        let pokemon_slot = {
            let pokemon_ref = match battle.pokemon_at(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon_ref.get_slot()
        };

        // pokemon.setItem('');
        // pokemon.lastItem = item.id;
        // pokemon.usedItemThisTurn = true;
        Pokemon::set_item(battle, pokemon, ID::empty(), None, None);
        {
            let pokemon_mut = match battle.pokemon_at_mut(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon_mut.last_item = item_id.clone();
            pokemon_mut.used_item_this_turn = true;
        }

        // this.add('-enditem', pokemon, item.name, '[from] move: Fling');
        battle.add(
            "-enditem",
            &[
                crate::battle::Arg::from(pokemon_slot),
                crate::battle::Arg::from(item_name),
                crate::battle::Arg::from("[from] move: Fling"),
            ],
        );

        // this.runEvent('AfterUseItem', pokemon, null, null, item);
        battle.run_event("AfterUseItem", Some(crate::event::EventTarget::Pokemon(pokemon)), None, Some(&Effect::item(item_id.clone())), EventResult::Continue, false, false);

        // pokemon.removeVolatile('fling');
        Pokemon::remove_volatile(battle, pokemon, &ID::from("fling"));

        EventResult::Continue
    }
}
