//! Symbiosis Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;
use crate::pokemon::Pokemon;

/// onAllyAfterUseItem(item, pokemon) {
///     if (pokemon.switchFlag) return;
///     const source = this.effectState.target;
///     const myItem = source.takeItem();
///     if (!myItem) return;
///     if (
///         !this.singleEvent('TakeItem', myItem, source.itemState, pokemon, source, this.effect, myItem) ||
///         !pokemon.setItem(myItem)
///     ) {
///         source.item = myItem.id;
///         return;
///     }
///     this.add('-activate', source, 'ability: Symbiosis', myItem, `[of] ${pokemon}`);
/// }
pub fn on_ally_after_use_item(battle: &mut Battle, _item_id: Option<&str>, pokemon_pos: (usize, usize)) -> EventResult {
    // if (pokemon.switchFlag) return;
    let switch_flag = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.switch_flag.is_some()
    };

    if switch_flag {
        return EventResult::Continue;
    }

    // const source = this.effectState.target;
    let source_pos = match battle.effect_state.target {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // const myItem = source.takeItem();
    let my_item = match Pokemon::take_item(battle, source_pos, None) {
        Some(item) => item,
        None => return EventResult::Continue,
    };

    // if (!this.singleEvent('TakeItem', myItem, source.itemState, pokemon, source, this.effect, myItem))
    // Note: singleEvent returns the relay variable, which is true by default unless modified
    // For TakeItem event, if it returns false/Null, we should not proceed
    let take_item_result = battle.single_event(
        "TakeItem",
        &my_item,
        Some(pokemon_pos),
        Some(source_pos),
        None,
        None,
    );

    // Check if TakeItem event blocked the transfer (returns Null/false)
    let take_allowed = match take_item_result {
        EventResult::Null => false,
        EventResult::Boolean(b) => b,
        _ => true, // Continue/other results mean allowed
    };

    if !take_allowed {
        // Restore item to source
        let source_mut = match battle.pokemon_at_mut(source_pos.0, source_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        source_mut.item = my_item;
        return EventResult::Continue;
    }

    // if (!pokemon.setItem(myItem))
    let set_success = Pokemon::set_item(battle, pokemon_pos, my_item.clone(), None, None);

    if !set_success {
        // Restore item to source
        let source_mut = match battle.pokemon_at_mut(source_pos.0, source_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        source_mut.item = my_item;
        return EventResult::Continue;
    }

    // Get slots and item name for message
    let (source_slot, pokemon_slot, item_name) = {
        let source = match battle.pokemon_at(source_pos.0, source_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        let item_name = battle
            .dex
            .items()
            .get_by_id(&my_item)
            .map(|item| item.name.clone())
            .unwrap_or_else(|| my_item.to_string());

        (source.get_slot(), pokemon.get_slot(), item_name)
    };

    // this.add('-activate', source, 'ability: Symbiosis', myItem, `[of] ${pokemon}`);
    battle.add(
        "-activate",
        &[
            source_slot.as_str().into(),
            "ability: Symbiosis".into(),
            item_name.into(),
            format!("[of] {}", pokemon_slot).into(),
        ],
    );

    EventResult::Continue
}

