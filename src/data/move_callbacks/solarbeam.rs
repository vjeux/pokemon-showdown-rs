//! Solar Beam Move
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
///     if (['sunnyday', 'desolateland'].includes(attacker.effectiveWeather())) {
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

    // onTryMove(attacker, defender, move) {
    //     if (attacker.removeVolatile(move.id)) {
    //         return;
    //     }
    //     this.add('-prepare', attacker, move.name);
    //     if (['sunnyday', 'desolateland'].includes(attacker.effectiveWeather())) {
    //         this.attrLastMove('[still]');
    //         this.addMove('-anim', attacker, move.name, defender);
    //         return;
    //     }
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
            Some(m) => m,
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
            Some(m) => m,
            None => return EventResult::Continue,
        };
        (crate::battle::Arg::from(attacker_pokemon), active_move.name.clone())
    };

    battle.add("-prepare", &[
        attacker_arg.clone(),
        move_name.clone().into(),
    ]);

    // if (['sunnyday', 'desolateland'].includes(attacker.effectiveWeather())) {
    let weather = battle.effective_weather(attacker);
    if weather == Some(ID::from("sunnyday")) || weather == Some(ID::from("desolateland")) {
        // this.attrLastMove('[still]');
        battle.attr_last_move("[still]");

        // this.addMove('-anim', attacker, move.name, defender);
        if let Some(def) = defender {
            let defender_arg = {
                let defender_pokemon = match battle.pokemon_at(def.0, def.1) {
                    Some(p) => p,
                    None => return EventResult::Continue,
                };
                crate::battle::Arg::from(defender_pokemon)
            };

            battle.add_move("-anim", &[
                attacker_arg,
                move_name.into(),
                defender_arg,
            ]);
        } else {
            battle.add_move("-anim", &[
                attacker_arg,
                move_name.into(),
            ]);
        }

        // return;
        return EventResult::Continue;
    }

    // if (!this.runEvent('ChargeMove', attacker, defender, move)) {
    //     return;
    // }
    let charge_result = battle.run_event("ChargeMove", attacker, defender, None);
    if !charge_result {
        return EventResult::Continue;
    }

    // attacker.addVolatile('twoturnmove', defender);
    battle.add_volatile(&ID::from("twoturnmove"), attacker, defender, None);

    // return null;
    EventResult::Stop
}

/// onBasePower(basePower, pokemon, target) {
///     const weakWeathers = ['raindance', 'primordialsea', 'sandstorm', 'hail', 'snowscape'];
///     if (weakWeathers.includes(pokemon.effectiveWeather())) {
///         this.debug('weakened by weather');
///         return this.chainModify(0.5);
///     }
/// }
pub fn on_base_power(battle: &mut Battle, base_power: i32, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    use crate::dex_data::ID;

    // onBasePower(basePower, pokemon, target) {
    //     const weakWeathers = ['raindance', 'primordialsea', 'sandstorm', 'hail', 'snowscape'];
    //     if (weakWeathers.includes(pokemon.effectiveWeather())) {
    //         this.debug('weakened by weather');
    //         return this.chainModify(0.5);
    //     }
    // }
    let pokemon = pokemon_pos;

    // const weakWeathers = ['raindance', 'primordialsea', 'sandstorm', 'hail', 'snowscape'];
    // if (weakWeathers.includes(pokemon.effectiveWeather())) {
    let weather = battle.effective_weather(pokemon);
    let is_weak_weather = weather == Some(ID::from("raindance")) ||
                          weather == Some(ID::from("primordialsea")) ||
                          weather == Some(ID::from("sandstorm")) ||
                          weather == Some(ID::from("hail")) ||
                          weather == Some(ID::from("snowscape"));

    if is_weak_weather {
        // this.debug('weakened by weather');
        battle.debug("weakened by weather");

        // return this.chainModify(0.5);
        return EventResult::Number(battle.chain_modify_fraction(1, 2));
    }

    EventResult::Continue
}

