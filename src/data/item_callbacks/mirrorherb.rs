//! Mirror Herb Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::battle::Effect;
use crate::event::EventResult;
use crate::Pokemon;

/// onFoeAfterBoost(boost, target, source, effect) {
///     if (effect?.name === 'Opportunist' || effect?.name === 'Mirror Herb') return;
///     if (!this.effectState.boosts) this.effectState.boosts = {} as SparseBoostsTable;
///     const boostPlus = this.effectState.boosts;
///     let i: BoostID;
///     for (i in boost) {
///         if (boost[i]! > 0) {
///             boostPlus[i] = (boostPlus[i] || 0) + boost[i]!;
///             this.effectState.ready = true;
///         }
///     }
/// }
pub fn on_foe_after_boost(
    battle: &mut Battle,
    _target_pos: Option<(usize, usize)>,
    _source_pos: Option<(usize, usize)>,
    effect: Option<&Effect>,
    boost: &crate::dex_data::BoostsTable,
) -> EventResult {
    let effect_id = effect.map(|e| &*e.name);

    // if (effect?.name === 'Opportunist' || effect?.name === 'Mirror Herb') return;
    if let Some(eff_id) = effect_id {
        if eff_id == "Opportunist" || eff_id == "Mirror Herb" {
            return EventResult::Continue;
        }
    }

    // Get current boosts from effectState or create empty BoostsTable
    let mut current_boosts = battle.with_effect_state_ref(|state| {
        state.boosts.unwrap_or_default()
    }).unwrap_or_default();

    // for (i in boost) { if (boost[i]! > 0) { boostPlus[i] = (boostPlus[i] || 0) + boost[i]!; this.effectState.ready = true; } }
    use crate::dex_data::BoostID;
    let mut ready = false;
    for boost_id in BoostID::all() {
        let boost_value = boost.get(*boost_id);
        if boost_value > 0 {
            let current = current_boosts.get(*boost_id);
            current_boosts.set(*boost_id, current.saturating_add(boost_value));
            ready = true;
        }
    }

    // Store updated boosts and ready in effectState
    battle.with_effect_state(|state| {
        state.boosts = Some(current_boosts);
        if ready {
            state.ready = Some(true);
        }
    });

    EventResult::Continue
}

/// onAnySwitchIn() {
///     if (!this.effectState.ready) return;
///     (this.effectState.target as Pokemon).useItem();
/// }
pub fn on_any_switch_in(battle: &mut Battle) -> EventResult {
    // if (!this.effectState.ready) return;
    let ready = battle.with_effect_state_ref(|state| {
        state.ready.unwrap_or(false)
    }).unwrap_or(false);

    if !ready {
        return EventResult::Continue;
    }

    // (this.effectState.target as Pokemon).useItem();
    let target_pos = battle.with_effect_state_ref(|state| state.target).flatten();

    if let Some(pos) = target_pos {
        Pokemon::use_item(battle, pos, None, None);
    }

    EventResult::Continue
}

/// onAnyAfterMega() {
///     if (!this.effectState.ready) return;
///     (this.effectState.target as Pokemon).useItem();
/// }
pub fn on_any_after_mega(battle: &mut Battle) -> EventResult {
    // if (!this.effectState.ready) return;
    let ready = battle.with_effect_state_ref(|state| {
        state.ready.unwrap_or(false)
    }).unwrap_or(false);

    if !ready {
        return EventResult::Continue;
    }

    // (this.effectState.target as Pokemon).useItem();
    let target_pos = battle.with_effect_state_ref(|state| state.target).flatten();

    if let Some(pos) = target_pos {
        Pokemon::use_item(battle, pos, None, None);
    }

    EventResult::Continue
}

/// onAnyAfterTerastallization() {
///     if (!this.effectState.ready) return;
///     (this.effectState.target as Pokemon).useItem();
/// }
pub fn on_any_after_terastallization(battle: &mut Battle) -> EventResult {
    // if (!this.effectState.ready) return;
    let ready = battle.with_effect_state_ref(|state| {
        state.ready.unwrap_or(false)
    }).unwrap_or(false);

    if !ready {
        return EventResult::Continue;
    }

    // (this.effectState.target as Pokemon).useItem();
    let target_pos = battle.with_effect_state_ref(|state| state.target).flatten();

    if let Some(pos) = target_pos {
        Pokemon::use_item(battle, pos, None, None);
    }

    EventResult::Continue
}

/// onAnyAfterMove() {
///     if (!this.effectState.ready) return;
///     (this.effectState.target as Pokemon).useItem();
/// }
pub fn on_any_after_move(battle: &mut Battle) -> EventResult {
    // if (!this.effectState.ready) return;
    let ready = battle.with_effect_state_ref(|state| {
        state.ready.unwrap_or(false)
    }).unwrap_or(false);

    if !ready {
        return EventResult::Continue;
    }

    // (this.effectState.target as Pokemon).useItem();
    let target_pos = battle.with_effect_state_ref(|state| state.target).flatten();

    if let Some(pos) = target_pos {
        Pokemon::use_item(battle, pos, None, None);
    }

    EventResult::Continue
}

/// onResidual(pokemon) {
///     if (!this.effectState.ready) return;
///     (this.effectState.target as Pokemon).useItem();
/// }
pub fn on_residual(battle: &mut Battle, _pokemon_pos: (usize, usize)) -> EventResult {
    // if (!this.effectState.ready) return;
    let ready = battle.with_effect_state_ref(|state| {
        state.ready.unwrap_or(false)
    }).unwrap_or(false);

    if !ready {
        return EventResult::Continue;
    }

    // (this.effectState.target as Pokemon).useItem();
    let target_pos = battle.with_effect_state_ref(|state| state.target).flatten();

    if let Some(pos) = target_pos {
        Pokemon::use_item(battle, pos, None, None);
    }

    EventResult::Continue
}

/// onUse(pokemon) {
///     this.boost(this.effectState.boosts, pokemon);
/// }
pub fn on_use(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // this.boost(this.effectState.boosts, pokemon);
    let boosts_opt = battle.with_effect_state_ref(|state| {
        state.boosts
    }).flatten();

    if let Some(boosts) = boosts_opt {
        // Convert BoostsTable to Vec of (&str, i8) tuples for battle.boost()
        let mut boost_pairs: Vec<(&str, i8)> = Vec::new();

        use crate::dex_data::BoostID;
        for boost_id in BoostID::all() {
            let value = boosts.get(*boost_id);
            if value != 0 {
                let name = match boost_id {
                    BoostID::Atk => "atk",
                    BoostID::Def => "def",
                    BoostID::SpA => "spa",
                    BoostID::SpD => "spd",
                    BoostID::Spe => "spe",
                    BoostID::Accuracy => "accuracy",
                    BoostID::Evasion => "evasion",
                };
                boost_pairs.push((name, value));
            }
        }

        battle.boost(&boost_pairs, pokemon_pos, None, None, false, false);
    }

    EventResult::Continue
}

/// onEnd() {
///     delete this.effectState.boosts;
///     delete this.effectState.ready;
/// }
pub fn on_end(battle: &mut Battle, _pokemon_pos: (usize, usize)) -> EventResult {
    // delete this.effectState.boosts;
    // delete this.effectState.ready;
    battle.with_effect_state(|state| {
        state.boosts = None;
        state.ready = None;
    });

    EventResult::Continue
}
