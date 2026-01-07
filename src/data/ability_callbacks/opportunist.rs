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
    // if (effect?.name === 'Opportunist' || effect?.name === 'Mirror Herb') return;
    if let Some(effect) = effect_id {
        if effect == "opportunist" || effect == "mirrorherb" {
            return EventResult::Continue;
        }
    }

    // Get boost information from current event
    let boosts = if let Some(ref event) = battle.event {
        if let Some(boost_table) = match &event.relay_var { Some(crate::event::EventResult::Boost(b)) => Some(*b), _ => None } {
            boost_table
        } else {
            return EventResult::Continue;
        }
    } else {
        return EventResult::Continue;
    };

    // if (!this.effectState.boosts) this.effectState.boosts = {} as SparseBoostsTable;
    // Get or create the boosts object in effect_state
    let mut accumulated_boosts = battle.effect_state.boosts.unwrap_or_default();

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
    battle.effect_state.boosts = Some(accumulated_boosts);

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
    battle.effect_state.boosts = None;
    EventResult::Continue
}

// Helper function to apply accumulated boosts
fn apply_accumulated_boosts(battle: &mut Battle) {
    // if (!this.effectState.boosts) return;
    let boosts_to_apply = match battle.effect_state.boosts {
        Some(ref boosts) => boosts.clone(),
        None => return,
    };

    // Check if there are any non-zero boosts
    if boosts_to_apply.atk == 0 && boosts_to_apply.def == 0 && boosts_to_apply.spa == 0
        && boosts_to_apply.spd == 0 && boosts_to_apply.spe == 0
        && boosts_to_apply.accuracy == 0 && boosts_to_apply.evasion == 0 {
        return;
    }

    // this.boost(this.effectState.boosts, this.effectState.target);
    let target_pos = match battle.effect_state.target {
        Some(pos) => pos,
        None => return,
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

    if !boosts.is_empty() {
        battle.boost(&boosts, target_pos, None, None, false, false);
    }

    // delete this.effectState.boosts;
    battle.effect_state.boosts = None;
}

