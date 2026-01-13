//! Meteor Beam Move
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
///     this.boost({ spa: 1 }, attacker, attacker, move);
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
    eprintln!("[METEORBEAM_ON_TRY_MOVE] turn={}, source=({},{}), target={:?}",
        battle.turn, source_pos.0, source_pos.1, target_pos);

    let attacker = source_pos;
    let defender = target_pos;

    // Get move ID
    let move_id = match &battle.active_move {
        Some(active_move) => active_move.id.clone(),
        None => {
            eprintln!("[METEORBEAM_ON_TRY_MOVE] No active_move, returning Continue");
            return EventResult::Continue;
        }
    };

    // if (attacker.removeVolatile(move.id)) {
    //     return;
    // }
    let removed = {
        Pokemon::remove_volatile(battle, attacker, &move_id)
    };

    eprintln!("[METEORBEAM_ON_TRY_MOVE] move_id={}, removed={}", move_id.as_str(), removed);

    if removed {
        // return;
        eprintln!("[METEORBEAM_ON_TRY_MOVE] Volatile removed, returning Continue (2nd turn execution)");
        return EventResult::Continue;
    }

    eprintln!("[METEORBEAM_ON_TRY_MOVE] Past removeVolatile check, preparing charge turn");

    // this.add('-prepare', attacker, move.name);
    let (attacker_arg, move_name) = {
        let attacker_pokemon = match battle.pokemon_at(attacker.0, attacker.1) {
            Some(p) => p,
            None => {
                eprintln!("[METEORBEAM_ON_TRY_MOVE] Attacker pokemon not found, returning Continue");
                return EventResult::Continue;
            }
        };
        let move_data = battle.dex.moves().get_by_id(&move_id);
        let move_name = move_data
            .map(|m| m.name.clone())
            .unwrap_or_else(|| move_id.to_string());

        (attacker_pokemon.get_slot(), move_name)
    };

    eprintln!("[METEORBEAM_ON_TRY_MOVE] Adding -prepare message");
    battle.add("-prepare", &[attacker_arg.into(), move_name.into()]);

    // this.boost({ spa: 1 }, attacker, attacker, move);
    eprintln!("[METEORBEAM_ON_TRY_MOVE] Calling boost for SpA+1");
    battle.boost(
        &[("spa", 1)],
        attacker,
        Some(attacker),
        Some(&move_id.to_string()),
        false,
        false,
    );

    // if (!this.runEvent('ChargeMove', attacker, defender, move)) {
    //     return;
    // }
    eprintln!("[METEORBEAM_ON_TRY_MOVE] Running ChargeMove event");
    let charge_result = battle.run_event("ChargeMove", Some(crate::event::EventTarget::Pokemon(attacker)), defender, None, EventResult::Continue, false, false);
    eprintln!("[METEORBEAM_ON_TRY_MOVE] ChargeMove result: {:?}", charge_result);
    // JavaScript: if (!result) return;
    // This checks if result is FALSY, not if it equals 0
    if !charge_result.is_truthy() {
        eprintln!("[METEORBEAM_ON_TRY_MOVE] ChargeMove returned falsy, returning Continue");
        return EventResult::Continue;
    }

    // attacker.addVolatile('twoturnmove', defender);
    eprintln!("[METEORBEAM_ON_TRY_MOVE] About to add twoturnmove volatile");
    Pokemon::add_volatile(battle, attacker, ID::from("twoturnmove"), defender, Some(&Effect::move_(move_id)), None, None);
    eprintln!("[METEORBEAM_ON_TRY_MOVE] Added twoturnmove volatile, returning Stop (charge turn)");

    // return null; - prevents the move from executing during charge turn
    EventResult::Null
}
