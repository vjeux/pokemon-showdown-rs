//! Electro Shot Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::dex_data::ID;
use crate::event::EventResult;
use crate::Pokemon;

/// onTryMove(attacker, defender, move) {
///     if (attacker.removeVolatile(move.id)) {
///         return;
///     }
///     this.add('-prepare', attacker, move.name);
///     this.boost({ spa: 1 }, attacker, attacker, move);
///     if (['raindance', 'primordialsea'].includes(attacker.effectiveWeather())) {
///         this.attrLastMove('[still]');
///         this.addMove('-anim', attacker, move.name, defender);
///         return;
///     }
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

    // Get move ID
    let move_id = match &battle.active_move {
        Some(active_move) => active_move.id.clone(),
        None => return EventResult::Continue,
    };

    // if (attacker.removeVolatile(move.id)) {
    //     return;
    // }
    let removed = {
        Pokemon::remove_volatile(battle, attacker, &move_id)
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
        &[attacker_ident.as_str().into(), move_name.clone().into()],
    );

    // this.boost({ spa: 1 }, attacker, attacker, move);
    let boosts = [("spa", 1)];
    battle.boost(&boosts, attacker, Some(attacker), None, false, false);

    // if (['raindance', 'primordialsea'].includes(attacker.effectiveWeather())) {
    //     this.attrLastMove('[still]');
    //     this.addMove('-anim', attacker, move.name, defender);
    //     return;
    // }
    let effective_weather = {
        let field_weather = battle.field.weather.as_str();
        let attacker_pokemon = match battle.pokemon_at(attacker.0, attacker.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        attacker_pokemon.effective_weather(battle, field_weather)
    };

    if effective_weather == "raindance" || effective_weather == "primordialsea" {
        // this.attrLastMove('[still]');
        battle.attr_last_move(&["[still]"]);

        // this.addMove('-anim', attacker, move.name, defender);
        if let Some(defender) = defender {
            let defender_ident = {
                let defender_pokemon = match battle.pokemon_at(defender.0, defender.1) {
                    Some(p) => p,
                    None => return EventResult::Continue,
                };
                defender_pokemon.get_slot()
            };

            // Need to recreate attacker_ident since it was moved
            let attacker_ident2 = {
                let attacker_pokemon = match battle.pokemon_at(attacker.0, attacker.1) {
                    Some(p) => p,
                    None => return EventResult::Continue,
                };
                attacker_pokemon.get_slot()
            };

            battle.add(
                "-anim",
                &[
                    attacker_ident2.as_str().into(),
                    move_name.into(),
                    defender_ident.as_str().into(),
                ],
            );
        } else {
            // Need to recreate attacker_ident since it was moved
            let attacker_ident2 = {
                let attacker_pokemon = match battle.pokemon_at(attacker.0, attacker.1) {
                    Some(p) => p,
                    None => return EventResult::Continue,
                };
                attacker_pokemon.get_slot()
            };

            battle.add(
                "-anim",
                &[attacker_ident2.as_str().into(), move_name.into()],
            );
        }

        // return;
        return EventResult::Continue;
    }

    // if (!this.runEvent('ChargeMove', attacker, defender, move)) {
    //     return;
    // }
    let charge_result = battle.run_event("ChargeMove", Some(crate::event::EventTarget::Pokemon(attacker)), defender, None, EventResult::Continue, false, false);
    // JavaScript: if (!result) return;
    // This checks if result is FALSY, not if it equals 0
    if !charge_result.is_truthy() {
        // return;
        return EventResult::Continue;
    }

    // attacker.addVolatile('twoturnmove', defender);
    Pokemon::add_volatile(battle, attacker, ID::from("twoturnmove"), defender, Some(&move_id), None, None);

    // return null;
    EventResult::Stop
}
