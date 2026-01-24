//! Skydrop Condition Callbacks
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! JavaScript source: data/moves.ts (skydrop condition)

use crate::battle::{Battle, EffectHolder};
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
    target_pos: (usize, usize),
    source_pos: (usize, usize),
    attacking_active_move: Option<&crate::battle_actions::ActiveMove>,
) -> EventResult {
    let attacking_move_id = attacking_active_move.map(|m| m.id.to_string()).unwrap_or_default();
    debug_log!("[SKYDROP_INVULN] Called for target={:?}, source={:?}, move='{}'", target_pos, source_pos, attacking_move_id);

    // Get the Pokemon that has the skydrop volatile from effect_holder
    // This is set by single_event to be the Pokemon that owns the volatile
    // The volatile is on the SKY DROP USER (e.g., Zygarde), not the grabbed target
    let effect_holder = match battle.effect.as_ref().and_then(|e| e.effect_holder.clone()) {
        Some(holder) => holder,
        None => {
            debug_log!("[SKYDROP_INVULN] No effect_holder, returning Continue");
            return EventResult::Continue;
        }
    };

    debug_log!("[SKYDROP_INVULN] effect_holder={:?}", effect_holder);

    let (effectstate_target, effectstate_source) = {
        let EffectHolder::Pokemon(side_idx, poke_idx) = effect_holder else {
            debug_log!("[SKYDROP_INVULN] Effect holder is not a Pokemon, returning Continue");
            return EventResult::Continue;
        };
        let pokemon = match battle.pokemon_at(side_idx, poke_idx) {
            Some(p) => p,
            None => {
                debug_log!("[SKYDROP_INVULN] Pokemon not found at effect_holder {:?}, returning Continue", effect_holder);
                return EventResult::Continue;
            }
        };

        let skydrop_id = crate::dex_data::ID::from("skydrop");
        let state = match pokemon.volatiles.get(&skydrop_id) {
            Some(s) => s,
            None => {
                debug_log!("[SKYDROP_INVULN] No skydrop volatile on effect_holder, returning Continue");
                return EventResult::Continue;
            }
        };

        (state.borrow().target, state.borrow().source)
    };

    debug_log!("[SKYDROP_INVULN] EffectState target: {:?}, source: {:?}", effectstate_target, effectstate_source);

    // JavaScript: if (target !== this.effectState.target && target !== this.effectState.source) return;
    if Some(target_pos) != effectstate_target && Some(target_pos) != effectstate_source {
        debug_log!("[SKYDROP_INVULN] Target not relevant, returning Continue");
        return EventResult::Continue;
    }

    // JavaScript: if (source === this.effectState.target && target === this.effectState.source) return;
    if Some(source_pos) == effectstate_target && Some(target_pos) == effectstate_source {
        debug_log!("[SKYDROP_INVULN] Source is target and target is source, returning Continue");
        return EventResult::Continue;
    }

    // JavaScript: if (["gust", "twister", "skyuppercut", "thunder", "hurricane", "smackdown", "thousandarrows"].includes(move.id)) return;
    const HITS_SKYDROP: &[&str] = &[
        "gust",
        "twister",
        "skyuppercut",
        "thunder",
        "hurricane",
        "smackdown",
        "thousandarrows",
    ];

    if HITS_SKYDROP.contains(&attacking_move_id.as_str()) {
        debug_log!("[SKYDROP_INVULN] Move {} can hit through skydrop, returning Continue", attacking_move_id);
        return EventResult::Continue;
    }

    // JavaScript: return false; (means invulnerable)
    debug_log!("[SKYDROP_INVULN] Making pokemon invulnerable, returning Boolean(false)");
    EventResult::Boolean(false)
}

/// onFoeBeforeMove
///
/// JavaScript source (data/moves.ts):
/// ```js
/// onFoeBeforeMove(attacker, defender, move) {
///     if (attacker === this.effectState.source) {
///         attacker.activeMoveActions--;
///         this.debug('Sky drop nullifying.');
///         return null;
///     }
/// },
/// ```
pub fn on_foe_before_move(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
    target_pos: Option<(usize, usize)>,
    _source_pos: Option<(usize, usize)>,
    _active_move: Option<&crate::battle_actions::ActiveMove>,
) -> EventResult {
    debug_elog!("[SKYDROP_FOE_BEFORE_MOVE] Called: pokemon_pos={:?}, target_pos={:?}", pokemon_pos, target_pos);

    // target_pos = the attacker (Pokemon trying to move)
    // We need to check if attacker === this.effectState.source

    // Get effectState.source from the skydrop volatile on the pokemon with the handler
    let effect_source = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => {
                debug_elog!("[SKYDROP_FOE_BEFORE_MOVE] No pokemon at {:?}", pokemon_pos);
                return EventResult::Continue;
            }
        };

        let skydrop_id = crate::dex_data::ID::from("skydrop");
        let state = match pokemon.volatiles.get(&skydrop_id) {
            Some(s) => s,
            None => {
                debug_elog!("[SKYDROP_FOE_BEFORE_MOVE] No skydrop volatile on {:?}", pokemon_pos);
                return EventResult::Continue;
            }
        };

        debug_elog!("[SKYDROP_FOE_BEFORE_MOVE] Found skydrop volatile, source={:?}", state.borrow().source);
        state.borrow().source
    };

    debug_elog!("[SKYDROP_FOE_BEFORE_MOVE] Comparing target_pos={:?} with effect_source={:?}", target_pos, effect_source);

    // JavaScript: if (attacker === this.effectState.source) {
    // attacker is target_pos, effectState.source is effect_source
    if target_pos == effect_source {
        debug_elog!("[SKYDROP_FOE_BEFORE_MOVE] MATCH! Nullifying attacker's move");
        if let Some(attacker_pos) = target_pos {
            // attacker.activeMoveActions--;
            battle.decrement_active_move_actions(attacker_pos);
        }
        battle.debug("Sky drop nullifying.");
        // JavaScript returns null, which is falsy and prevents the move
        return EventResult::Null;
    }

    debug_elog!("[SKYDROP_FOE_BEFORE_MOVE] No match, returning Continue");
    EventResult::Continue
}
