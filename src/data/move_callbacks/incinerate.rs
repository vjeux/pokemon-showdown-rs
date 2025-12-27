//! Incinerate Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onHit(pokemon, source) {
///     const item = pokemon.getItem();
///     if ((item.isBerry || item.isGem) && pokemon.takeItem(source)) {
///         this.add('-enditem', pokemon, item.name, '[from] move: Incinerate');
///     }
/// }
pub fn on_hit(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    let pokemon = pokemon_pos;
    let source = target_pos;

    // const item = pokemon.getItem();
    let (is_berry_or_gem, item_name) = {
        let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        let item_id = match &pokemon_pokemon.item {
            Some(id) => id,
            None => return EventResult::Continue,
        };

        let item = battle.dex.get_item_by_id(item_id);
        match item {
            Some(i) => (i.is_berry || i.is_gem, i.name.clone()),
            None => return EventResult::Continue,
        }
    };

    // if ((item.isBerry || item.isGem) && pokemon.takeItem(source)) {
    if is_berry_or_gem {
        let took_item = battle.take_item(pokemon, source);
        if took_item {
            // this.add('-enditem', pokemon, item.name, '[from] move: Incinerate');
            let pokemon_arg = crate::battle::Arg::Pos(pokemon.0, pokemon.1);
            battle.add("-enditem", &[pokemon_arg, item_name.into(), "[from] move: Incinerate".into()]);
        }
    }

    EventResult::Continue
}

