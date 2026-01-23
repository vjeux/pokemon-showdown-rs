//! Focus Punch Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// priorityChargeCallback(pokemon) {
///     pokemon.addVolatile('focuspunch');
/// }
pub fn priority_charge_callback(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    use crate::dex_data::ID;
    use crate::pokemon::Pokemon;

    // pokemon.addVolatile('focuspunch');
    Pokemon::add_volatile(battle, pokemon_pos, ID::from("focuspunch"), None, None, None, None);

    EventResult::Continue
}

/// beforeMoveCallback(pokemon) {
///     if (pokemon.volatiles['focuspunch']?.lostFocus) {
///         this.add('cant', pokemon, 'Focus Punch', 'Focus Punch');
///         return true;
///     }
/// }
pub fn before_move_callback(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    use crate::dex_data::ID;

    let pokemon = pokemon_pos;

    // if (pokemon.volatiles['focuspunch']?.lostFocus) {
    let lost_focus = {
        let pokemon_ref = match battle.pokemon_at(pokemon.0, pokemon.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        debug_elog!("[FOCUSPUNCH before_move_callback] pokemon_pos={:?}, has volatile={}, volatiles={:?}",
            pokemon_pos,
            pokemon_ref.volatiles.contains_key(&ID::from("focuspunch")),
            pokemon_ref.volatiles.keys().map(|k| k.as_str()).collect::<Vec<_>>());

        if let Some(volatile_state) = pokemon_ref.volatiles.get(&ID::from("focuspunch")) {
            debug_elog!("[FOCUSPUNCH before_move_callback] volatile state: lost_focus={:?}", volatile_state.lost_focus);
            volatile_state.lost_focus.unwrap_or(false)
        } else {
            false
        }
    };

    debug_elog!("[FOCUSPUNCH before_move_callback] lost_focus={}", lost_focus);

    if lost_focus {
        // this.add('cant', pokemon, 'Focus Punch', 'Focus Punch');
        let pokemon_arg = {
            let pokemon_ref = match battle.pokemon_at(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon_ref.get_slot()
        };

        battle.add(
            "cant",
            &[
                crate::battle::Arg::from(pokemon_arg),
                crate::battle::Arg::from("Focus Punch"),
                crate::battle::Arg::from("Focus Punch"),
            ],
        );

        // return true;
        return EventResult::Boolean(true);
    }

    EventResult::Continue
}

pub mod condition {
    use super::*;

    /// onStart(pokemon) {
    ///     this.add('-singleturn', pokemon, 'move: Focus Punch');
    /// }
    pub fn on_start(
        battle: &mut Battle,
        pokemon_pos: (usize, usize),
        _source_pos: Option<(usize, usize)>,
        _effect: Option<&crate::battle::Effect>,
    ) -> EventResult {
        let pokemon = pokemon_pos;

        // this.add('-singleturn', pokemon, 'move: Focus Punch');
        let pokemon_arg = {
            let pokemon_ref = match battle.pokemon_at(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon_ref.get_slot()
        };

        battle.add(
            "-singleturn",
            &[
                crate::battle::Arg::from(pokemon_arg),
                crate::battle::Arg::from("move: Focus Punch"),
            ],
        );

        EventResult::Continue
    }

    /// onHit(pokemon, source, move) {
    ///     if (move.category !== 'Status') {
    ///         this.effectState.lostFocus = true;
    ///     }
    /// }
    pub fn on_hit(
        battle: &mut Battle,
        pokemon_pos: (usize, usize),
        _target_pos: Option<(usize, usize)>,
    ) -> EventResult {
        // DEBUG: Log effect context
        debug_elog!("[FOCUSPUNCH on_hit] pokemon_pos={:?}, effect={:?}",
            pokemon_pos,
            battle.effect.as_ref().map(|e| (e.id.as_str(), e.effect_type, e.effect_holder.clone())));

        // if (move.category !== 'Status') {
        let move_category = match &battle.active_move {
            Some(active_move) => active_move.category.as_str(),
            None => return EventResult::Continue,
        };

        debug_elog!("[FOCUSPUNCH on_hit] move_category={}", move_category);

        if move_category != "Status" {
            // this.effectState.lostFocus = true;
            let result = battle.with_effect_state(|state| {
                state.lost_focus = Some(true);
                debug_elog!("[FOCUSPUNCH on_hit] Set lost_focus=true in effect_state");
            });
            debug_elog!("[FOCUSPUNCH on_hit] with_effect_state returned: {:?}", result.is_some());
        }

        EventResult::Continue
    }

    /// onTryAddVolatile(status, pokemon) {
    ///     if (status.id === 'flinch') return null;
    /// }
    pub fn on_try_add_volatile(
        _battle: &mut Battle,
        status: Option<&str>,
        _pokemon_pos: (usize, usize),
    ) -> EventResult {
        // if (status.id === 'flinch') return null; - prevents the volatile from being added
        if let Some(status_id) = status {
            if status_id == "flinch" {
                return EventResult::Null;
            }
        }

        EventResult::Continue
    }
}
