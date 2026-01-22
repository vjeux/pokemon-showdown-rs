//! Stall Condition
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! The "stall" volatile tracks protect move usage and increasing failure rate.
//! JavaScript source: data/conditions.ts

use crate::battle::Battle;
use crate::battle::Effect;
use crate::dex_data::ID;
use crate::event::EventResult;

/// onStart() {
///     this.effectState.counter = 3;
/// }
pub fn on_start(
    battle: &mut Battle,
    _pokemon_pos: (usize, usize),
    _source_pos: Option<(usize, usize)>,
    _effect: Option<&Effect>,
) -> EventResult {
    // Set counter to 3 when stall is first added
    // In JavaScript: this.effectState.counter = 3
    battle.with_effect_state(|state| {
        state.counter = Some(3);
    });

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
    // Get counter from effect state
    // JavaScript: this.effectState.counter || 1
    let counter = battle.with_effect_state_ref(|state| {
        state.counter.unwrap_or(1)
    }).unwrap_or(1);

    // Call randomChance(1, counter)
    let success = battle.random_chance(1.0, counter);

    // If unsuccessful, remove the stall volatile
    if !success {
        let pokemon_mut = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon_mut.volatiles.remove(&ID::from("stall"));
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
    _pokemon_pos: (usize, usize),
    _source_pos: Option<(usize, usize)>,
    _effect: Option<&Effect>,
) -> EventResult {
    const COUNTER_MAX: i32 = 729; // From conditions.json

    // Get current counter and update it
    battle.with_effect_state(|state| {
        let current_counter = state.counter.unwrap_or(3);

        // if (this.effectState.counter < this.effect.counterMax) {
        //     this.effectState.counter *= 3;
        // }
        let new_counter = if current_counter < COUNTER_MAX {
            current_counter * 3
        } else {
            current_counter
        };

        state.counter = Some(new_counter);
        state.duration = Some(2);
    });

    EventResult::Continue
}
