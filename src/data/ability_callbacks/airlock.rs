//! Air Lock Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::{Battle, Effect};
use crate::event::EventResult;

/// onSwitchIn(pokemon) {
///     // Air Lock does not activate when Skill Swapped or when Neutralizing Gas leaves the field
///     this.add('-ability', pokemon, 'Air Lock');
///     ((this.effect as any).onStart as (p: Pokemon) => void).call(this, pokemon);
/// }
pub fn on_switch_in(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    use crate::battle::Arg;

    // this.add('-ability', pokemon, 'Air Lock');
    let pokemon_slot = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.get_slot()
    };

    battle.add("-ability", &[
        Arg::String(pokemon_slot),
        Arg::Str("Air Lock"),
    ]);

    // ((this.effect as any).onStart as (p: Pokemon) => void).call(this, pokemon);
    // This calls the ability's own onStart callback
    on_start(battle, pokemon_pos, None, None)
}

/// onStart(pokemon) {
///     pokemon.abilityState.ending = false; // Clear the ending flag
///     this.eachEvent('WeatherChange', this.effect);
/// }
pub fn on_start(battle: &mut Battle, pokemon_pos: (usize, usize), _source_pos: Option<(usize, usize)>, _effect: Option<&Effect>) -> EventResult {
    use crate::dex_data::ID;

    // pokemon.abilityState.ending = false;
    {
        let pokemon = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.ability_state.borrow_mut().ending = Some(false);
    }

    // this.eachEvent('WeatherChange', this.effect);
    battle.each_event("WeatherChange", Some(&Effect::ability(ID::from("airlock"))), None);

    EventResult::Continue
}

/// onEnd(pokemon) {
///     pokemon.abilityState.ending = true;
///     this.eachEvent('WeatherChange', this.effect);
/// }
pub fn on_end(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    use crate::dex_data::ID;

    // pokemon.abilityState.ending = true;
    {
        let pokemon = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.ability_state.borrow_mut().ending = Some(true);
    }

    // this.eachEvent('WeatherChange', this.effect);
    battle.each_event("WeatherChange", Some(&Effect::ability(ID::from("airlock"))), None);

    EventResult::Continue
}

