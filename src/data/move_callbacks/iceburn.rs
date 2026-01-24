//! Ice Burn Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::{Battle, Effect};
use crate::dex_data::ID;
use crate::event::EventResult;
use crate::pokemon::Pokemon;

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
    let attacker = source_pos;
    let defender = target_pos;

    // if (attacker.removeVolatile(move.id)) {
    //     return;
    // }
    let move_id = match battle.active_move.as_ref() {
        Some(m) => m.clone(),
        None => return EventResult::Continue,
    };

    let removed = Pokemon::remove_volatile(battle, attacker, &move_id.borrow().id);
    if removed {
        return EventResult::Continue;
    }

    // this.add('-prepare', attacker, move.name);
    let move_name = battle
        .active_move
        .as_ref()
        .map(|m| m.borrow().name.clone())
        .unwrap_or_default();
    let attacker_ident = {
        let pokemon = match battle.pokemon_at(attacker.0, attacker.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.get_slot()
    };
    battle.add(
        "-prepare",
        &[attacker_ident.as_str().into(), move_name.into()],
    );

    // if (!this.runEvent('ChargeMove', attacker, defender, move)) {
    //     return;
    // }
    let charge_result = battle.run_event("ChargeMove", Some(crate::event::EventTarget::Pokemon(attacker)), defender, None, EventResult::Continue, false, false);
    // JavaScript: if (!result) return;
    // This checks if result is FALSY, not if it equals 0
    if !charge_result.is_truthy() {
        return EventResult::Continue;
    }

    // attacker.addVolatile('twoturnmove', defender);
    Pokemon::add_volatile(battle, attacker, ID::from("twoturnmove"), defender, Some(&Effect::move_(move_id.borrow().id.clone())), None, None);

    // return null; - prevents the move from executing during charge turn
    EventResult::Null
}
