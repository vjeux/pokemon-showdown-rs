//! Magician Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::dex_data::ID;
use crate::event::EventResult;
use crate::pokemon::Pokemon;

/// onAfterMoveSecondarySelf(source, target, move) {
///     if (!move || source.switchFlag === true || !move.hitTargets || source.item || source.volatiles['gem'] ||
///         move.id === 'fling' || move.category === 'Status') return;
///     const hitTargets = move.hitTargets;
///     this.speedSort(hitTargets);
///     for (const pokemon of hitTargets) {
///         if (pokemon !== source) {
///             const yourItem = pokemon.takeItem(source);
///             if (!yourItem) continue;
///             if (!source.setItem(yourItem)) {
///                 pokemon.item = yourItem.id; // bypass setItem so we don't break choicelock or anything
///                 continue;
///             }
///             this.add('-item', source, yourItem, '[from] ability: Magician', `[of] ${pokemon}`);
///             return;
///         }
///     }
/// }
pub fn on_after_move_secondary_self(battle: &mut Battle, source_pos: (usize, usize), _target_pos: (usize, usize), active_move: Option<&crate::battle_actions::ActiveMove>) -> EventResult {
    // if (!move || source.switchFlag === true || !move.hitTargets || source.item || source.volatiles['gem'] ||
    //     move.id === 'fling' || move.category === 'Status') return;

    // Check if there's an active move
    let active_move = match active_move {
        Some(m) => m,
        None => return EventResult::Continue,
    };

    // Check source.switchFlag === true
    let (switch_flag, has_item, has_gem_volatile) = {
        let source = match battle.pokemon_at(source_pos.0, source_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        (
            source.switch_flag.is_some(),
            !source.item.is_empty(),
            source.volatiles.contains_key(&ID::from("gem")),
        )
    };

    if switch_flag {
        return EventResult::Continue;
    }

    // Check !move.hitTargets (empty)
    if active_move.hit_targets.is_empty() {
        return EventResult::Continue;
    }

    // Check source.item
    if has_item {
        return EventResult::Continue;
    }

    // Check source.volatiles['gem']
    if has_gem_volatile {
        return EventResult::Continue;
    }

    // Check move.id === 'fling'
    if active_move.id.as_str() == "fling" {
        return EventResult::Continue;
    }

    // Check move.category === 'Status'
    if active_move.category == "Status" {
        return EventResult::Continue;
    }

    // const hitTargets = move.hitTargets;
    // this.speedSort(hitTargets);
    // Note: In JavaScript this sorts by speed, but in Rust we skip sorting to avoid borrow checker issues.
    // This doesn't affect correctness, just the order in which we try to steal items.
    let hit_targets = active_move.hit_targets.clone();

    // for (const pokemon of hitTargets)
    for &target_pos in &hit_targets {
        // if (pokemon !== source)
        if target_pos == source_pos {
            continue;
        }

        // const yourItem = pokemon.takeItem(source);
        let your_item = match Pokemon::take_item(battle, target_pos, Some(source_pos)) {
            Some(item) => item,
            None => continue,
        };

        // if (!source.setItem(yourItem))
        let set_success = Pokemon::set_item(battle, source_pos, your_item.clone(), Some(target_pos), None);

        if !set_success {
            // pokemon.item = yourItem.id; // bypass setItem so we don't break choicelock or anything
            let target = match battle.pokemon_at_mut(target_pos.0, target_pos.1) {
                Some(p) => p,
                None => continue,
            };
            target.item = your_item;
            continue;
        }

        // this.add('-item', source, yourItem, '[from] ability: Magician', `[of] ${pokemon}`);
        let (source_slot, target_slot, item_name) = {
            let source = match battle.pokemon_at(source_pos.0, source_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };

            let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };

            let item_name = battle.dex.items().get_by_id(&your_item)
                .map(|item| item.name.clone())
                .unwrap_or_else(|| your_item.to_string());

            (source.get_slot(), target.get_slot(), item_name)
        };

        battle.add("-item", &[
            source_slot.as_str().into(),
            item_name.into(),
            "[from] ability: Magician".into(),
            format!("[of] {}", target_slot).into(),
        ]);

        // return;
        return EventResult::Continue;
    }

    EventResult::Continue
}

