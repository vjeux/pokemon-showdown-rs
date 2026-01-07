//! White Herb Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::event::EventResult;
use crate::Pokemon;

/// onStart(pokemon) {
///     this.effectState.boosts = {} as SparseBoostsTable;
///     let ready = false;
///     let i: BoostID;
///     for (i in pokemon.boosts) {
///         if (pokemon.boosts[i] < 0) {
///             ready = true;
///             this.effectState.boosts[i] = 0;
///         }
///     }
///     if (ready) (this.effectState.target as Pokemon).useItem();
///     delete this.effectState.boosts;
/// }
pub fn on_start(battle: &mut Battle, target_pos: Option<(usize, usize)>) -> EventResult {
    // if (!pokemon) return;
    let pokemon_pos = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // this.effectState.boosts = {} as SparseBoostsTable;
    // let ready = false;
    // for (i in pokemon.boosts) { if (pokemon.boosts[i] < 0) { ready = true; this.effectState.boosts[i] = 0; } }
    let (boosts_to_clear, ready) = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        use crate::dex_data::BoostID;
        let mut boosts_to_clear = crate::dex_data::BoostsTable::default();
        let mut ready = false;

        for boost_id in BoostID::all() {
            let boost_value = pokemon.boosts.get(*boost_id);
            if boost_value < 0 {
                ready = true;
                boosts_to_clear.set(*boost_id, 0);
            }
        }

        (boosts_to_clear, ready)
    };

    // Store boosts in effectState
    battle.with_effect_state(|state| {
        state.boosts = Some(boosts_to_clear);
    });

    // if (ready) (this.effectState.target as Pokemon).useItem();
    if ready {
        let _pokemon_mut = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        Pokemon::use_item(battle, pokemon_pos, None, None);
    }

    // delete this.effectState.boosts;
    battle.with_effect_state(|state| {
        state.boosts = None;
    });

    EventResult::Continue
}

/// onAnySwitchIn() {
///     ((this.effect as any).onStart as (p: Pokemon) => void).call(this, this.effectState.target);
/// }
pub fn on_any_switch_in(battle: &mut Battle) -> EventResult {
    // Get effectState.target
    let target_pos = battle.with_effect_state_ref(|state| state.target).flatten();

    // Call onStart with effectState.target
    on_start(battle, target_pos)
}

/// onAnyAfterMega() {
///     ((this.effect as any).onStart as (p: Pokemon) => void).call(this, this.effectState.target);
/// }
pub fn on_any_after_mega(battle: &mut Battle) -> EventResult {
    // Get effectState.target
    let target_pos = battle.with_effect_state_ref(|state| state.target).flatten();

    // Call onStart with effectState.target
    on_start(battle, target_pos)
}

/// onAnyAfterMove() {
///     ((this.effect as any).onStart as (p: Pokemon) => void).call(this, this.effectState.target);
/// }
pub fn on_any_after_move(battle: &mut Battle) -> EventResult {
    // Get effectState.target
    let target_pos = battle.with_effect_state_ref(|state| state.target).flatten();

    // Call onStart with effectState.target
    on_start(battle, target_pos)
}

/// onResidual(pokemon) {
///     ((this.effect as any).onStart as (p: Pokemon) => void).call(this, pokemon);
/// }
pub fn on_residual(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // Call onStart with pokemon parameter
    on_start(battle, Some(pokemon_pos))
}

/// onUse(pokemon) {
///     pokemon.setBoost(this.effectState.boosts);
///     this.add('-clearnegativeboost', pokemon, '[silent]');
/// }
pub fn on_use(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // pokemon.setBoost(this.effectState.boosts);
    let boosts_opt = battle.with_effect_state_ref(|state| {
        state.boosts
    }).flatten();

    if let Some(boosts) = boosts_opt {
        let pokemon_mut = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        use crate::dex_data::BoostID;
        let mut boost_map = std::collections::HashMap::new();
        for boost_id in BoostID::all() {
            let value = boosts.get(*boost_id);
            if value != 0 {
                boost_map.insert(*boost_id, value);
            }
        }
        pokemon_mut.set_boost(boost_map);
    }

    // this.add('-clearnegativeboost', pokemon, '[silent]');
    let pokemon_slot = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.get_slot()
    };

    battle.add(
        "-clearnegativeboost",
        &[
            crate::battle::Arg::from(pokemon_slot),
            crate::battle::Arg::from("[silent]"),
        ],
    );

    EventResult::Continue
}
