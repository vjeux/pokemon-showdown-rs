//! Electro Shot Move
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
pub fn on_try_move(battle: &mut Battle, source_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    use crate::dex_data::ID;

    let attacker = source_pos;
    let defender = target_pos;

    // Get move ID
    let move_id = match &battle.current_move {
        Some(id) => id.clone(),
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
    let (attacker_arg, move_name) = {
        let attacker_pokemon = match battle.pokemon_at(attacker.0, attacker.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        let move_data = battle.dex.get_move_by_id(&move_id);
        let move_name = move_data.map(|m| m.name.clone()).unwrap_or_else(|| move_id.to_string());

        (crate::battle::Arg::from(attacker_pokemon), move_name)
    };

    battle.add("-prepare", &[attacker_arg.clone(), move_name.clone().into()]);

    // this.boost({ spa: 1 }, attacker, attacker, move);
    let mut boosts = std::collections::HashMap::new();
    boosts.insert("spa".to_string(), 1);
    battle.boost(&boosts, attacker);

    // if (['raindance', 'primordialsea'].includes(attacker.effectiveWeather())) {
    //     this.attrLastMove('[still]');
    //     this.addMove('-anim', attacker, move.name, defender);
    //     return;
    // }
    let effective_weather = {
        let attacker_pokemon = match battle.pokemon_at(attacker.0, attacker.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        attacker_pokemon.effective_weather(battle)
    };

    if effective_weather == Some(ID::from("raindance")) || effective_weather == Some(ID::from("primordialsea")) {
        // this.attrLastMove('[still]');
        battle.attr_last_move("[still]");

        // this.addMove('-anim', attacker, move.name, defender);
        if let Some(defender) = defender {
            let defender_arg = {
                let defender_pokemon = match battle.pokemon_at(defender.0, defender.1) {
                    Some(p) => p,
                    None => return EventResult::Continue,
                };
                crate::battle::Arg::from(defender_pokemon)
            };
            battle.add_move("-anim", &[attacker_arg, move_name.into(), defender_arg]);
        } else {
            battle.add_move("-anim", &[attacker_arg, move_name.into()]);
        }

        // return;
        return EventResult::Continue;
    }

    // if (!this.runEvent('ChargeMove', attacker, defender, move)) {
    //     return;
    // }
    let charge_result = battle.run_event("ChargeMove", attacker, defender, None);
    if !charge_result {
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
    EventResult::Null
}

