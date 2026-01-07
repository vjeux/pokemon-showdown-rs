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
    }

    // this.effectState.trueDuration--;
    let pokemon = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };

    let lockedmove_id = ID::from("lockedmove");
    if let Some(volatile) = pokemon.volatiles.get_mut(&lockedmove_id) {
        if let Some(true_duration_val) = volatile.data.get_mut("trueDuration") {
            if let Some(true_duration) = true_duration_val.as_i64() {
                *true_duration_val = serde_json::Value::Number(serde_json::Number::from(true_duration - 1));
            }
        }
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

    // this.effectState.move = effect.id;
    let move_id = match &battle.effect {
        Some(eff) => eff.to_string(),
        None => return EventResult::Continue,
    };

    // Set volatile state data
    let pokemon = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };

    let lockedmove_id = ID::from("lockedmove");
    if let Some(volatile) = pokemon.volatiles.get_mut(&lockedmove_id) {
        volatile.data.insert("trueDuration".to_string(), serde_json::Value::Number(serde_json::Number::from(true_duration)));
        volatile.data.insert("move".to_string(), serde_json::Value::String(move_id));
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
    let true_duration_ge_2 = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        let lockedmove_id = ID::from("lockedmove");
        pokemon.volatiles.get(&lockedmove_id)
            .and_then(|v| v.data.get("trueDuration"))
            .and_then(|d| d.as_i64())
            .map(|d| d >= 2)
            .unwrap_or(false)
    };

    if true_duration_ge_2 {
        // this.effectState.duration = 2;
        let pokemon = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        let lockedmove_id = ID::from("lockedmove");
        if let Some(volatile) = pokemon.volatiles.get_mut(&lockedmove_id) {
            volatile.duration = Some(2);
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
            .map(|d| d == 1)
            .unwrap_or(false)
    };

    if duration_is_1 {
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
    let true_duration_gt_1 = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        let lockedmove_id = ID::from("lockedmove");
        pokemon.volatiles.get(&lockedmove_id)
            .and_then(|v| v.data.get("trueDuration"))
            .and_then(|d| d.as_i64())
            .map(|d| d > 1)
            .unwrap_or(false)
    };

    if true_duration_gt_1 {
        return EventResult::Continue;
    }

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
    let move_id = {
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

