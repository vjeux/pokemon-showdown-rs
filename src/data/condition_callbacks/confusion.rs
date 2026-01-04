//! Confusion Condition
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! JavaScript source: data/conditions.ts

use crate::battle::Battle;
use crate::dex_data::ID;
use crate::event::EventResult;

/// onStart
/// JavaScript source (data/conditions.ts):
/// ```js
/// onStart(target, source, sourceEffect) {
///     if (sourceEffect?.id === 'lockedmove') {
///         this.add('-start', target, 'confusion', '[fatigue]');
///     } else if (sourceEffect?.effectType === 'Ability') {
///         this.add('-start', target, 'confusion', '[from] ability: ' + sourceEffect.name, `[of] ${source}`);
///     } else {
///         this.add('-start', target, 'confusion');
///     }
///     const min = sourceEffect?.id === 'axekick' ? 3 : 2;
///     this.effectState.time = this.random(min, 6);
/// }
/// ```
pub fn on_start(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
) -> EventResult {
    eprintln!("[CONFUSION_ON_START] Called for {:?}", pokemon_pos);

    // Set confusion duration (time) to random value between 2-5 turns (or 3-5 for axekick)
    // this.effectState.time = this.random(min, 6);
    // Note: In Rust, we use effectState.duration instead of time
    // The duration is decremented in field_event.rs Residual phase (skipping first turn due to created_turn)

    // Check if source effect is axekick (min=3) or normal (min=2)
    let min = if let Some(ref source_effect) = battle.active_move {
        if source_effect.id.as_str() == "axekick" {
            3
        } else {
            2
        }
    } else {
        2
    };

    // Set random duration from min to 5 (inclusive)
    // JavaScript: this.random(min, 6) returns [min, 6) = [min, 5]
    let prng_before = battle.prng.call_count;
    let duration = battle.random_with_range(min, 6); // [min, 6) = [min, 5]
    let prng_after = battle.prng.call_count;

    eprintln!("[CONFUSION_ON_START] Setting duration={} (min={}, PRNG: {}->{})",
        duration, min, prng_before, prng_after);

    // Get mutable reference to the confusion volatile state and set duration
    let pokemon = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
        Some(p) => p,
        None => {
            eprintln!("[CONFUSION_ON_START] ERROR: Pokemon not found at {:?}", pokemon_pos);
            return EventResult::Continue;
        }
    };

    if let Some(confusion_state) = pokemon.volatiles.get_mut(&ID::new("confusion")) {
        confusion_state.duration = Some(duration);
        eprintln!("[CONFUSION_ON_START] Set confusion.duration to {}", duration);
    } else {
        eprintln!("[CONFUSION_ON_START] WARNING: confusion volatile not found in pokemon.volatiles");
    }

    EventResult::Continue
}

/// onEnd
/// TODO: Implement 1-to-1 from JavaScript
/// JavaScript source (data/conditions.ts):
/// confusion: {
///     onEnd(...) {
///         // Extract implementation from conditions.ts
///     }
/// }
pub fn on_end(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
) -> EventResult {
    eprintln!("[CONFUSION_ON_END] Called for {:?}", pokemon_pos);
    // TODO: Implement callback
    EventResult::Continue
}

/// onBeforeMove
/// TODO: Implement 1-to-1 from JavaScript
/// JavaScript source (data/conditions.ts):
/// confusion: {
///     onBeforeMove(...) {
///         // Extract implementation from conditions.ts
///     }
/// }
pub fn on_before_move(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
) -> EventResult {
    eprintln!("[CONFUSION_ON_BEFORE_MOVE] Called for {:?}", pokemon_pos);
    // TODO: Implement callback
    EventResult::Continue
}

