//! Stall Condition
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! The "stall" volatile tracks protect move usage and increasing failure rate.
//! JavaScript source: data/conditions.ts

use crate::battle::Battle;
use crate::dex_data::ID;
use crate::event::EventResult;

/// onStart() {
///     this.effectState.counter = 3;
/// }
pub fn on_start(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
    _source_pos: Option<(usize, usize)>,
    _effect_id: Option<&str>,
) -> EventResult {
    eprintln!("[STALL_START] Called for {:?}", pokemon_pos);

    // Set counter to 3 when stall is first added
    // In JavaScript: this.effectState.counter = 3
    // In Rust: battle.current_effect_state is a clone that will be copied back
    if let Some(ref mut effect_state) = battle.current_effect_state {
        effect_state.data.insert("counter".to_string(), serde_json::Value::from(3));
        eprintln!("[STALL_START] Set counter to 3 in current_effect_state");
    } else {
        eprintln!("[STALL_START] WARNING: No current_effect_state!");
    }

    EventResult::Continue
}

/// onStallMove(pokemon) {
///     const counter = this.effectState.counter || 1;
///     this.debug(`Success chance: ${Math.round(100 / counter)}%`);
///     const success = this.randomChance(1, counter);
///     if (!success) delete pokemon.volatiles["stall"];
///     return success;
/// }
pub fn on_stall_move(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
) -> EventResult {
    // Phase 1: Get counter from current_effect_state
    // In JavaScript: this.effectState.counter || 1
    // In Rust: battle.current_effect_state is a clone of the volatile's state
    let counter = if let Some(ref effect_state) = battle.current_effect_state {
        effect_state.data.get("counter")
            .and_then(|v| v.as_i64())
            .map(|v| v as i32)
            .unwrap_or(1)
    } else {
        // Fallback: try to read from pokemon.volatiles if current_effect_state not set
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.volatiles.get(&ID::from("stall"))
            .and_then(|v| v.data.get("counter"))
            .and_then(|v| v.as_i64())
            .map(|v| v as i32)
            .unwrap_or(1)
    };

    eprintln!("[STALL_MOVE] turn={}, counter={}, Success chance: {}%", battle.turn, counter, 100 / counter);

    // Call randomChance(1, counter)
    let success = battle.random_chance(1, counter);

    eprintln!("[STALL_MOVE] turn={}, randomChance(1, {}) = {}", battle.turn, counter, success);

    // If unsuccessful, remove the stall volatile
    if !success {
        let pokemon_mut = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon_mut.volatiles.remove(&ID::from("stall"));
        eprintln!("[STALL] Removed stall volatile (failed)");
    }

    EventResult::Boolean(success)
}

/// onRestart() {
///     if (this.effectState.counter < this.effect.counterMax) {
///         this.effectState.counter *= 3;
///     }
///     this.effectState.duration = 2;
/// }
pub fn on_restart(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
    _source_pos: Option<(usize, usize)>,
    _effect_id: Option<&str>,
) -> EventResult {
    eprintln!("[STALL_RESTART] Called for {:?}", pokemon_pos);

    const COUNTER_MAX: i32 = 729; // From conditions.json

    // Phase 1: Get current counter from current_effect_state
    // In JavaScript: this.effectState.counter
    // In Rust: battle.current_effect_state is a clone that will be copied back
    let current_counter = if let Some(ref effect_state) = battle.current_effect_state {
        effect_state.data.get("counter")
            .and_then(|v| v.as_i64())
            .map(|v| v as i32)
            .unwrap_or(3)
    } else {
        // Fallback: try to read from pokemon.volatiles if current_effect_state not set
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.volatiles.get(&ID::from("stall"))
            .and_then(|v| v.data.get("counter"))
            .and_then(|v| v.as_i64())
            .map(|v| v as i32)
            .unwrap_or(3)
    };

    eprintln!("[STALL_RESTART] Current counter: {}", current_counter);

    // Phase 2: Update counter and duration in current_effect_state
    if let Some(ref mut effect_state) = battle.current_effect_state {
        // if (this.effectState.counter < this.effect.counterMax) {
        //     this.effectState.counter *= 3;
        // }
        let new_counter = if current_counter < COUNTER_MAX {
            current_counter * 3
        } else {
            current_counter
        };

        effect_state.data.insert("counter".to_string(), serde_json::Value::from(new_counter));
        effect_state.duration = Some(2);

        eprintln!("[STALL_RESTART] Updated counter to {}, duration to 2", new_counter);
    } else {
        eprintln!("[STALL_RESTART] WARNING: No current_effect_state!");
    }

    EventResult::Continue
}
