//! Knock Off Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onBasePower(basePower, source, target, move) {
///     const item = target.getItem();
///     if (!this.singleEvent('TakeItem', item, target.itemState, target, target, move, item)) return;
///     if (item.id) {
///         return this.chainModify(1.5);
///     }
/// }
pub fn on_base_power(battle: &mut Battle, base_power: i32, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    use crate::dex_data::ID;

    let target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // const item = target.getItem();
    let item_id = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target_pokemon.item.clone()
    };

    // Check if item is empty
    if item_id == ID::from("") {
        return EventResult::Continue;
    }

    // if (!this.singleEvent('TakeItem', item, target.itemState, target, target, move, item)) return;
    let can_take_item = battle.run_single_event("TakeItem", Some(&item_id), target, target, None);
    if !can_take_item {
        return EventResult::Continue;
    }

    // if (item.id) {
    //     return this.chainModify(1.5);
    // }
    EventResult::Number(battle.chain_modify(1.5 as f32))
}

/// onAfterHit(target, source) {
///     if (source.hp) {
///         const item = target.takeItem();
///         if (item) {
///             this.add('-enditem', target, item.name, '[from] move: Knock Off', `[of] ${source}`);
///         }
///     }
/// }
pub fn on_after_hit(battle: &mut Battle, source_pos: (usize, usize), target_pos: (usize, usize)) -> EventResult {
    let source = source_pos;
    let target = target_pos;

    // if (source.hp) {
    let source_has_hp = {
        let source_pokemon = match battle.pokemon_at(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        source_pokemon.hp > 0
    };

    if !source_has_hp {
        return EventResult::Continue;
    }

    //     const item = target.takeItem();
    let taken_item = {
        let target_pokemon = match battle.pokemon_at_mut(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target_pokemon.take_item()
    };

    //     if (item) {
    //         this.add('-enditem', target, item.name, '[from] move: Knock Off', `[of] ${source}`);
    //     }
    if let Some(item_id) = taken_item {
        let item_name = battle.dex.get_item_by_id(&item_id)
            .map(|i| i.name.clone())
            .unwrap_or_else(|| item_id.to_string());

        let target_arg = crate::battle::Arg::Pos(target.0, target.1);
        let source_arg = crate::battle::Arg::Pos(source.0, source.1);
        battle.add("-enditem", &[
            target_arg,
            item_name.into(),
            "[from] move: Knock Off".into(),
            format!("[of] {}", source_arg).into(),
        ]);
    }

    EventResult::Continue
}

