//! Lockedmove Condition
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! JavaScript source: data/conditions.ts

use crate::battle::Battle;
use crate::dex_data::ID;
use crate::event::EventResult;

/// onResidual
/// JavaScript source (data/conditions.ts):
/// ```js
/// onResidual(target) {
///     if (target.status === 'slp') {
///         // don't lock, and bypass confusion for calming
///         delete target.volatiles['lockedmove'];
///     }
///     this.effectState.trueDuration--;
/// }
/// ```
pub fn on_residual(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
    _source_pos: Option<(usize, usize)>,
    _effect_id: Option<&str>,
) -> EventResult {
    eprintln!("[LOCKEDMOVE_RESIDUAL ENTRY] turn={}, pokemon=({}, {})",
        battle.turn, pokemon_pos.0, pokemon_pos.1);

    // if (target.status === 'slp')
    let is_asleep = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.status.as_str() == "slp"
    };

    if is_asleep {
        // delete target.volatiles['lockedmove'];
        let lockedmove_id = ID::from("lockedmove");
        crate::pokemon::Pokemon::remove_volatile(battle, pokemon_pos, &lockedmove_id);
        return EventResult::Continue;
    }

    // this.effectState.trueDuration--;
    // In JavaScript: this.effectState is a reference to the volatile's state
    // In Rust: battle.current_effect_state is a clone that will be copied back
    if let Some(ref mut effect_state) = battle.current_effect_state {
        if let Some(true_duration_val) = effect_state.data.get_mut("trueDuration") {
            if let Some(true_duration) = true_duration_val.as_i64() {
                let new_duration = true_duration - 1;
                eprintln!("[LOCKEDMOVE_RESIDUAL] turn={}, pokemon=({}, {}), trueDuration {} -> {}",
                    battle.turn, pokemon_pos.0, pokemon_pos.1, true_duration, new_duration);
                *true_duration_val = serde_json::Value::Number(serde_json::Number::from(new_duration));
            } else {
                eprintln!("[LOCKEDMOVE_RESIDUAL] trueDuration is not an i64");
            }
        } else {
            eprintln!("[LOCKEDMOVE_RESIDUAL] No trueDuration in effect_state.data");
        }
    } else {
        eprintln!("[LOCKEDMOVE_RESIDUAL] ERROR: No current_effect_state!");
    }

    EventResult::Continue
}

/// onStart
/// JavaScript source (data/conditions.ts):
/// ```js
/// onStart(target, source, effect) {
///     this.effectState.trueDuration = this.random(2, 4);
///     this.effectState.move = effect.id;
/// }
/// ```
pub fn on_start(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
    _source_pos: Option<(usize, usize)>,
    _effect_id: Option<&str>,
) -> EventResult {
    // this.effectState.trueDuration = this.random(2, 4);
    let true_duration = battle.random_with_range(2, 4);

    eprintln!("[LOCKEDMOVE_START] turn={}, pokemon=({}, {}), trueDuration={}",
        battle.turn, pokemon_pos.0, pokemon_pos.1, true_duration);

    // this.effectState.move = effect.id;
    // In JavaScript: effect is the second parameter to singleEvent, which is the condition (status)
    // But looking at the actual usage, effect.id should be the MOVE that triggered lockedmove
    // In Rust: current_effect_state.source_effect contains the move that caused this volatile
    let move_id = if let Some(ref effect_state) = battle.current_effect_state {
        effect_state.source_effect.as_ref()
            .map(|id| id.to_string())
            .or_else(|| battle.current_effect.as_ref().map(|id| id.to_string()))
    } else {
        None
    };

    let move_id = match move_id {
        Some(id) => id,
        None => {
            eprintln!("[LOCKEDMOVE_START] No source_effect or current_effect!");
            return EventResult::Continue;
        }
    };

    // Set volatile state data in current_effect_state
    // In JavaScript: this.effectState.trueDuration = ...
    // In Rust: battle.current_effect_state is a clone that will be copied back to pokemon.volatiles
    if let Some(ref mut effect_state) = battle.current_effect_state {
        effect_state.data.insert("trueDuration".to_string(), serde_json::Value::Number(serde_json::Number::from(true_duration)));
        effect_state.data.insert("move".to_string(), serde_json::Value::String(move_id));
        eprintln!("[LOCKEDMOVE_START] Set trueDuration={} and move in current_effect_state", true_duration);
    } else {
        eprintln!("[LOCKEDMOVE_START] ERROR: No current_effect_state!");
    }

    EventResult::Continue
}

/// onRestart
/// JavaScript source (data/conditions.ts):
/// ```js
/// onRestart() {
///     if (this.effectState.trueDuration >= 2) {
///         this.effectState.duration = 2;
///     }
/// }
/// ```
pub fn on_restart(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
    _source_pos: Option<(usize, usize)>,
    _effect_id: Option<&str>,
) -> EventResult {
    // if (this.effectState.trueDuration >= 2)
    // In JavaScript: this.effectState is a reference to the volatile's state
    // In Rust: battle.current_effect_state is a clone that will be copied back
    let true_duration_ge_2 = if let Some(ref effect_state) = battle.current_effect_state {
        effect_state.data.get("trueDuration")
            .and_then(|d| d.as_i64())
            .map(|d| d >= 2)
            .unwrap_or(false)
    } else {
        false
    };

    if true_duration_ge_2 {
        // this.effectState.duration = 2;
        if let Some(ref mut effect_state) = battle.current_effect_state {
            effect_state.duration = Some(2);
        }
    }

    EventResult::Continue
}

/// onAfterMove
/// JavaScript source (data/conditions.ts):
/// ```js
/// onAfterMove(pokemon) {
///     if (this.effectState.duration === 1) {
///         pokemon.removeVolatile('lockedmove');
///     }
/// }
/// ```
pub fn on_after_move(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
    _move_id: &str,
) -> EventResult {
    // if (this.effectState.duration === 1)
    let duration_is_1 = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        let lockedmove_id = ID::from("lockedmove");
        pokemon.volatiles.get(&lockedmove_id)
            .and_then(|v| v.duration)
            .map(|d| {
                eprintln!("[LOCKEDMOVE_AFTERMOVE] turn={}, pokemon=({}, {}), duration={}, checking if == 1",
                    battle.turn, pokemon_pos.0, pokemon_pos.1, d);
                d == 1
            })
            .unwrap_or(false)
    };

    if duration_is_1 {
        eprintln!("[LOCKEDMOVE_AFTERMOVE] Removing lockedmove volatile, will trigger onEnd");
        // pokemon.removeVolatile('lockedmove');
        let lockedmove_id = ID::from("lockedmove");
        crate::pokemon::Pokemon::remove_volatile(battle, pokemon_pos, &lockedmove_id);
    }

    EventResult::Continue
}

/// onEnd
/// JavaScript source (data/conditions.ts):
/// ```js
/// onEnd(target) {
///     if (this.effectState.trueDuration > 1) return;
///     target.addVolatile('confusion');
/// }
/// ```
pub fn on_end(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
) -> EventResult {
    // if (this.effectState.trueDuration > 1) return;
    // In JavaScript: this.effectState is a reference to the volatile's state
    // In Rust: battle.current_effect_state is a clone of the volatile's state
    let true_duration = if let Some(ref effect_state) = battle.current_effect_state {
        effect_state.data.get("trueDuration")
            .and_then(|d| d.as_i64())
            .unwrap_or(0)
    } else {
        // Fallback: try to read from pokemon.volatiles if current_effect_state not set
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        let lockedmove_id = ID::from("lockedmove");
        pokemon.volatiles.get(&lockedmove_id)
            .and_then(|v| v.data.get("trueDuration"))
            .and_then(|d| d.as_i64())
            .unwrap_or(0)
    };

    eprintln!("[LOCKEDMOVE_END] turn={}, pokemon=({}, {}), trueDuration={}",
        battle.turn, pokemon_pos.0, pokemon_pos.1, true_duration);

    if true_duration > 1 {
        eprintln!("[LOCKEDMOVE_END] trueDuration > 1, NOT adding confusion");
        return EventResult::Continue;
    }

    eprintln!("[LOCKEDMOVE_END] trueDuration <= 1, adding confusion");
    // target.addVolatile('confusion');
    let confusion_id = ID::from("confusion");
    crate::pokemon::Pokemon::add_volatile(battle, pokemon_pos, confusion_id, Some(pokemon_pos), None, None, None);

    EventResult::Continue
}

/// onLockMove
/// JavaScript source (data/conditions.ts):
/// ```js
/// onLockMove(pokemon) {
///     if (pokemon.volatiles['dynamax']) return;
///     return this.effectState.move;
/// }
/// ```
pub fn on_lock_move(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
) -> EventResult {
    // if (pokemon.volatiles['dynamax']) return;
    let has_dynamax = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        let dynamax_id = ID::from("dynamax");
        pokemon.volatiles.contains_key(&dynamax_id)
    };

    if has_dynamax {
        return EventResult::Continue;
    }

    // return this.effectState.move;
    // In JavaScript: this.effectState is a reference to the volatile's state
    // In Rust: battle.current_effect_state is a clone of the volatile's state
    let move_id = if let Some(ref effect_state) = battle.current_effect_state {
        effect_state.data.get("move")
            .and_then(|m| m.as_str())
            .map(|s| s.to_string())
    } else {
        // Fallback: try to read from pokemon.volatiles if current_effect_state not set
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        let lockedmove_id = ID::from("lockedmove");
        pokemon.volatiles.get(&lockedmove_id)
            .and_then(|v| v.data.get("move"))
            .and_then(|m| m.as_str())
            .map(|s| s.to_string())
    };

    match move_id {
        Some(m) => EventResult::ID(ID::from(m.as_str())),
        None => EventResult::Continue,
    }
}

