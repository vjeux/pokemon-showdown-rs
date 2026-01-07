//! Mirror Herb Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::event::EventResult;
use crate::Pokemon;

/// onFoeAfterBoost(boost, target, source, effect) {
///     if (effect?.name === 'Opportunist' || effect?.name === 'Mirror Herb') return;
///     if (!this.effectState.boosts) this.effectState.boosts = {} as SparseBoostsTable;
///     const boostPlus = this.effectState.boosts;
///     let i: BoostID;
///     for (i in boost) {
///         if (boost[i]! > 0) {
///             boostPlus[i] = (boostPlus[i] || 0) + boost[i]!;
///             this.effectState.ready = true;
///         }
///     }
/// }
pub fn on_foe_after_boost(
    battle: &mut Battle,
    _target_pos: Option<(usize, usize)>,
    _source_pos: Option<(usize, usize)>,
    effect_id: Option<&str>,
    boost: &crate::dex_data::BoostsTable,
) -> EventResult {
    // if (effect?.name === 'Opportunist' || effect?.name === 'Mirror Herb') return;
    if let Some(eff_id) = effect_id {
        if eff_id == "Opportunist" || eff_id == "Mirror Herb" {
            return EventResult::Continue;
        }
    }

    // Get current boosts from effectState or create empty HashMap
    let mut boosts_map = battle.with_effect_state_ref(|state| {
        if let Some(boosts_value) = state.data.get("boosts") {
            if let Some(obj) = boosts_value.as_object() {
                let mut map = std::collections::HashMap::new();
                for (key, value) in obj {
                    if let Some(num) = value.as_i64() {
                        map.insert(key.clone(), num as i8);
                    }
                }
                map
            } else {
                std::collections::HashMap::new()
            }
        } else {
            std::collections::HashMap::new()
        }
    }).unwrap_or_else(|| std::collections::HashMap::new());

    // for (i in boost) { if (boost[i]! > 0) { boostPlus[i] = (boostPlus[i] || 0) + boost[i]!; this.effectState.ready = true; } }
    use crate::dex_data::BoostID;
    let mut ready = false;
    for boost_id in BoostID::all() {
        let boost_value = boost.get(*boost_id);
        if boost_value > 0 {
            let boost_name = format!("{:?}", boost_id).to_lowercase();
            let current = boosts_map.get(&boost_name).copied().unwrap_or(0);
            boosts_map.insert(boost_name, current + boost_value);
            ready = true;
        }
    }

    // Store updated boosts and ready in effectState
    battle.with_effect_state(|state| {
        state
            .data
            .insert("boosts".to_string(), serde_json::json!(boosts_map));
        if ready {
            state
                .data
                .insert("ready".to_string(), serde_json::json!(true));
        }
    });

    EventResult::Continue
}

/// onAnySwitchIn() {
///     if (!this.effectState.ready) return;
///     (this.effectState.target as Pokemon).useItem();
/// }
pub fn on_any_switch_in(battle: &mut Battle) -> EventResult {
    // if (!this.effectState.ready) return;
    let ready = battle.with_effect_state_ref(|state| {
        state
            .data
            .get("ready")
            .and_then(|v| v.as_bool())
            .unwrap_or(false)
    }).unwrap_or(false);

    if !ready {
        return EventResult::Continue;
    }

    // (this.effectState.target as Pokemon).useItem();
    let target_pos = battle.with_effect_state_ref(|state| state.target).flatten();

    if let Some(pos) = target_pos {
        Pokemon::use_item(battle, pos, None, None);
    }

    EventResult::Continue
}

/// onAnyAfterMega() {
///     if (!this.effectState.ready) return;
///     (this.effectState.target as Pokemon).useItem();
/// }
pub fn on_any_after_mega(battle: &mut Battle) -> EventResult {
    // if (!this.effectState.ready) return;
    let ready = battle.with_effect_state_ref(|state| {
        state
            .data
            .get("ready")
            .and_then(|v| v.as_bool())
            .unwrap_or(false)
    }).unwrap_or(false);

    if !ready {
        return EventResult::Continue;
    }

    // (this.effectState.target as Pokemon).useItem();
    let target_pos = battle.with_effect_state_ref(|state| state.target).flatten();

    if let Some(pos) = target_pos {
        Pokemon::use_item(battle, pos, None, None);
    }

    EventResult::Continue
}

/// onAnyAfterTerastallization() {
///     if (!this.effectState.ready) return;
///     (this.effectState.target as Pokemon).useItem();
/// }
pub fn on_any_after_terastallization(battle: &mut Battle) -> EventResult {
    // if (!this.effectState.ready) return;
    let ready = battle.with_effect_state_ref(|state| {
        state
            .data
            .get("ready")
            .and_then(|v| v.as_bool())
            .unwrap_or(false)
    }).unwrap_or(false);

    if !ready {
        return EventResult::Continue;
    }

    // (this.effectState.target as Pokemon).useItem();
    let target_pos = battle.with_effect_state_ref(|state| state.target).flatten();

    if let Some(pos) = target_pos {
        Pokemon::use_item(battle, pos, None, None);
    }

    EventResult::Continue
}

/// onAnyAfterMove() {
///     if (!this.effectState.ready) return;
///     (this.effectState.target as Pokemon).useItem();
/// }
pub fn on_any_after_move(battle: &mut Battle) -> EventResult {
    // if (!this.effectState.ready) return;
    let ready = battle.with_effect_state_ref(|state| {
        state
            .data
            .get("ready")
            .and_then(|v| v.as_bool())
            .unwrap_or(false)
    }).unwrap_or(false);

    if !ready {
        return EventResult::Continue;
    }

    // (this.effectState.target as Pokemon).useItem();
    let target_pos = battle.with_effect_state_ref(|state| state.target).flatten();

    if let Some(pos) = target_pos {
        Pokemon::use_item(battle, pos, None, None);
    }

    EventResult::Continue
}

/// onResidual(pokemon) {
///     if (!this.effectState.ready) return;
///     (this.effectState.target as Pokemon).useItem();
/// }
pub fn on_residual(battle: &mut Battle, _pokemon_pos: (usize, usize)) -> EventResult {
    // if (!this.effectState.ready) return;
    let ready = battle.with_effect_state_ref(|state| {
        state
            .data
            .get("ready")
            .and_then(|v| v.as_bool())
            .unwrap_or(false)
    }).unwrap_or(false);

    if !ready {
        return EventResult::Continue;
    }

    // (this.effectState.target as Pokemon).useItem();
    let target_pos = battle.with_effect_state_ref(|state| state.target).flatten();

    if let Some(pos) = target_pos {
        Pokemon::use_item(battle, pos, None, None);
    }

    EventResult::Continue
}

/// onUse(pokemon) {
///     this.boost(this.effectState.boosts, pokemon);
/// }
pub fn on_use(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // this.boost(this.effectState.boosts, pokemon);
    let boosts_map = battle.with_effect_state_ref(|state| {
        if let Some(boosts_value) = state.data.get("boosts") {
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
    }).flatten();

    if let Some(boosts) = boosts_map {
        // Convert HashMap to Vec of (&str, i8) tuples for battle.boost()
        let boost_pairs: Vec<(&str, i8)> = boosts
            .iter()
            .map(|(k, v)| (k.as_str(), *v))
            .collect();

        battle.boost(&boost_pairs, pokemon_pos, None, None, false, false);
    }

    EventResult::Continue
}

/// onEnd() {
///     delete this.effectState.boosts;
///     delete this.effectState.ready;
/// }
pub fn on_end(battle: &mut Battle, _pokemon_pos: (usize, usize)) -> EventResult {
    // delete this.effectState.boosts;
    // delete this.effectState.ready;
    battle.with_effect_state(|state| {
        state.data.remove("boosts");
        state.data.remove("ready");
    });

    EventResult::Continue
}
