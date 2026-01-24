//! Shadow Force Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::{Battle, Effect};
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
        active_move.borrow().id.clone()
    };

    let removed = {
        let pokemon = match battle.pokemon_at(attacker.0, attacker.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.has_volatile(&move_id)
    };

    if removed {
        Pokemon::remove_volatile(battle, attacker, &move_id);
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
        (attacker_pokemon.get_slot(), active_move.borrow().name.clone())
    };

    battle.add("-prepare", &[attacker_arg.into(), move_name.into()]);

    // if (!this.runEvent('ChargeMove', attacker, defender, move)) {
    //     return;
    // }
    let charge_result = battle.run_event("ChargeMove", Some(crate::event::EventTarget::Pokemon(attacker)), defender, None, EventResult::Continue, false, false);
    // runEvent returns false/null when the charge should be skipped (e.g., Power Herb)
    if matches!(charge_result, EventResult::Boolean(false) | EventResult::Null) {
        return EventResult::Continue;
    }

    // attacker.addVolatile('twoturnmove', defender);
    Pokemon::add_volatile(battle, attacker, ID::from("twoturnmove"), defender, Some(&Effect::move_(move_id)), None, None);

    // return null; - prevents the move from executing during charge turn
    EventResult::Null
}
