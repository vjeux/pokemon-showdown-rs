//! Opportunist Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::battle::Effect;
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
pub fn on_foe_after_boost(battle: &mut Battle, _target_pos: Option<(usize, usize)>, _source_pos: Option<(usize, usize)>, effect: Option<&Effect>) -> EventResult {
    let effect_id = effect.map(|e| e.id.as_str());
    // Debug logging
    debug_elog!("[OPPORTUNIST on_foe_after_boost] ENTRY: effect_id={:?}", effect_id);
    debug_elog!("[OPPORTUNIST on_foe_after_boost] battle.event={:?}", battle.event.as_ref().map(|e| (&e.id, &e.relay_var)));

    // if (effect?.name === 'Opportunist' || effect?.name === 'Mirror Herb') return;
    if let Some(effect) = effect_id {
        if effect == "opportunist" || effect == "mirrorherb" {
            debug_elog!("[OPPORTUNIST on_foe_after_boost] Returning early - effect is self");
            return EventResult::Continue;
        }
    }

    // Get boost information from current event
    let boosts = if let Some(ref event) = battle.event {
        if let Some(boost_table) = match &event.relay_var { Some(crate::event::EventResult::Boost(b)) => Some(*b), _ => None } {
            debug_elog!("[OPPORTUNIST on_foe_after_boost] Got boost_table: {:?}", boost_table);
            boost_table
        } else {
            debug_elog!("[OPPORTUNIST on_foe_after_boost] No Boost in relay_var, returning early");
            return EventResult::Continue;
        }
    } else {
        debug_elog!("[OPPORTUNIST on_foe_after_boost] No event, returning early");
        return EventResult::Continue;
    };

    // if (!this.effectState.boosts) this.effectState.boosts = {} as SparseBoostsTable;
    // Get or create the boosts object in effect_state
    let mut accumulated_boosts = battle.with_effect_state_ref(|state| {
        state.boosts
    }).flatten().unwrap_or_default();

    // for (i in boost) { if (boost[i]! > 0) { boostPlus[i] = (boostPlus[i] || 0) + boost[i]!; } }
    if boosts.atk > 0 {
        accumulated_boosts.atk += boosts.atk;
    }
    if boosts.def > 0 {
        accumulated_boosts.def += boosts.def;
    }
    if boosts.spa > 0 {
        accumulated_boosts.spa += boosts.spa;
    }
    if boosts.spd > 0 {
        accumulated_boosts.spd += boosts.spd;
    }
    if boosts.spe > 0 {
        accumulated_boosts.spe += boosts.spe;
    }
    if boosts.accuracy > 0 {
        accumulated_boosts.accuracy += boosts.accuracy;
    }
    if boosts.evasion > 0 {
        accumulated_boosts.evasion += boosts.evasion;
    }

    // Store the accumulated boosts
    debug_elog!("[OPPORTUNIST on_foe_after_boost] About to store accumulated_boosts={:?}", accumulated_boosts);
    debug_elog!("[OPPORTUNIST on_foe_after_boost] battle.effect={:?}", battle.effect.as_ref().map(|e| (&e.id, &e.effect_type, &e.effect_holder)));
    battle.with_effect_state(|state| {
        state.boosts = Some(accumulated_boosts);
        debug_elog!("[OPPORTUNIST on_foe_after_boost] Stored in effect_state: boosts={:?}", state.boosts);
    });

    EventResult::Continue
}

/// onAnySwitchIn() {
///     if (!this.effectState.boosts) return;
///     this.boost(this.effectState.boosts, this.effectState.target);
///     delete this.effectState.boosts;
/// }
pub fn on_any_switch_in(battle: &mut Battle) -> EventResult {
    debug_elog!("[OPPORTUNIST on_any_switch_in] Called");
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
    debug_elog!("[OPPORTUNIST on_any_after_move] Called");
    apply_accumulated_boosts(battle);
    EventResult::Continue
}

/// onResidual(pokemon) {
///     if (!this.effectState.boosts) return;
///     this.boost(this.effectState.boosts, this.effectState.target);
///     delete this.effectState.boosts;
/// }
pub fn on_residual(battle: &mut Battle, _pokemon_pos: (usize, usize), _source_pos: Option<(usize, usize)>, _effect: Option<&Effect>) -> EventResult {
    debug_elog!("[OPPORTUNIST on_residual] Called");
    apply_accumulated_boosts(battle);
    EventResult::Continue
}

/// onEnd() {
///     delete this.effectState.boosts;
/// }
pub fn on_end(battle: &mut Battle, _pokemon_pos: (usize, usize)) -> EventResult {
    debug_elog!("[OPPORTUNIST on_end] Called");
    // delete this.effectState.boosts;
    battle.with_effect_state(|state| {
        state.boosts = None;
    });
    EventResult::Continue
}

// Helper function to apply accumulated boosts
fn apply_accumulated_boosts(battle: &mut Battle) {
    // Debug logging
    debug_elog!("[OPPORTUNIST apply_accumulated_boosts] ENTRY");
    debug_elog!("[OPPORTUNIST apply_accumulated_boosts] battle.effect={:?}", battle.effect.as_ref().map(|e| (&e.id, &e.effect_type, &e.effect_holder)));

    // if (!this.effectState.boosts) return;
    let boosts_to_apply = match battle.with_effect_state_ref(|state| state.boosts).flatten() {
        Some(boosts) => {
            debug_elog!("[OPPORTUNIST apply_accumulated_boosts] Found boosts: {:?}", boosts);
            boosts.clone()
        },
        None => {
            debug_elog!("[OPPORTUNIST apply_accumulated_boosts] No boosts found, returning early");
            return;
        }
    };

    // Check if there are any non-zero boosts
    if boosts_to_apply.atk == 0 && boosts_to_apply.def == 0 && boosts_to_apply.spa == 0
        && boosts_to_apply.spd == 0 && boosts_to_apply.spe == 0
        && boosts_to_apply.accuracy == 0 && boosts_to_apply.evasion == 0 {
        debug_elog!("[OPPORTUNIST apply_accumulated_boosts] All boosts are zero, returning");
        return;
    }

    // this.boost(this.effectState.boosts, this.effectState.target);
    // Note: In JS, this.effectState.target is set to the effect holder by the event system.
    // In Rust, we use battle.current_effect_holder() which returns the effect holder from battle.effect.
    let target_pos = match battle.current_effect_holder() {
        Some(pos) => {
            debug_elog!("[OPPORTUNIST apply_accumulated_boosts] target_pos={:?}", pos);
            pos
        },
        None => {
            debug_elog!("[OPPORTUNIST apply_accumulated_boosts] No target_pos, returning");
            return;
        }
    };

    // Build the boosts array filtering out zero values
    let boosts: Vec<(&str, i8)> = vec![
        ("atk", boosts_to_apply.atk),
        ("def", boosts_to_apply.def),
        ("spa", boosts_to_apply.spa),
        ("spd", boosts_to_apply.spd),
        ("spe", boosts_to_apply.spe),
        ("accuracy", boosts_to_apply.accuracy),
        ("evasion", boosts_to_apply.evasion),
    ].into_iter()
        .filter(|(_, val)| *val != 0)
        .collect();

    debug_elog!("[OPPORTUNIST apply_accumulated_boosts] About to call battle.boost with boosts={:?}, target={:?}", boosts, target_pos);

    if !boosts.is_empty() {
        battle.boost(&boosts, target_pos, None, None, false, false);
        debug_elog!("[OPPORTUNIST apply_accumulated_boosts] battle.boost completed");
    }

    // delete this.effectState.boosts;
    battle.with_effect_state(|state| {
        state.boosts = None;
    });
}

