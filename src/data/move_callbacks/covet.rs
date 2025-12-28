//! Covet Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onAfterHit(target, source, move) {
///     if (source.item || source.volatiles['gem']) {
///         return;
///     }
///     const yourItem = target.takeItem(source);
///     if (!yourItem) {
///         return;
///     }
///     if (
///         !this.singleEvent('TakeItem', yourItem, target.itemState, source, target, move, yourItem) ||
///         !source.setItem(yourItem)
///     ) {
///         target.item = yourItem.id; // bypass setItem so we don't break choicelock or anything
///         return;
///     }
///     this.add('-item', source, yourItem, '[from] move: Covet', `[of] ${target}`);
/// }
pub fn on_after_hit(battle: &mut Battle, source_pos: (usize, usize), target_pos: (usize, usize)) -> EventResult {
    use crate::dex_data::ID;

    // if (source.item || source.volatiles['gem']) {
    //     return;
    // }
    let source_has_item_or_gem = {
        let source_pokemon = match battle.pokemon_at(source_pos.0, source_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        !source_pokemon.item.is_empty() || source_pokemon.volatiles.contains_key(&ID::from("gem"))
    };

    if source_has_item_or_gem {
        // return;
        return EventResult::Continue;
    }

    // const yourItem = target.takeItem(source);
    let your_item = {
        let target_pokemon = match battle.pokemon_at_mut(target_pos.0, target_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target_pokemon.take_item()
    };

    // if (!yourItem) {
    //     return;
    // }
    let your_item_id = match your_item {
        Some(id) => id,
        None => {
            // return;
            return EventResult::Continue;
        }
    };

    // if (
    //     !this.singleEvent('TakeItem', yourItem, target.itemState, source, target, move, yourItem) ||
    //     !source.setItem(yourItem)
    // ) {
    //     target.item = yourItem.id; // bypass setItem so we don't break choicelock or anything
    //     return;
    // }
    // TODO: Implement singleEvent for TakeItem
    // For now, we'll just try to set the item directly
    let set_item_success = {
        let source_pokemon = match battle.pokemon_at_mut(source_pos.0, source_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        source_pokemon.set_item(your_item_id.clone())
    };

    if !set_item_success {
        // target.item = yourItem.id; // bypass setItem so we don't break choicelock or anything
        let target_pokemon = match battle.pokemon_at_mut(target_pos.0, target_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target_pokemon.item = your_item_id;
        // return;
        return EventResult::Continue;
    }

    // this.add('-item', source, yourItem, '[from] move: Covet', `[of] ${target}`);
    let (source_arg, target_arg, item_name) = {
        let source_pokemon = match battle.pokemon_at(source_pos.0, source_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        let target_pokemon = match battle.pokemon_at(target_pos.0, target_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        let item_data = battle.dex.get_item_by_id(&your_item_id);
        let item_name = item_data.map(|i| i.name.clone()).unwrap_or_else(|| your_item_id.to_string());

        (crate::battle::Arg::from(source_pokemon), crate::battle::Arg::from(target_pokemon), item_name)
    };

    battle.add("-item", &[
        source_arg,
        item_name.into(),
        "[from] move: Covet".into(),
        format!("[of] {}", target_arg).into(),
    ]);

    EventResult::Continue
}

