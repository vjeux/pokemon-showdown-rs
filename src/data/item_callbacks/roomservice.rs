//! Room Service Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::event::EventResult;

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
        pokemon.ignoring_item()
    };

    // Check if trick room is active
    let has_trick_room = battle.field.has_pseudo_weather(&ID::from("trickroom"));

    // if (!pokemon.ignoringItem() && this.field.getPseudoWeather('trickroom'))
    if !ignoring_item && has_trick_room {
        // pokemon.useItem();
        let pokemon_mut = match battle.pokemon_at_mut(target_pos.0, target_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon_mut.use_item();
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
    // TODO: Need effectState.target to get pokemon holding this item,
    // battle.field.getPseudoWeather('trickroom'), and pokemon.useItem()
    EventResult::Continue
}
