//! Recycle Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onHit(pokemon, source, move) {
///     if (pokemon.item || !pokemon.lastItem) return false;
///     const item = pokemon.lastItem;
///     pokemon.lastItem = '';
///     this.add('-item', pokemon, this.dex.items.get(item), '[from] move: Recycle');
///     pokemon.setItem(item, source, move);
/// }
pub fn on_hit(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    let pokemon = pokemon_pos;
    let source = pokemon_pos;

    // if (pokemon.item || !pokemon.lastItem) return false;
    let (has_item, last_item) = {
        let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        (pokemon_pokemon.item.is_some(), pokemon_pokemon.last_item.clone())
    };

    if has_item || last_item.is_none() {
        return EventResult::Boolean(false);
    }

    // const item = pokemon.lastItem;
    let item = last_item.unwrap();

    // pokemon.lastItem = '';
    let pokemon_pokemon = match battle.pokemon_at_mut(pokemon.0, pokemon.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };
    pokemon_pokemon.last_item = None;

    // this.add('-item', pokemon, this.dex.items.get(item), '[from] move: Recycle');
    let (pokemon_arg, item_name) = {
        let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        let pokemon_arg = crate::battle::Arg::from(pokemon_pokemon);

        let item_data = match battle.dex.get_item_by_id(&item) {
            Some(i) => i,
            None => return EventResult::Continue,
        };

        (pokemon_arg, item_data.name.clone())
    };

    battle.add("-item", &[
        pokemon_arg,
        item_name.into(),
        "[from] move: Recycle".into(),
    ]);

    // pokemon.setItem(item, source, move);
    let move_id = {
        let active_move = match &battle.active_move {
            Some(active_move) => &active_move.id,
            None => return EventResult::Continue,
        };
        active_move.clone()
    };

    battle.set_item(pokemon, &item, Some(source), Some(&move_id));

    EventResult::Continue
}

