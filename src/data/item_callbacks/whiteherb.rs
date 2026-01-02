//! White Herb Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::event::EventResult;
use crate::Pokemon;

/// onStart(pokemon) {
///     this.effectState.boosts = {} as SparseBoostsTable;
///     let ready = false;
///     let i: BoostID;
///     for (i in pokemon.boosts) {
///         if (pokemon.boosts[i] < 0) {
///             ready = true;
///             this.effectState.boosts[i] = 0;
///         }
///     }
///     if (ready) (this.effectState.target as Pokemon).useItem();
///     delete this.effectState.boosts;
/// }
pub fn on_start(battle: &mut Battle, target_pos: Option<(usize, usize)>) -> EventResult {
    // if (!pokemon) return;
    let pokemon_pos = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // this.effectState.boosts = {} as SparseBoostsTable;
    // let ready = false;
    // for (i in pokemon.boosts) { if (pokemon.boosts[i] < 0) { ready = true; this.effectState.boosts[i] = 0; } }
    let (boosts_to_clear, ready) = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        use crate::dex_data::BoostID;
        let mut boosts_to_clear = std::collections::HashMap::new();
        let mut ready = false;

        for boost_id in BoostID::all() {
            let boost_value = pokemon.boosts.get(*boost_id);
            if boost_value < 0 {
                ready = true;
                boosts_to_clear.insert(format!("{:?}", boost_id).to_lowercase(), 0i8);
            }
        }

        (boosts_to_clear, ready)
    };

    // Store boosts in effectState.data
    if let Some(ref mut effect_state) = battle.current_effect_state {
        effect_state
            .data
            .insert("boosts".to_string(), serde_json::json!(boosts_to_clear));
    }

    // if (ready) (this.effectState.target as Pokemon).useItem();
    if ready {
        let _pokemon_mut = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        Pokemon::use_item(battle, pokemon_pos, None, None);
    }

    // delete this.effectState.boosts;
    if let Some(ref mut effect_state) = battle.current_effect_state {
        effect_state.data.remove("boosts");
    }

    EventResult::Continue
}

/// onAnySwitchIn() {
///     ((this.effect as any).onStart as (p: Pokemon) => void).call(this, this.effectState.target);
/// }
pub fn on_any_switch_in(battle: &mut Battle) -> EventResult {
    // Get effectState.target
    let target_pos = {
        if let Some(ref effect_state) = battle.current_effect_state {
            effect_state.target
        } else {
            return EventResult::Continue;
        }
    };

    // Call onStart with effectState.target
    on_start(battle, target_pos)
}

/// onAnyAfterMega() {
///     ((this.effect as any).onStart as (p: Pokemon) => void).call(this, this.effectState.target);
/// }
pub fn on_any_after_mega(battle: &mut Battle) -> EventResult {
    // Get effectState.target
    let target_pos = {
        if let Some(ref effect_state) = battle.current_effect_state {
            effect_state.target
        } else {
            return EventResult::Continue;
        }
    };

    // Call onStart with effectState.target
    on_start(battle, target_pos)
}

/// onAnyAfterMove() {
///     ((this.effect as any).onStart as (p: Pokemon) => void).call(this, this.effectState.target);
/// }
pub fn on_any_after_move(battle: &mut Battle) -> EventResult {
    // Get effectState.target
    let target_pos = {
        if let Some(ref effect_state) = battle.current_effect_state {
            effect_state.target
        } else {
            return EventResult::Continue;
        }
    };

    // Call onStart with effectState.target
    on_start(battle, target_pos)
}

/// onResidual(pokemon) {
///     ((this.effect as any).onStart as (p: Pokemon) => void).call(this, pokemon);
/// }
pub fn on_residual(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // Call onStart with pokemon parameter
    on_start(battle, Some(pokemon_pos))
}

/// onUse(pokemon) {
///     pokemon.setBoost(this.effectState.boosts);
///     this.add('-clearnegativeboost', pokemon, '[silent]');
/// }
pub fn on_use(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // pokemon.setBoost(this.effectState.boosts);
    let boosts_map = {
        if let Some(ref effect_state) = battle.current_effect_state {
            if let Some(boosts_value) = effect_state.data.get("boosts") {
                if let Some(obj) = boosts_value.as_object() {
                    let mut map = std::collections::HashMap::new();
                    for (key, value) in obj {
                        if let Some(num) = value.as_i64() {
                            map.insert(key.clone(), num as i8);
                        }
                    }
                    Some(map)
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    };

    if let Some(boosts) = boosts_map {
        let pokemon_mut = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        use crate::dex_data::BoostID;
        let mut boost_map = std::collections::HashMap::new();
        for (boost_name, boost_value) in boosts {
            let boost_id = match boost_name.to_lowercase().as_str() {
                "atk" => BoostID::Atk,
                "def" => BoostID::Def,
                "spa" => BoostID::SpA,
                "spd" => BoostID::SpD,
                "spe" => BoostID::Spe,
                "accuracy" => BoostID::Accuracy,
                "evasion" => BoostID::Evasion,
                _ => continue,
            };
            boost_map.insert(boost_id, boost_value);
        }
        pokemon_mut.set_boost(boost_map);
    }

    // this.add('-clearnegativeboost', pokemon, '[silent]');
    let pokemon_slot = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
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

    EventResult::Continue
}
