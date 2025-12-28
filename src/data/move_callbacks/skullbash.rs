//! Skull Bash Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onTryMove(attacker, defender, move) {
///     if (attacker.removeVolatile(move.id)) {
///         return;
///     }
///     this.add('-prepare', attacker, move.name);
///     this.boost({ def: 1 }, attacker, attacker, move);
///     if (!this.runEvent('ChargeMove', attacker, defender, move)) {
///         return;
///     }
///     attacker.addVolatile('twoturnmove', defender);
///     return null;
/// }
pub fn on_try_move(battle: &mut Battle, source_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    use crate::dex_data::ID;

    // onTryMove(attacker, defender, move) {
    //     if (attacker.removeVolatile(move.id)) {
    //         return;
    //     }
    //     this.add('-prepare', attacker, move.name);
    //     this.boost({ def: 1 }, attacker, attacker, move);
    //     if (!this.runEvent('ChargeMove', attacker, defender, move)) {
    //         return;
    //     }
    //     attacker.addVolatile('twoturnmove', defender);
    //     return null;
    // }
    let attacker = source_pos;
    let defender = target_pos;

    // if (attacker.removeVolatile(move.id)) {
    //     return;
    // }
    let move_id = {
        let active_move = match &battle.active_move {
            Some(active_move) => active_move,
            None => return EventResult::Continue,
        };
        active_move.id.clone()
    };

    let removed = battle.remove_volatile(&move_id, attacker);
    if removed {
        return EventResult::Continue;
    }

    // this.add('-prepare', attacker, move.name);
    let (attacker_arg, move_name) = {
        let attacker_pokemon = match battle.pokemon_at(attacker.0, attacker.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        let active_move = match &battle.active_move {
            Some(active_move) => active_move,
            None => return EventResult::Continue,
        };
        (crate::battle::Arg::from(attacker_pokemon), active_move.name.clone())
    };

    battle.add("-prepare", &[
        attacker_arg,
        move_name.into(),
    ]);

    // this.boost({ def: 1 }, attacker, attacker, move);
    battle.boost(&[("def", 1)], attacker, Some(attacker), Some(move_id.as_str()));

    // if (!this.runEvent('ChargeMove', attacker, defender, move)) {
    //     return;
    // }
    let charge_result = battle.run_event("ChargeMove", Some(attacker), defender, None, None);
    if charge_result == Some(0) {
        return EventResult::Continue;
    }

    // attacker.addVolatile('twoturnmove', defender);
    battle.add_volatile(&ID::from("twoturnmove"), attacker, defender, None);

    // return null;
    EventResult::Stop
}

