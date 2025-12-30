//! Geomancy Move
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

    // Get move ID
    let move_id = match &battle.active_move {
        Some(active_move) => active_move.id.clone(),
        None => return EventResult::Continue,
    };

    // if (attacker.removeVolatile(move.id)) {
    //     return;
    // }
    let removed = {
        let attacker_pokemon = match battle.pokemon_at_mut(attacker.0, attacker.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        attacker_pokemon.remove_volatile(&move_id)
    };

    if removed {
        // return;
        return EventResult::Continue;
    }

    // this.add('-prepare', attacker, move.name);
    let (attacker_ident, move_name) = {
        let attacker_pokemon = match battle.pokemon_at(attacker.0, attacker.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        let move_data = battle.dex.moves().get_by_id(&move_id);
        let move_name = move_data
            .map(|m| m.name.clone())
            .unwrap_or_else(|| move_id.to_string());

        (attacker_pokemon.get_slot(), move_name)
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
        // return;
        return EventResult::Continue;
    }

    // attacker.addVolatile('twoturnmove', defender);
    let attacker_pokemon = match battle.pokemon_at_mut(attacker.0, attacker.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };
    attacker_pokemon.add_volatile(ID::from("twoturnmove"));

    // return null;
    EventResult::Stop
}
