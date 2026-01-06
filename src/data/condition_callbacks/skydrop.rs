//! Skydrop Condition Callbacks
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! JavaScript source: data/moves.ts (skydrop condition)

use crate::battle::Battle;
use crate::event::EventResult;

/// onAnyInvulnerability
///
/// JavaScript source (data/moves.ts):
/// ```js
/// onAnyInvulnerability(target, source, move) {
///     if (target !== this.effectState.target && target !== this.effectState.source) {
///         return;
///     }
///     if (source === this.effectState.target && target === this.effectState.source) {
///         return;
///     }
///     if (["gust", "twister", "skyuppercut", "thunder", "hurricane", "smackdown", "thousandarrows"].includes(move.id)) {
///         return;
///     }
///     return false;
/// }
/// ```
pub fn on_any_invulnerability(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
) -> EventResult {
    println!("[SKYDROP_INVULN] Called for pokemon at {:?}", pokemon_pos);

    // Get attacking move ID from current event's effect field
    let attacking_move_id = battle.current_event.as_ref()
        .and_then(|e| e.effect.as_ref())
        .map(|id| id.as_str());

    println!("[SKYDROP_INVULN] Attacking move: {:?}", attacking_move_id);

    // Get target and source from invulnerability event
    let invuln_target_pos = battle.current_event.as_ref()
        .and_then(|e| e.target);
    let invuln_source_pos = battle.current_event.as_ref()
        .and_then(|e| e.source);

    println!("[SKYDROP_INVULN] Event target: {:?}, source: {:?}", invuln_target_pos, invuln_source_pos);

    // Get skydrop effectState target and source
    let (effectstate_target, effectstate_source) = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => {
                println!("[SKYDROP_INVULN] Pokemon not found at {:?}, returning Continue", pokemon_pos);
                return EventResult::Continue;
            }
        };

        let skydrop_id = crate::dex_data::ID::from("skydrop");
        let state = match pokemon.volatiles.get(&skydrop_id) {
            Some(s) => s,
            None => {
                println!("[SKYDROP_INVULN] No skydrop volatile on pokemon, returning Continue");
                return EventResult::Continue;
            }
        };

        (state.target, state.source)
    };

    println!("[SKYDROP_INVULN] EffectState target: {:?}, source: {:?}", effectstate_target, effectstate_source);

    // JavaScript: if (target !== this.effectState.target && target !== this.effectState.source) return;
    if invuln_target_pos != effectstate_target && invuln_target_pos != effectstate_source {
        println!("[SKYDROP_INVULN] Target not relevant, returning Continue");
        return EventResult::Continue;
    }

    // JavaScript: if (source === this.effectState.target && target === this.effectState.source) return;
    if invuln_source_pos == effectstate_target && invuln_target_pos == effectstate_source {
        println!("[SKYDROP_INVULN] Source is target and target is source, returning Continue");
        return EventResult::Continue;
    }

    // JavaScript: if (["gust", "twister", "skyuppercut", "thunder", "hurricane", "smackdown", "thousandarrows"].includes(move.id)) return;
    if let Some(move_id) = attacking_move_id {
        const HITS_SKYDROP: &[&str] = &[
            "gust",
            "twister",
            "skyuppercut",
            "thunder",
            "hurricane",
            "smackdown",
            "thousandarrows",
        ];

        if HITS_SKYDROP.contains(&move_id) {
            println!("[SKYDROP_INVULN] Move {} can hit through skydrop, returning Continue", move_id);
            return EventResult::Continue;
        }
    }

    // JavaScript: return false; (means invulnerable)
    println!("[SKYDROP_INVULN] Making pokemon invulnerable, returning Boolean(false)");
    EventResult::Boolean(false)
}

/// onFoeBeforeMove
///
/// JavaScript source (data/moves.ts):
/// ```js
/// onFoeBeforeMove(target, source, move) {
///     if (source === this.effectState.target && target === this.effectState.source) {
///         target.removeMove(move.id);
///         this.debug('Sky drop nullifying.');
///         return null;
///     }
/// },
/// ```
pub fn on_foe_before_move(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
) -> EventResult {
    // Get effectState.source from the skydrop volatile
    let effect_source = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        let skydrop_id = crate::dex_data::ID::from("skydrop");
        let state = match pokemon.volatiles.get(&skydrop_id) {
            Some(s) => s,
            None => return EventResult::Continue,
        };

        state.source
    };

    if let Some(source) = effect_source {
        battle.decrement_active_move_actions(source);
        battle.debug("Sky drop nullifying.");
        return EventResult::Null;
    }

    EventResult::Continue
}
