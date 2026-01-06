//! Switcheroo Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;
use crate::Pokemon;

/// onTryImmunity(target) {
///     return !target.hasAbility('stickyhold');
/// }
pub fn on_try_immunity(battle: &mut Battle, target_pos: Option<(usize, usize)>) -> EventResult {
    let target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // return !target.hasAbility('stickyhold');
    let has_stickyhold = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target_pokemon.has_ability(battle, &["stickyhold"])
    };

    // Return the negation - if has stickyhold, return false (immune), otherwise true (not immune)
    EventResult::Boolean(!has_stickyhold)
}

/// onHit(target, source, move) {
///     const yourItem = target.takeItem(source);
///     const myItem = source.takeItem();
///     if (target.item || source.item || (!yourItem && !myItem)) {
///         if (yourItem) target.item = yourItem.id;
///         if (myItem) source.item = myItem.id;
///         return false;
///     }
///     if (
///         (myItem && !this.singleEvent('TakeItem', myItem, source.itemState, target, source, move, myItem)) ||
///         (yourItem && !this.singleEvent('TakeItem', yourItem, target.itemState, source, target, move, yourItem))
///     ) {
///         if (yourItem) target.item = yourItem.id;
///         if (myItem) source.item = myItem.id;
///         return false;
///     }
///     this.add('-activate', source, 'move: Trick', `[of] ${target}`);
///     if (myItem) {
///         target.setItem(myItem);
///         this.add('-item', target, myItem, '[from] move: Switcheroo');
///     } else {
///         this.add('-enditem', target, yourItem, '[silent]', '[from] move: Switcheroo');
///     }
///     if (yourItem) {
///         source.setItem(yourItem);
///         this.add('-item', source, yourItem, '[from] move: Switcheroo');
///     } else {
///         this.add('-enditem', source, myItem, '[silent]', '[from] move: Switcheroo');
///     }
/// }
pub fn on_hit(
    battle: &mut Battle,
    source_pos: (usize, usize),
    target_pos: Option<(usize, usize)>,
) -> EventResult {
    let target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // const yourItem = target.takeItem(source);
    let your_item = Pokemon::take_item(battle, target, Some(source_pos));

    // const myItem = source.takeItem();
    let my_item = Pokemon::take_item(battle, source_pos, None);

    // if (target.item || source.item || (!yourItem && !myItem)) {
    //     if (yourItem) target.item = yourItem.id;
    //     if (myItem) source.item = myItem.id;
    //     return false;
    // }
    let (target_has_item, source_has_item) = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        let source_pokemon = match battle.pokemon_at(source_pos.0, source_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        (!target_pokemon.item.is_empty(), !source_pokemon.item.is_empty())
    };

    if target_has_item || source_has_item || (your_item.is_none() && my_item.is_none()) {
        // Restore items
        if let Some(item_id) = your_item {
            if let Some(target_pokemon) = battle.pokemon_at_mut(target.0, target.1) {
                target_pokemon.item = item_id;
            }
        }
        if let Some(item_id) = my_item {
            if let Some(source_pokemon) = battle.pokemon_at_mut(source_pos.0, source_pos.1) {
                source_pokemon.item = item_id;
            }
        }
        return EventResult::Boolean(false);
    }

    // if (
    //     (myItem && !this.singleEvent('TakeItem', myItem, source.itemState, target, source, move, myItem)) ||
    //     (yourItem && !this.singleEvent('TakeItem', yourItem, target.itemState, source, target, move, yourItem))
    // ) {
    //     if (yourItem) target.item = yourItem.id;
    //     if (myItem) source.item = myItem.id;
    //     return false;
    // }

    // Check if TakeItem events allow the swap
    let my_item_allowed = if let Some(ref my_item_id) = my_item {
        let result = battle.single_event(
            "TakeItem",
            my_item_id,
            Some(source_pos),
            Some(target),
            None,
            None,
        );
        !matches!(result, EventResult::Null | EventResult::Boolean(false))
    } else {
        true
    };

    let your_item_allowed = if let Some(ref your_item_id) = your_item {
        let result = battle.single_event(
            "TakeItem",
            your_item_id,
            Some(target),
            Some(source_pos),
            None,
            None,
        );
        !matches!(result, EventResult::Null | EventResult::Boolean(false))
    } else {
        true
    };

    if !my_item_allowed || !your_item_allowed {
        // Restore items
        if let Some(item_id) = your_item {
            if let Some(target_pokemon) = battle.pokemon_at_mut(target.0, target.1) {
                target_pokemon.item = item_id;
            }
        }
        if let Some(item_id) = my_item {
            if let Some(source_pokemon) = battle.pokemon_at_mut(source_pos.0, source_pos.1) {
                source_pokemon.item = item_id;
            }
        }
        return EventResult::Boolean(false);
    }

    // this.add('-activate', source, 'move: Trick', `[of] ${target}`);
    // Note: JS code says 'move: Trick' even though this is switcheroo
    let (source_ident, target_ident) = {
        let source_pokemon = match battle.pokemon_at(source_pos.0, source_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        (source_pokemon.get_slot(), target_pokemon.get_slot())
    };

    battle.add(
        "-activate",
        &[
            source_ident.as_str().into(),
            "move: Trick".into(),
            format!("[of] {}", target_ident).into(),
        ],
    );

    // if (myItem) {
    //     target.setItem(myItem);
    //     this.add('-item', target, myItem, '[from] move: Switcheroo');
    // } else {
    //     this.add('-enditem', target, yourItem, '[silent]', '[from] move: Switcheroo');
    // }
    if let Some(my_item_id) = my_item.clone() {
        Pokemon::set_item(battle, target, my_item_id.clone(), None, None);

        let my_item_name = {
            let item_data = battle.dex.items().get_by_id(&my_item_id);
            item_data
                .map(|i| i.name.clone())
                .unwrap_or_else(|| my_item_id.to_string())
        };

        battle.add(
            "-item",
            &[
                target_ident.as_str().into(),
                my_item_name.into(),
                "[from] move: Switcheroo".into(),
            ],
        );
    } else if let Some(your_item_id) = your_item.clone() {
        let your_item_name = {
            let item_data = battle.dex.items().get_by_id(&your_item_id);
            item_data
                .map(|i| i.name.clone())
                .unwrap_or_else(|| your_item_id.to_string())
        };

        battle.add(
            "-enditem",
            &[
                target_ident.as_str().into(),
                your_item_name.into(),
                "[silent]".into(),
                "[from] move: Switcheroo".into(),
            ],
        );
    }

    // if (yourItem) {
    //     source.setItem(yourItem);
    //     this.add('-item', source, yourItem, '[from] move: Switcheroo');
    // } else {
    //     this.add('-enditem', source, myItem, '[silent]', '[from] move: Switcheroo');
    // }
    if let Some(your_item_id) = your_item {
        Pokemon::set_item(battle, source_pos, your_item_id.clone(), None, None);

        let your_item_name = {
            let item_data = battle.dex.items().get_by_id(&your_item_id);
            item_data
                .map(|i| i.name.clone())
                .unwrap_or_else(|| your_item_id.to_string())
        };

        battle.add(
            "-item",
            &[
                source_ident.as_str().into(),
                your_item_name.into(),
                "[from] move: Switcheroo".into(),
            ],
        );
    } else if let Some(my_item_id) = my_item {
        let my_item_name = {
            let item_data = battle.dex.items().get_by_id(&my_item_id);
            item_data
                .map(|i| i.name.clone())
                .unwrap_or_else(|| my_item_id.to_string())
        };

        battle.add(
            "-enditem",
            &[
                source_ident.as_str().into(),
                my_item_name.into(),
                "[silent]".into(),
                "[from] move: Switcheroo".into(),
            ],
        );
    }

    EventResult::Continue
}
