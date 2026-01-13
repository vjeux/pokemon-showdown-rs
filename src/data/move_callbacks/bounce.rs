//! Bounce Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::{Battle, Effect};
use crate::dex_data::ID;
use crate::event::EventResult;
use crate::Pokemon;

/// onTryMove(attacker, defender, move) {
///     if (attacker.removeVolatile(move.id)) {
///         return;
///     }
///     this.add('-prepare', attacker, move.name);
///     if (!this.runEvent('ChargeMove', attacker, defender, move)) {
///         return;
///     }
///     attacker.addVolatile('twoturnmove', defender);
///     return null;
/// }
pub fn on_try_move(
    battle: &mut Battle,
    source_pos: (usize, usize),
    target_pos: Option<(usize, usize)>,
) -> EventResult {
    // Get the active move
    let move_id = match &battle.active_move {
        Some(active_move) => active_move.id.clone(),
        None => return EventResult::Continue,
    };

    // if (attacker.removeVolatile(move.id)) {
    //     return;
    // }
    let has_volatile = {
        let attacker = match battle.pokemon_at(source_pos.0, source_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        attacker.volatiles.contains_key(&move_id)
    };

    if has_volatile {
        Pokemon::remove_volatile(battle, source_pos, &move_id);
        return EventResult::Continue;
    }

    // this.add('-prepare', attacker, move.name);
    let (attacker_ident, move_name) = {
        let attacker = match battle.pokemon_at(source_pos.0, source_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        let move_data = battle.dex.moves().get_by_id(&move_id);
        let move_name = move_data
            .map(|m| m.name.clone())
            .unwrap_or_else(|| move_id.to_string());

        (attacker.get_slot(), move_name)
    };

    battle.add(
        "-prepare",
        &[attacker_ident.as_str().into(), move_name.into()],
    );

    // if (!this.runEvent('ChargeMove', attacker, defender, move)) {
    //     return;
    // }
    match battle.run_event("ChargeMove", Some(crate::event::EventTarget::Pokemon(source_pos)), target_pos, None, EventResult::Continue, false, false) {
        EventResult::Boolean(false) | EventResult::Null => {
            // return;
            return EventResult::Continue;
        }
        _ => {}
    }

    // attacker.addVolatile('twoturnmove', defender);
    Pokemon::add_volatile(battle, source_pos, ID::from("twoturnmove"), target_pos, Some(&Effect::move_(move_id.clone())), None, None);

    // return null; - prevents the move from executing during charge turn
    EventResult::Null
}

pub mod condition {
    use super::*;

    /// onInvulnerability(target, source, move) {
    ///     if (['gust', 'twister', 'skyuppercut', 'thunder', 'hurricane', 'smackdown', 'thousandarrows'].includes(move.id)) {
    ///         return;
    ///     }
    ///     return false;
    /// }
    pub fn on_invulnerability(
        _battle: &mut Battle,
        _target_pos: Option<(usize, usize)>,
        _source_pos: Option<(usize, usize)>,
        active_move: Option<&crate::battle_actions::ActiveMove>,
    ) -> EventResult {
        let move_id = active_move.map(|m| m.id.as_str()).unwrap_or("");
        // if (['gust', 'twister', 'skyuppercut', 'thunder', 'hurricane', 'smackdown', 'thousandarrows'].includes(move.id)) {
        //     return;
        // }
        if move_id == "gust"
            || move_id == "twister"
            || move_id == "skyuppercut"
            || move_id == "thunder"
            || move_id == "hurricane"
            || move_id == "smackdown"
            || move_id == "thousandarrows"
        {
            // return;
            return EventResult::Continue;
        }

        // return false;
        EventResult::Boolean(false)
    }

    /// onSourceBasePower(basePower, target, source, move) {
    ///     if (move.id === 'gust' || move.id === 'twister') {
    ///         return this.chainModify(2);
    ///     }
    /// }
    pub fn on_source_base_power(
        battle: &mut Battle,
        _base_power: i32,
        _target_pos: Option<(usize, usize)>,
        _source_pos: Option<(usize, usize)>,
        active_move: Option<&crate::battle_actions::ActiveMove>,
    ) -> EventResult {
        let move_id = active_move.map(|m| m.id.as_str()).unwrap_or("");
        // if (move.id === 'gust' || move.id === 'twister') {
        //     return this.chainModify(2);
        // }
        if move_id == "gust" || move_id == "twister" {
            // chain_modify accumulates the modifier - we return Continue, not the result
            battle.chain_modify(2.0);
        }

        EventResult::Continue
    }
}
