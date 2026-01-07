//! Phantom Force Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
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
    use crate::dex_data::ID;

    eprintln!("[PHANTOMFORCE_ONTRYMOVE] turn={}, source={:?}, target={:?}", battle.turn, source_pos, target_pos);

    let attacker = source_pos;
    let defender = target_pos;

    // if (attacker.removeVolatile(move.id)) {
    let move_id = {
        match &battle.active_move {
            Some(active_move) => active_move.id.clone(),
            None => return EventResult::Continue,
        }
    };

    eprintln!("[PHANTOMFORCE_ONTRYMOVE] move_id={}", move_id.as_str());

    let removed = {
        Pokemon::remove_volatile(battle, attacker, &move_id)
    };

    eprintln!("[PHANTOMFORCE_ONTRYMOVE] removed={}", removed);

    if removed {
        // return;
        eprintln!("[PHANTOMFORCE_ONTRYMOVE] Volatile was removed, this is turn 2 of the move");
        return EventResult::Continue;
    }

    // this.add('-prepare', attacker, move.name);
    let (attacker_arg, move_name) = {
        let attacker_pokemon = match battle.pokemon_at(attacker.0, attacker.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        let attacker_arg = attacker_pokemon.get_slot();

        let move_data = battle.dex.moves().get_by_id(&move_id);
        let move_name = move_data
            .map(|m| m.name.clone())
            .unwrap_or_else(|| move_id.to_string());

        (attacker_arg, move_name)
    };

    battle.add("-prepare", &[attacker_arg.into(), move_name.into()]);

    // if (!this.runEvent('ChargeMove', attacker, defender, move)) {
    let charge_result =
        battle.run_event("ChargeMove", Some(crate::event::EventTarget::Pokemon(attacker)), defender, Some(&move_id), EventResult::Continue, false, false);

    eprintln!("[PHANTOMFORCE_ONTRYMOVE] charge_result={:?}", charge_result);

    if matches!(charge_result, EventResult::Number(0)) {
        // return;
        eprintln!("[PHANTOMFORCE_ONTRYMOVE] ChargeMove returned 0, aborting");
        return EventResult::Continue;
    }

    // attacker.addVolatile('twoturnmove', defender);
    eprintln!("[PHANTOMFORCE_ONTRYMOVE] About to add twoturnmove volatile");
    Pokemon::add_volatile(battle, attacker, ID::from("twoturnmove"), defender, Some(&move_id), None, None);
    eprintln!("[PHANTOMFORCE_ONTRYMOVE] Added twoturnmove volatile");

    // return null;
    eprintln!("[PHANTOMFORCE_ONTRYMOVE] Returning Stop");
    EventResult::Stop
}
