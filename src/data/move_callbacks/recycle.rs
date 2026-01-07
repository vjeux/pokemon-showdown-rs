//! Recycle Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::{Battle, Effect};
use crate::event::EventResult;
use crate::Pokemon;

/// onHit(pokemon, source, move) {
///     if (pokemon.item || !pokemon.lastItem) return false;
///     const item = pokemon.lastItem;
///     pokemon.lastItem = '';
///     this.add('-item', pokemon, this.dex.items.get(item), '[from] move: Recycle');
///     pokemon.setItem(item, source, move);
/// }
pub fn on_hit(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
) -> EventResult {
    use crate::dex_data::ID;

    let pokemon = pokemon_pos;
    let _source = pokemon_pos;

    // if (pokemon.item || !pokemon.lastItem) return false;
    let (has_item, last_item) = {
        let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        (
            pokemon_pokemon.item != ID::from(""),
            pokemon_pokemon.last_item.clone(),
        )
    };

    if has_item || last_item == ID::from("") {
        return EventResult::Boolean(false);
    }

    // const item = pokemon.lastItem;
    let item = last_item;

    // pokemon.lastItem = '';
    let pokemon_pokemon = match battle.pokemon_at_mut(pokemon.0, pokemon.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };
    pokemon_pokemon.last_item = ID::from("");

    // this.add('-item', pokemon, this.dex.items.get(item), '[from] move: Recycle');
    let (pokemon_arg, item_name) = {
        let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        let pokemon_arg = pokemon_pokemon.get_slot();

        let item_data = match battle.dex.items().get_by_id(&item) {
            Some(i) => i,
            None => return EventResult::Continue,
        };

        (pokemon_arg, item_data.name.clone())
    };

    battle.add(
        "-item",
        &[
            pokemon_arg.into(),
            item_name.into(),
            "[from] move: Recycle".into(),
        ],
    );

    // pokemon.setItem(item, source, move);
    // JavaScript: pokemon.setItem(item, source, move)
    // source = pokemon (self-targeting move)
    // move = "recycle"
    // ✅ NOW PASSING: source_pos = Some(pokemon_pos), source_effect = Some("recycle")
    Pokemon::set_item(battle, pokemon_pos, item, Some(pokemon_pos), Some(&Effect::move_(ID::new("recycle"))));

    EventResult::Continue
}
