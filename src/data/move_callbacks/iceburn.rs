//! Ice Burn Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
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

    let removed = {
        let pokemon = match battle.pokemon_at_mut(attacker.0, attacker.1) {
            Some(p) => p,

            None => return EventResult::Continue,
        };

        pokemon.remove_volatile(&move_id.id)
    };
    if removed {
        return EventResult::Continue;
    }

    // this.add('-prepare', attacker, move.name);
    let move_name = battle
        .active_move
        .as_ref()
        .map(|m| m.name.clone())
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
    let charge_result = battle.run_event("ChargeMove", Some(attacker), defender, None, None);
    if charge_result.unwrap_or(0) == 0 {
        return EventResult::Continue;
    }

    // attacker.addVolatile('twoturnmove', defender);
    Pokemon::add_volatile(battle, attacker, ID::from("twoturnmove"), defender);

    // return null;
    EventResult::Stop
}
