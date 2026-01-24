//! Bestow Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;
use crate::Pokemon;

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
pub fn on_hit(
    battle: &mut Battle,
    target_pos: (usize, usize),
    source_pos: Option<(usize, usize)>,
) -> EventResult {
    // Get source (the Pokemon using Bestow, who gives their item)
    let source = match source_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // if (target.item) {
    //     return false;
    // }
    // Check if target already has an item - if so, fail
    {
        let target_pokemon = match battle.pokemon_at(target_pos.0, target_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        if !target_pokemon.item.is_empty() {
            return EventResult::Boolean(false);
        }
    }

    // const myItem = source.takeItem();
    // Take item from source (the one using Bestow)
    let my_item = {
        Pokemon::take_item(battle, source, None)
    };

    // if (!myItem) return false;
    let my_item = match my_item {
        Some(item) => item,
        None => return EventResult::Boolean(false),
    };

    // if (!this.singleEvent('TakeItem', myItem, source.itemState, target, source, move, myItem) || !target.setItem(myItem)) {
    let item_effect = battle.make_item_effect(&my_item);
    let take_item_event =
        battle.single_event("TakeItem", &item_effect, None, Some(target_pos), Some(source), None, None);

    // Give item to target (the one receiving via Bestow)
    let set_item_success = Pokemon::set_item(battle, target_pos, my_item.clone(), None, None);

    if matches!(take_item_event, EventResult::Boolean(false)) || !set_item_success {
        // source.item = myItem.id;
        // Restore item to source if transfer failed
        let source_pokemon = match battle.pokemon_at_mut(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        source_pokemon.item = my_item;

        // return false;
        return EventResult::Boolean(false);
    }

    // this.add('-item', target, myItem.name, '[from] move: Bestow', `[of] ${source}`);
    let (item_name, target_ident, source_ident) = {
        let target_pokemon = match battle.pokemon_at(target_pos.0, target_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        let source_pokemon = match battle.pokemon_at(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        let item_data = battle.dex.items().get_by_id(&my_item);
        let item_name = item_data
            .map(|i| i.name.clone())
            .unwrap_or_else(|| my_item.to_string());

        (
            item_name,
            target_pokemon.get_slot(),
            source_pokemon.get_slot(),
        )
    };

    battle.add(
        "-item",
        &[
            target_ident.as_str().into(),
            item_name.into(),
            "[from] move: Bestow".into(),
            format!("[of] {}", source_ident).into(),
        ],
    );

    EventResult::Continue
}
