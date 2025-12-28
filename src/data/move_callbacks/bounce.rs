//! Bounce Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::{Battle, Arg};
use crate::event::EventResult;
use crate::dex_data::ID;

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
pub fn on_try_move(battle: &mut Battle, source_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    // Get the active move
    let move_id = match &battle.active_move {
        Some(active_move) => active_move.id.clone(),
        None => return EventResult::Continue,
    };

    // if (attacker.removeVolatile(move.id)) {
    //     return;
    // }
    {
        let attacker = match battle.pokemon_at_mut(source_pos.0, source_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        if attacker.volatiles.contains_key(&move_id) {
            attacker.remove_volatile(&move_id);
            // return;
            return EventResult::Continue;
        }
    }

    // this.add('-prepare', attacker, move.name);
    let (attacker_arg, move_name) = {
        let attacker = match battle.pokemon_at(source_pos.0, source_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        let move_data = battle.dex.get_move_by_id(&move_id);
        let move_name = move_data.map(|m| m.name.clone()).unwrap_or_else(|| move_id.to_string());

        (Arg::from(attacker), move_name)
    };

    battle.add("-prepare", &[attacker_arg, move_name.into()]);

    // if (!this.runEvent('ChargeMove', attacker, defender, move)) {
    //     return;
    // }
    if battle.run_event("ChargeMove", Some(source_pos), target_pos, None, None).unwrap_or(0) == 0 {
        // return;
        return EventResult::Continue;
    }

    // attacker.addVolatile('twoturnmove', defender);
    let attacker = match battle.pokemon_at_mut(source_pos.0, source_pos.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };

    attacker.add_volatile(ID::from("twoturnmove"));

    // return null;
    EventResult::Stop
}

pub mod condition {
    use super::*;

    /// onInvulnerability(target, source, move) {
    ///     if (['gust', 'twister', 'skyuppercut', 'thunder', 'hurricane', 'smackdown', 'thousandarrows'].includes(move.id)) {
    ///         return;
    ///     }
    ///     return false;
    /// }
    pub fn on_invulnerability(battle: &mut Battle, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, move_id: &str) -> EventResult {
        // if (['gust', 'twister', 'skyuppercut', 'thunder', 'hurricane', 'smackdown', 'thousandarrows'].includes(move.id)) {
        //     return;
        // }
        if move_id == "gust" || move_id == "twister" || move_id == "skyuppercut"
            || move_id == "thunder" || move_id == "hurricane" || move_id == "smackdown"
            || move_id == "thousandarrows" {
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
    pub fn on_source_base_power(battle: &mut Battle, base_power: i32, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, move_id: &str) -> EventResult {
        // if (move.id === 'gust' || move.id === 'twister') {
        //     return this.chainModify(2);
        // }
        if move_id == "gust" || move_id == "twister" {
            let result = battle.chain_modify(2.0);
            return EventResult::Number(result);
        }

        EventResult::Continue
    }
}
