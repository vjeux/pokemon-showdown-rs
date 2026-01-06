//! Thief Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::dex_data::ID;
use crate::event::EventResult;
use crate::Pokemon;

/// onAfterHit(target, source, move)
///
/// ```text
/// JS Source (data/moves.ts):
/// onAfterHit(target, source, move) {
///     if (source.item || source.volatiles['gem']) {
///         return;
///     }
///     const yourItem = target.takeItem(source);
///     if (!yourItem) {
///         return;
///     }
///     if (!this.singleEvent('TakeItem', yourItem, target.itemState, source, target, move, yourItem) ||
///         !source.setItem(yourItem)) {
///         target.item = yourItem.id; // bypass setItem so we don't break choicelock or anything
///         return;
///     }
///     this.add('-enditem', target, yourItem, '[silent]', '[from] move: Thief', `[of] ${source}`);
///     this.add('-item', source, yourItem, '[from] move: Thief', `[of] ${target}`);
/// }
/// ```
pub fn on_after_hit(
    battle: &mut Battle,
    source_pos: (usize, usize),
    target_pos: (usize, usize),
) -> EventResult {
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
    let your_item = Pokemon::take_item(battle, target_pos, Some(source_pos));

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

    // if (!this.singleEvent('TakeItem', yourItem, target.itemState, source, target, move, yourItem) ||
    //     !source.setItem(yourItem)) {
    //     target.item = yourItem.id; // bypass setItem so we don't break choicelock or anything
    //     return;
    // }

    // Check if taking the item is allowed via singleEvent
    let take_item_result = battle.single_event(
        "TakeItem",
        &your_item_id,
        Some(source_pos),
        Some(target_pos),
        Some(&your_item_id),
        None,
    );

    let can_take_item = !matches!(take_item_result, EventResult::Boolean(false));

    let set_item_success = if can_take_item {
        Pokemon::set_item(battle, source_pos, your_item_id.clone(), None, None)
    } else {
        false
    };

    if !can_take_item || !set_item_success {
        // target.item = yourItem.id; // bypass setItem so we don't break choicelock or anything
        let target_pokemon = match battle.pokemon_at_mut(target_pos.0, target_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target_pokemon.item = your_item_id;
        // return;
        return EventResult::Continue;
    }

    // this.add('-enditem', target, yourItem, '[silent]', '[from] move: Thief', `[of] ${source}`);
    // this.add('-item', source, yourItem, '[from] move: Thief', `[of] ${target}`);
    let (source_ident, target_ident, item_name) = {
        let source_pokemon = match battle.pokemon_at(source_pos.0, source_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        let target_pokemon = match battle.pokemon_at(target_pos.0, target_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        let item_data = battle.dex.items().get_by_id(&your_item_id);
        let item_name = item_data
            .map(|i| i.name.clone())
            .unwrap_or_else(|| your_item_id.to_string());

        (
            source_pokemon.get_slot(),
            target_pokemon.get_slot(),
            item_name,
        )
    };

    battle.add(
        "-enditem",
        &[
            target_ident.as_str().into(),
            item_name.clone().into(),
            "[silent]".into(),
            "[from] move: Thief".into(),
            format!("[of] {}", source_ident).into(),
        ],
    );

    battle.add(
        "-item",
        &[
            source_ident.as_str().into(),
            item_name.into(),
            "[from] move: Thief".into(),
            format!("[of] {}", target_ident).into(),
        ],
    );

    EventResult::Continue
}
