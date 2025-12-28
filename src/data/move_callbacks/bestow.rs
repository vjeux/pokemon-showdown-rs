//! Bestow Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onHit(target, source, move) {
///     if (target.item) {
///         return false;
///     }
///     const myItem = source.takeItem();
///     if (!myItem) return false;
///     if (!this.singleEvent('TakeItem', myItem, source.itemState, target, source, move, myItem) || !target.setItem(myItem)) {
///         source.item = myItem.id;
///         return false;
///     }
///     this.add('-item', target, myItem.name, '[from] move: Bestow', `[of] ${source}`);
/// }
pub fn on_hit(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    // Get target
    let target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // if (target.item) {
    //     return false;
    // }
    {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        if !target_pokemon.item.is_empty() {
            return EventResult::Boolean(false);
        }
    }

    // const myItem = source.takeItem();
    let my_item = {
        let source_pokemon = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        source_pokemon.take_item()
    };

    // if (!myItem) return false;
    let my_item = match my_item {
        Some(item) => item,
        None => return EventResult::Boolean(false),
    };

    // if (!this.singleEvent('TakeItem', myItem, source.itemState, target, source, move, myItem) || !target.setItem(myItem)) {
    let take_item_event = battle.single_event("TakeItem", &my_item, Some(target), Some(pokemon_pos), None);

    let set_item_success = {
        let target_pokemon = battle.pokemon_at_mut(target.0, target.1);

        if let Some(target_pokemon) = target_pokemon {
            target_pokemon.set_item(my_item.clone())
        } else {
            false
        }
    };

    if matches!(take_item_event, EventResult::Boolean(false)) || !set_item_success {
        // source.item = myItem.id;
        let source_pokemon = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        source_pokemon.item = my_item;

        // return false;
        return EventResult::Boolean(false);
    }

    // this.add('-item', target, myItem.name, '[from] move: Bestow', `[of] ${source}`);
    let (item_name, target_ident, source_ident) = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        let source_pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        let item_data = battle.dex.get_item_by_id(&my_item);
        let item_name = item_data.map(|i| i.name.clone()).unwrap_or_else(|| my_item.to_string());

        (item_name, target_pokemon.get_slot(), source_pokemon.get_slot())
    };

    battle.add("-item", &[
        target_ident.as_str().into(),
        item_name.into(),
        "[from] move: Bestow".into(),
        format!("[of] {}", source_ident).into(),
    ]);

    EventResult::Continue
}

