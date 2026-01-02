//! Stuff Cheeks Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;
use crate::Pokemon;

/// onDisableMove(pokemon) {
///     if (!pokemon.getItem().isBerry) pokemon.disableMove('stuffcheeks');
/// }
pub fn on_disable_move(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    let pokemon = pokemon_pos;

    // if (!pokemon.getItem().isBerry) pokemon.disableMove('stuffcheeks');
    let is_berry = {
        let pokemon_ref = match battle.pokemon_at(pokemon.0, pokemon.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        let item_id = pokemon_ref.get_item();
        let item_data = battle.dex.items().get_by_id(&item_id);

        match item_data {
            Some(i) => i.is_berry,
            None => false,
        }
    };

    if !is_berry {
        let pokemon_mut = match battle.pokemon_at_mut(pokemon.0, pokemon.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        pokemon_mut.disable_move("stuffcheeks", false, None);
    }

    EventResult::Continue
}

/// onTry(source) {
///     return source.getItem().isBerry;
/// }
pub fn on_try(
    battle: &mut Battle,
    source_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
) -> EventResult {
    let source = source_pos;

    // return source.getItem().isBerry;
    let is_berry = {
        let source_pokemon = match battle.pokemon_at(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        let item_id = source_pokemon.get_item();
        let item_data = battle.dex.items().get_by_id(&item_id);

        match item_data {
            Some(i) => i.is_berry,
            None => false,
        }
    };

    if !is_berry {
        return EventResult::NotFail;
    }

    EventResult::Continue
}

/// onHit(pokemon) {
///     if (!this.boost({ def: 2 })) return null;
///     pokemon.eatItem(true);
/// }
pub fn on_hit(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
) -> EventResult {
    let pokemon = pokemon_pos;

    // if (!this.boost({ def: 2 })) return null;
    let boost_result = battle.boost(&[("def", 2)], pokemon, Some(pokemon), None, false, false);

    if !boost_result {
        return EventResult::Stop; // return null
    }

    // pokemon.eatItem(true);
    let _pokemon_mut = match battle.pokemon_at_mut(pokemon.0, pokemon.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };

    Pokemon::eat_item(battle, pokemon_pos, true, None, None);

    EventResult::Continue
}
