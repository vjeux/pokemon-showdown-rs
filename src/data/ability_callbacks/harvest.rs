//! Harvest Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::dex_data::ID;
use crate::event::EventResult;
use crate::pokemon::Pokemon;

/// onResidual(pokemon) {
///     if (this.field.isWeather(['sunnyday', 'desolateland']) || this.randomChance(1, 2)) {
///         if (pokemon.hp && !pokemon.item && this.dex.items.get(pokemon.lastItem).isBerry) {
///             pokemon.setItem(pokemon.lastItem);
///             pokemon.lastItem = '';
///             this.add('-item', pokemon, pokemon.getItem(), '[from] ability: Harvest');
///         }
///     }
/// }
pub fn on_residual(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // if (this.field.isWeather(['sunnyday', 'desolateland']) || this.randomChance(1, 2))
    let should_activate = battle.field.is_weather_any(&["sunnyday", "desolateland"])
        || battle.random_chance(1, 2);

    if !should_activate {
        return EventResult::Continue;
    }

    // if (pokemon.hp && !pokemon.item && this.dex.items.get(pokemon.lastItem).isBerry)
    let (has_hp, no_item, last_item_id, is_berry) = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        let has_hp = pokemon.hp > 0;
        let no_item = pokemon.item.is_empty();
        let last_item_id = pokemon.last_item.clone();

        // Check if last item was a berry
        let is_berry = if !last_item_id.is_empty() {
            battle.dex.items().get_by_id(&last_item_id)
                .map(|item| item.is_berry)
                .unwrap_or(false)
        } else {
            false
        };

        (has_hp, no_item, last_item_id, is_berry)
    };

    if !has_hp || !no_item || !is_berry {
        return EventResult::Continue;
    }

    // pokemon.setItem(pokemon.lastItem);
    Pokemon::set_item(battle, pokemon_pos, ID::from(last_item_id.clone()), None, None);

    // pokemon.lastItem = '';
    {
        let pokemon = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.last_item = "".into();
    }

    // Get pokemon slot and item name for message
    let (pokemon_slot, item_name) = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        let item_name = battle.dex.items().get_by_id(&pokemon.item)
            .map(|item| item.name.clone())
            .unwrap_or_else(|| last_item_id.to_string());

        (pokemon.get_slot(), item_name)
    };

    // this.add('-item', pokemon, pokemon.getItem(), '[from] ability: Harvest');
    battle.add("-item", &[
        pokemon_slot.as_str().into(),
        item_name.into(),
        "[from] ability: Harvest".into(),
    ]);

    EventResult::Continue
}

