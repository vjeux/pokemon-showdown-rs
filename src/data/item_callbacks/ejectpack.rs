//! Eject Pack Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::event::EventResult;
use crate::Pokemon;

/// onAfterBoost(boost, pokemon) {
///     if (this.effectState.eject || this.activeMove?.id === 'partingshot') return;
///     let i: BoostID;
///     for (i in boost) {
///         if (boost[i]! < 0) {
///             this.effectState.eject = true;
///             break;
///         }
///     }
/// }
pub fn on_after_boost(
    battle: &mut Battle,
    _pokemon_pos: (usize, usize),
    boost: &crate::dex_data::BoostsTable,
) -> EventResult {
    // if (this.effectState.eject || this.activeMove?.id === 'partingshot') return;
    let eject_already_set = battle.with_effect_state_ref(|state| {
        state.eject.unwrap_or(false)
    }).unwrap_or(false);

    let active_move_is_parting_shot = battle
        .active_move
        .as_ref()
        .map(|m| m.id.as_str() == "partingshot")
        .unwrap_or(false);

    if eject_already_set || active_move_is_parting_shot {
        return EventResult::Continue;
    }

    // for (i in boost) { if (boost[i]! < 0) { this.effectState.eject = true; break; } }
    use crate::dex_data::BoostID;
    let mut has_negative_boost = false;
    for boost_id in BoostID::all() {
        if boost.get(*boost_id) < 0 {
            has_negative_boost = true;
            break;
        }
    }

    // this.effectState.eject = true;
    if has_negative_boost {
        battle.with_effect_state(|state| {
            state.eject = Some(true);
        });
    }

    EventResult::Continue
}

/// onAnySwitchIn() {
///     if (!this.effectState.eject) return;
///     (this.effectState.target as Pokemon).useItem();
/// }
pub fn on_any_switch_in(battle: &mut Battle) -> EventResult {
    // if (!this.effectState.eject) return;
    let eject_set = battle.with_effect_state_ref(|state| {
        state.eject.unwrap_or(false)
    }).unwrap_or(false);

    if !eject_set {
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
///     if (!this.effectState.eject) return;
///     (this.effectState.target as Pokemon).useItem();
/// }
pub fn on_any_after_mega(battle: &mut Battle) -> EventResult {
    // if (!this.effectState.eject) return;
    let eject_set = battle.with_effect_state_ref(|state| {
        state.eject.unwrap_or(false)
    }).unwrap_or(false);

    if !eject_set {
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
///     if (!this.effectState.eject) return;
///     (this.effectState.target as Pokemon).useItem();
/// }
pub fn on_any_after_move(battle: &mut Battle) -> EventResult {
    // if (!this.effectState.eject) return;
    let eject_set = battle.with_effect_state_ref(|state| {
        state.eject.unwrap_or(false)
    }).unwrap_or(false);

    if !eject_set {
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
///     if (!this.effectState.eject) return;
///     (this.effectState.target as Pokemon).useItem();
/// }
pub fn on_residual(battle: &mut Battle, _pokemon_pos: (usize, usize)) -> EventResult {
    // if (!this.effectState.eject) return;
    let eject_set = battle.with_effect_state_ref(|state| {
        state.eject.unwrap_or(false)
    }).unwrap_or(false);

    if !eject_set {
        return EventResult::Continue;
    }

    // (this.effectState.target as Pokemon).useItem();
    let target_pos = battle.with_effect_state_ref(|state| state.target).flatten();

    if let Some(pos) = target_pos {
        Pokemon::use_item(battle, pos, None, None);
    }

    EventResult::Continue
}

/// onUseItem(item, pokemon) {
///     if (!this.canSwitch(pokemon.side)) return false;
///     if (pokemon.volatiles['commanding'] || pokemon.volatiles['commanded']) return false;
///     for (const active of this.getAllActive()) {
///         if (active.switchFlag === true) return false;
///     }
///     return true;
/// }
pub fn on_use_item(battle: &mut Battle, _item_id: &str, pokemon_pos: (usize, usize)) -> EventResult {
    // if (!this.canSwitch(pokemon.side)) return false;
    let side_idx = pokemon_pos.0;
    let can_switch_count = battle.can_switch(side_idx);
    if can_switch_count == 0 {
        return EventResult::Boolean(false);
    }

    // if (pokemon.volatiles['commanding'] || pokemon.volatiles['commanded']) return false;
    let has_commanding_volatiles = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Boolean(false),
        };

        use crate::dex_data::ID;
        pokemon.volatiles.contains_key(&ID::from("commanding"))
            || pokemon.volatiles.contains_key(&ID::from("commanded"))
    };

    if has_commanding_volatiles {
        return EventResult::Boolean(false);
    }

    // for (const active of this.getAllActive()) { if (active.switchFlag === true) return false; }
    // IMPORTANT: JavaScript checks `switchFlag === true` (strict equality with boolean true)
    // - switchFlag = true (boolean) -> represented in Rust as Some("") (empty string)
    // - switchFlag = "flipturn" (string from selfSwitch) -> represented in Rust as Some("flipturn")
    // The JS check only matches boolean true, NOT string values like "flipturn"
    // So in Rust, we only match Some("") (empty string), not any Some value
    let all_active_positions = battle.get_all_active(false);
    for pos in all_active_positions {
        let pokemon = match battle.pokemon_at(pos.0, pos.1) {
            Some(p) => p,
            None => continue,
        };
        if pokemon.switch_flag.as_deref() == Some("") {
            return EventResult::Boolean(false);
        }
    }

    // return true;
    EventResult::Boolean(true)
}

/// onUse(pokemon) {
///     pokemon.switchFlag = true;
/// }
pub fn on_use(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // pokemon.switchFlag = true;
    let pokemon_mut = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };
    pokemon_mut.switch_flag = Some(String::new());

    EventResult::Continue
}

/// onEnd() {
///     delete this.effectState.eject;
/// }
pub fn on_end(battle: &mut Battle, _pokemon_pos: (usize, usize)) -> EventResult {
    // delete this.effectState.eject;
    battle.with_effect_state(|state| {
        state.eject = None;
    });

    EventResult::Continue
}
