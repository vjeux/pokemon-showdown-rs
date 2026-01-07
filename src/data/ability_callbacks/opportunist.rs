//! Opportunist Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onFoeAfterBoost(boost, target, source, effect) {
///     if (effect?.name === 'Opportunist' || effect?.name === 'Mirror Herb') return;
///     if (!this.effectState.boosts) this.effectState.boosts = {} as SparseBoostsTable;
///     const boostPlus = this.effectState.boosts;
///     let i: BoostID;
///     for (i in boost) {
///         if (boost[i]! > 0) {
///             boostPlus[i] = (boostPlus[i] || 0) + boost[i]!;
///         }
///     }
/// }
pub fn on_foe_after_boost(battle: &mut Battle, _target_pos: Option<(usize, usize)>, _source_pos: Option<(usize, usize)>, effect_id: Option<&str>) -> EventResult {
    use serde_json::Value;

    // if (effect?.name === 'Opportunist' || effect?.name === 'Mirror Herb') return;
    if let Some(effect) = effect_id {
        if effect == "opportunist" || effect == "mirrorherb" {
            return EventResult::Continue;
        }
    }

    // Get boost information from current event
    let boosts = if let Some(ref event) = battle.current_event {
        if let Some(boost_table) = match &event.relay_var { Some(crate::event::EventResult::Boost(b)) => Some(*b), _ => None } {
            boost_table
        } else {
            return EventResult::Continue;
        }
    } else {
        return EventResult::Continue;
    };

    // if (!this.effectState.boosts) this.effectState.boosts = {} as SparseBoostsTable;
    // Get or create the boosts object in effect_state.data
    let accumulated_boosts: Vec<(&str, i8)> = if let Some(Value::Object(map)) = battle.effect_state.data.get("boosts") {
        vec![
            ("atk", map.get("atk").and_then(|v| v.as_i64()).unwrap_or(0) as i8),
            ("def", map.get("def").and_then(|v| v.as_i64()).unwrap_or(0) as i8),
            ("spa", map.get("spa").and_then(|v| v.as_i64()).unwrap_or(0) as i8),
            ("spd", map.get("spd").and_then(|v| v.as_i64()).unwrap_or(0) as i8),
            ("spe", map.get("spe").and_then(|v| v.as_i64()).unwrap_or(0) as i8),
            ("accuracy", map.get("accuracy").and_then(|v| v.as_i64()).unwrap_or(0) as i8),
            ("evasion", map.get("evasion").and_then(|v| v.as_i64()).unwrap_or(0) as i8),
        ]
    } else {
        vec![("atk", 0), ("def", 0), ("spa", 0), ("spd", 0), ("spe", 0), ("accuracy", 0), ("evasion", 0)]
    };

    // for (i in boost) { if (boost[i]! > 0) { boostPlus[i] = (boostPlus[i] || 0) + boost[i]!; } }
    let mut new_boosts = serde_json::Map::new();

    if boosts.atk > 0 {
        new_boosts.insert("atk".to_string(), Value::Number((accumulated_boosts[0].1 + boosts.atk).into()));
    } else if accumulated_boosts[0].1 != 0 {
        new_boosts.insert("atk".to_string(), Value::Number(accumulated_boosts[0].1.into()));
    }

    if boosts.def > 0 {
        new_boosts.insert("def".to_string(), Value::Number((accumulated_boosts[1].1 + boosts.def).into()));
    } else if accumulated_boosts[1].1 != 0 {
        new_boosts.insert("def".to_string(), Value::Number(accumulated_boosts[1].1.into()));
    }

    if boosts.spa > 0 {
        new_boosts.insert("spa".to_string(), Value::Number((accumulated_boosts[2].1 + boosts.spa).into()));
    } else if accumulated_boosts[2].1 != 0 {
        new_boosts.insert("spa".to_string(), Value::Number(accumulated_boosts[2].1.into()));
    }

    if boosts.spd > 0 {
        new_boosts.insert("spd".to_string(), Value::Number((accumulated_boosts[3].1 + boosts.spd).into()));
    } else if accumulated_boosts[3].1 != 0 {
        new_boosts.insert("spd".to_string(), Value::Number(accumulated_boosts[3].1.into()));
    }

    if boosts.spe > 0 {
        new_boosts.insert("spe".to_string(), Value::Number((accumulated_boosts[4].1 + boosts.spe).into()));
    } else if accumulated_boosts[4].1 != 0 {
        new_boosts.insert("spe".to_string(), Value::Number(accumulated_boosts[4].1.into()));
    }

    if boosts.accuracy > 0 {
        new_boosts.insert("accuracy".to_string(), Value::Number((accumulated_boosts[5].1 + boosts.accuracy).into()));
    } else if accumulated_boosts[5].1 != 0 {
        new_boosts.insert("accuracy".to_string(), Value::Number(accumulated_boosts[5].1.into()));
    }

    if boosts.evasion > 0 {
        new_boosts.insert("evasion".to_string(), Value::Number((accumulated_boosts[6].1 + boosts.evasion).into()));
    } else if accumulated_boosts[6].1 != 0 {
        new_boosts.insert("evasion".to_string(), Value::Number(accumulated_boosts[6].1.into()));
    }

    // Store the accumulated boosts
    if !new_boosts.is_empty() {
        battle.effect_state.data.insert("boosts".to_string(), Value::Object(new_boosts));
    }

    EventResult::Continue
}

/// onAnySwitchIn() {
///     if (!this.effectState.boosts) return;
///     this.boost(this.effectState.boosts, this.effectState.target);
///     delete this.effectState.boosts;
/// }
pub fn on_any_switch_in(battle: &mut Battle) -> EventResult {
    apply_accumulated_boosts(battle);
    EventResult::Continue
}

/// onAnyAfterMega() {
///     if (!this.effectState.boosts) return;
///     this.boost(this.effectState.boosts, this.effectState.target);
///     delete this.effectState.boosts;
/// }
pub fn on_any_after_mega(battle: &mut Battle) -> EventResult {
    apply_accumulated_boosts(battle);
    EventResult::Continue
}

/// onAnyAfterTerastallization() {
///     if (!this.effectState.boosts) return;
///     this.boost(this.effectState.boosts, this.effectState.target);
///     delete this.effectState.boosts;
/// }
pub fn on_any_after_terastallization(battle: &mut Battle) -> EventResult {
    apply_accumulated_boosts(battle);
    EventResult::Continue
}

/// onAnyAfterMove() {
///     if (!this.effectState.boosts) return;
///     this.boost(this.effectState.boosts, this.effectState.target);
///     delete this.effectState.boosts;
/// }
pub fn on_any_after_move(battle: &mut Battle) -> EventResult {
    apply_accumulated_boosts(battle);
    EventResult::Continue
}

/// onResidual(pokemon) {
///     if (!this.effectState.boosts) return;
///     this.boost(this.effectState.boosts, this.effectState.target);
///     delete this.effectState.boosts;
/// }
pub fn on_residual(battle: &mut Battle, _pokemon_pos: (usize, usize), _source_pos: Option<(usize, usize)>, _effect_id: Option<&str>) -> EventResult {
    apply_accumulated_boosts(battle);
    EventResult::Continue
}

/// onEnd() {
///     delete this.effectState.boosts;
/// }
pub fn on_end(battle: &mut Battle, _pokemon_pos: (usize, usize)) -> EventResult {
    // delete this.effectState.boosts;
    battle.effect_state.data.remove("boosts");
    EventResult::Continue
}

// Helper function to apply accumulated boosts
fn apply_accumulated_boosts(battle: &mut Battle) {
    use serde_json::Value;

    // if (!this.effectState.boosts) return;
    let boosts_to_apply: Vec<(&str, i8)> = if let Some(Value::Object(map)) = battle.effect_state.data.get("boosts") {
        vec![
            ("atk", map.get("atk").and_then(|v| v.as_i64()).unwrap_or(0) as i8),
            ("def", map.get("def").and_then(|v| v.as_i64()).unwrap_or(0) as i8),
            ("spa", map.get("spa").and_then(|v| v.as_i64()).unwrap_or(0) as i8),
            ("spd", map.get("spd").and_then(|v| v.as_i64()).unwrap_or(0) as i8),
            ("spe", map.get("spe").and_then(|v| v.as_i64()).unwrap_or(0) as i8),
            ("accuracy", map.get("accuracy").and_then(|v| v.as_i64()).unwrap_or(0) as i8),
            ("evasion", map.get("evasion").and_then(|v| v.as_i64()).unwrap_or(0) as i8),
        ]
    } else {
        return; // No boosts to apply
    };

    // Check if there are any non-zero boosts
    if !boosts_to_apply.iter().any(|(_, val)| *val != 0) {
        return;
    }

    // this.boost(this.effectState.boosts, this.effectState.target);
    let target_pos = match battle.effect_state.target {
        Some(pos) => pos,
        None => return,
    };

    // Build the boosts array filtering out zero values
    let boosts: Vec<(&str, i8)> = boosts_to_apply.into_iter()
        .filter(|(_, val)| *val != 0)
        .collect();

    if !boosts.is_empty() {
        battle.boost(&boosts, target_pos, None, None, false, false);
    }

    // delete this.effectState.boosts;
    battle.effect_state.data.remove("boosts");
}

