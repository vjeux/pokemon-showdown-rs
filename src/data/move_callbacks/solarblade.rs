//! Solar Blade Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;
use crate::pokemon::Pokemon;

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
pub fn on_try_move(
    battle: &mut Battle,
    source_pos: (usize, usize),
    target_pos: Option<(usize, usize)>,
) -> EventResult {
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
            Some(active_move) => active_move,
            None => return EventResult::Continue,
        };
        active_move.id.clone()
    };

    let removed = {
        let pokemon = match battle.pokemon_at_mut(attacker.0, attacker.1) {
            Some(p) => p,

            None => return EventResult::Continue,
        };

        pokemon.remove_volatile(&move_id)
    };
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
        (attacker_pokemon.get_slot(), active_move.name.clone())
    };

    battle.add(
        "-prepare",
        &[attacker_arg.clone().into(), move_name.clone().into()],
    );

    // if (['sunnyday', 'desolateland'].includes(attacker.effectiveWeather())) {
    let weather = {
        let field_weather = battle.field.weather.as_str();
        let pokemon = match battle.pokemon_at(attacker.0, attacker.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        let weather_str = pokemon.effective_weather(field_weather);
        if weather_str.is_empty() {
            None
        } else {
            Some(ID::from(weather_str.as_str()))
        }
    };
    if weather == Some(ID::from("sunnyday")) || weather == Some(ID::from("desolateland")) {
        // this.attrLastMove('[still]');
        battle.attr_last_move(&["[still]"]);

        // this.addMove('-anim', attacker, move.name, defender);
        if let Some(def) = defender {
            let defender_arg = {
                let defender_pokemon = match battle.pokemon_at(def.0, def.1) {
                    Some(p) => p,
                    None => return EventResult::Continue,
                };
                defender_pokemon.get_slot()
            };

            battle.add(
                "-anim",
                &[attacker_arg.into(), move_name.into(), defender_arg.into()],
            );
        } else {
            battle.add("-anim", &[attacker_arg.into(), move_name.into()]);
        }

        // return;
        return EventResult::Continue;
    }

    // if (!this.runEvent('ChargeMove', attacker, defender, move)) {
    //     return;
    // }
    let charge_result = battle.run_event("ChargeMove", Some(attacker), defender, None, None);
    if charge_result == Some(0) {
        return EventResult::Continue;
    }

    // attacker.addVolatile('twoturnmove', defender);
    Pokemon::add_volatile(battle, attacker, ID::from("twoturnmove"), defender);

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
pub fn on_base_power(
    battle: &mut Battle,
    _base_power: i32,
    pokemon_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
) -> EventResult {
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
    let weather = {
        let field_weather = battle.field.weather.as_str();
        let pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        let weather_str = pokemon.effective_weather(field_weather);
        if weather_str.is_empty() {
            None
        } else {
            Some(ID::from(weather_str.as_str()))
        }
    };
    let is_weak_weather = weather == Some(ID::from("raindance"))
        || weather == Some(ID::from("primordialsea"))
        || weather == Some(ID::from("sandstorm"))
        || weather == Some(ID::from("hail"))
        || weather == Some(ID::from("snowscape"));

    if is_weak_weather {
        // this.debug('weakened by weather');
        battle.debug("weakened by weather");

        // return this.chainModify(0.5);
        return EventResult::Number(battle.chain_modify_fraction(1, 2));
    }

    EventResult::Continue
}
