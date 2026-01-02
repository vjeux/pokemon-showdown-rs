//! Room Service Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::event::EventResult;
use crate::Pokemon;

/// onStart(pokemon) {
///     if (!pokemon.ignoringItem() && this.field.getPseudoWeather('trickroom')) {
///         pokemon.useItem();
///     }
/// }
pub fn on_start(battle: &mut Battle, target_pos: Option<(usize, usize)>) -> EventResult {
    // if (!pokemon.ignoringItem() && this.field.getPseudoWeather('trickroom')) {
    //     pokemon.useItem();
    // }

    use crate::dex_data::ID;

    let target_pos = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // Check if pokemon is ignoring item
    let ignoring_item = {
        let pokemon = match battle.pokemon_at(target_pos.0, target_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.ignoring_item(battle, false)
    };

    // Check if trick room is active
    let has_trick_room = battle.field.has_pseudo_weather(&ID::from("trickroom"));

    // if (!pokemon.ignoringItem() && this.field.getPseudoWeather('trickroom'))
    if !ignoring_item && has_trick_room {
        // pokemon.useItem();
        Pokemon::use_item(battle, target_pos, None, None);
    }

    EventResult::Continue
}

/// onAnyPseudoWeatherChange() {
///     const pokemon = this.effectState.target;
///     if (this.field.getPseudoWeather('trickroom')) {
///         pokemon.useItem(pokemon);
///     }
/// }
pub fn on_any_pseudo_weather_change(battle: &mut Battle) -> EventResult {
    // const pokemon = this.effectState.target;
    let pokemon_pos = {
        if let Some(ref effect_state) = battle.current_effect_state {
            effect_state.target
        } else {
            return EventResult::Continue;
        }
    };

    let pokemon_pos = match pokemon_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // if (this.field.getPseudoWeather('trickroom'))
    use crate::dex_data::ID;
    let has_trick_room = battle.field.has_pseudo_weather(&ID::from("trickroom"));

    if has_trick_room {
        // pokemon.useItem(pokemon);
        let pokemon_mut = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        Pokemon::use_item(battle, pokemon_pos, None, None);
    }

    EventResult::Continue
}
