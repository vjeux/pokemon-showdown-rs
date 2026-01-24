//! Natural Gift Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::{Battle, Effect};
use crate::event::EventResult;

/// onModifyType(move, pokemon) {
///     if (pokemon.ignoringItem()) return;
///     const item = pokemon.getItem();
///     if (!item.naturalGift) return;
///     move.type = item.naturalGift.type;
/// }
pub fn on_modify_type(
    battle: &mut Battle,
    _active_move: Option<&crate::battle_actions::ActiveMove>,
    pokemon_pos: (usize, usize),
) -> EventResult {
    use crate::dex_data::ID;

    let pokemon = pokemon_pos;

    // if (pokemon.ignoringItem()) return;
    let ignoring_item = {
        let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon_pokemon.ignoring_item(battle, false)
    };

    if ignoring_item {
        return EventResult::Continue;
    }

    // const item = pokemon.getItem();
    let item_id = {
        let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon_pokemon.item.clone()
    };

    // Check if item is empty
    if item_id == ID::from("") {
        return EventResult::Continue;
    }

    let item_data = match battle.dex.items().get_by_id(&item_id) {
        Some(item) => item,
        None => return EventResult::Continue,
    };

    // if (!item.naturalGift) return;
    let natural_gift = match &item_data.natural_gift {
        Some(ng) => ng,
        None => return EventResult::Continue,
    };

    // move.type = item.naturalGift.type;
    if let Some(ref mut active_move) = battle.active_move {
        if let Some(move_type_str) = natural_gift.get("type").and_then(|v| v.as_str()) {
            active_move.borrow_mut().move_type = move_type_str.to_string();
        }
    }

    EventResult::Continue
}

/// onPrepareHit(target, pokemon, move) {
///     if (pokemon.ignoringItem()) return false;
///     const item = pokemon.getItem();
///     if (!item.naturalGift) return false;
///     move.basePower = item.naturalGift.basePower;
///     this.debug(`BP: ${move.basePower}`);
///     pokemon.setItem('');
///     pokemon.lastItem = item.id;
///     pokemon.usedItemThisTurn = true;
///     this.runEvent('AfterUseItem', pokemon, null, null, item);
/// }
pub fn on_prepare_hit(
    battle: &mut Battle,
    _target_pos: (usize, usize),
    source_pos: Option<(usize, usize)>,
) -> EventResult {
    use crate::dex_data::ID;

    // In JS signature: onPrepareHit(target, pokemon, move)
    // 'pokemon' is the source (user of the move)
    let pokemon = match source_pos {
        Some(pos) => pos,
        None => return EventResult::Boolean(false),
    };

    // if (pokemon.ignoringItem()) return false;
    let ignoring_item = {
        let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
            Some(p) => p,
            None => return EventResult::Boolean(false),
        };
        pokemon_pokemon.ignoring_item(battle, false)
    };

    if ignoring_item {
        return EventResult::Boolean(false);
    }

    // const item = pokemon.getItem();
    let item_id = {
        let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
            Some(p) => p,
            None => return EventResult::Boolean(false),
        };
        pokemon_pokemon.item.clone()
    };

    // Check if item is empty
    if item_id == ID::from("") {
        return EventResult::Boolean(false);
    }

    let item_data = match battle.dex.items().get_by_id(&item_id) {
        Some(item) => item,
        None => return EventResult::Boolean(false),
    };

    // if (!item.naturalGift) return false;
    let natural_gift = match &item_data.natural_gift {
        Some(ng) => ng,
        None => return EventResult::Boolean(false),
    };

    // move.basePower = item.naturalGift.basePower;
    let base_power = natural_gift
        .get("basePower")
        .and_then(|v| v.as_i64())
        .unwrap_or(0) as i32;
    if let Some(ref mut active_move) = battle.active_move {
        active_move.borrow_mut().base_power = base_power;
    }

    // this.debug(`BP: ${move.basePower}`);
    // (debug is typically not needed in Rust implementation)

    // pokemon.setItem('');
    // pokemon.lastItem = item.id;
    // pokemon.usedItemThisTurn = true;
    {
        let pokemon_pokemon = match battle.pokemon_at_mut(pokemon.0, pokemon.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon_pokemon.item = ID::from("");
        pokemon_pokemon.last_item = item_id.clone();
        pokemon_pokemon.used_item_this_turn = true;
    }

    // this.runEvent('AfterUseItem', pokemon, null, null, item);
    battle.run_event("AfterUseItem", Some(crate::event::EventTarget::Pokemon(pokemon)), None, Some(&Effect::item(item_id.clone())), EventResult::Continue, false, false);

    EventResult::Continue
}
