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
    battle.with_effect_state(|state| {
        if let Some(true_duration) = state.true_duration {
            let new_duration = true_duration - 1;
            eprintln!("[LOCKEDMOVE_RESIDUAL] trueDuration {} -> {}", true_duration, new_duration);
            state.true_duration = Some(new_duration);
        }
    });

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
    // Get move ID from source_effect or current_effect
    let move_id = battle.with_effect_state_ref(|state| {
        state.source_effect.as_ref().map(|id| id.to_string())
    }).flatten()
        .or_else(|| battle.current_effect_id().map(|id| id.to_string()));

    let move_id = match move_id {
        Some(id) => id,
        None => {
            eprintln!("[LOCKEDMOVE_START] No source_effect or current_effect!");
            return EventResult::Continue;
        }
    };

    // Set volatile state data
    battle.with_effect_state(|state| {
        state.true_duration = Some(true_duration);
        state.move_id = Some(move_id);
        eprintln!("[LOCKEDMOVE_START] Set trueDuration={} and move", true_duration);
    });

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
    _pokemon_pos: (usize, usize),
    _source_pos: Option<(usize, usize)>,
    _effect_id: Option<&str>,
) -> EventResult {
    // if (this.effectState.trueDuration >= 2) { this.effectState.duration = 2; }
    battle.with_effect_state(|state| {
        let true_duration_ge_2 = state.true_duration
            .map(|d| d >= 2)
            .unwrap_or(false);

        if true_duration_ge_2 {
            state.duration = Some(2);
        }
    });

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
    _active_move: Option<&crate::battle_actions::ActiveMove>,
) -> EventResult {
    // if (this.effectState.duration === 1)
    // JavaScript: this.effectState.duration
    let duration_is_1 = battle.with_effect_state_ref(|state| {
        state.duration.map(|d| {
            eprintln!("[LOCKEDMOVE_AFTERMOVE] turn={}, pokemon=({}, {}), duration={}, checking if == 1",
                battle.turn, pokemon_pos.0, pokemon_pos.1, d);
            d == 1
        }).unwrap_or(false)
    }).unwrap_or(false);

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
    // JavaScript: this.effectState.trueDuration
    let true_duration = battle.with_effect_state_ref(|state| {
        state.true_duration.unwrap_or(0)
    }).unwrap_or(0);

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
    // JavaScript: this.effectState.move
    let move_id = battle.with_effect_state_ref(|state| {
        state.move_id.clone()
    }).flatten();

    match move_id {
        Some(m) => EventResult::ID(ID::from(m.as_str())),
        None => EventResult::Continue,
    }
}
