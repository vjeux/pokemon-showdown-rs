//! Knock Off Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;
use crate::Pokemon;

/// onBasePower(basePower, source, target, move) {
///     const item = target.getItem();
///     if (!this.singleEvent('TakeItem', item, target.itemState, target, target, move, item)) return;
///     if (item.id) {
///         return this.chainModify(1.5);
///     }
/// }
pub fn on_base_power(
    battle: &mut Battle,
    _base_power: i32,
    pokemon_pos: (usize, usize),
    target_pos: Option<(usize, usize)>,
) -> EventResult {
    use crate::dex_data::ID;

    let target = match target_pos {
        Some(pos) => pos,
        None => {
            eprintln!("KNOCKOFF BasePower: No target_pos");
            return EventResult::Continue;
        }
    };

    eprintln!("KNOCKOFF BasePower: source=p{}{}, target=p{}{}",
        pokemon_pos.0 + 1, if pokemon_pos.1 == 0 { "a" } else { "b" },
        target.0 + 1, if target.1 == 0 { "a" } else { "b" });

    // const item = target.getItem();
    let item_id = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => {
                eprintln!("KNOCKOFF BasePower: No target pokemon");
                return EventResult::Continue;
            }
        };
        eprintln!("KNOCKOFF BasePower: target item = '{}'", target_pokemon.item);
        target_pokemon.item.clone()
    };

    // Check if item is empty
    if item_id == ID::from("") {
        eprintln!("KNOCKOFF BasePower: Item is empty, no boost");
        return EventResult::Continue;
    }

    // if (!this.singleEvent('TakeItem', item, target.itemState, target, target, move, item)) return;
    let result = battle.single_event("TakeItem", &item_id, Some(target), Some(target), None, None);
    if let EventResult::Boolean(false) = result {
        eprintln!("KNOCKOFF BasePower: TakeItem returned false");
        return EventResult::Continue;
    }

    // if (item.id) {
    //     return this.chainModify(1.5);
    // }
    eprintln!("KNOCKOFF BasePower: Applying 1.5x boost!");
    battle.chain_modify(1.5_f32);
    EventResult::Continue
}

/// onAfterHit(target, source) {
///     if (source.hp) {
///         const item = target.takeItem();
///         if (item) {
///             this.add('-enditem', target, item.name, '[from] move: Knock Off', `[of] ${source}`);
///         }
///     }
/// }
pub fn on_after_hit(
    battle: &mut Battle,
    source_pos: (usize, usize),
    target_pos: (usize, usize),
) -> EventResult {

    eprintln!("KNOCKOFF AfterHit: source=p{}{}, target=p{}{}",
        source_pos.0 + 1, if source_pos.1 == 0 { "a" } else { "b" },
        target_pos.0 + 1, if target_pos.1 == 0 { "a" } else { "b" });

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
        let item = Pokemon::take_item(battle, target, Some(source));
        eprintln!("KNOCKOFF AfterHit: Removed item {:?}", item);
        item
    };

    //     if (item) {
    //         this.add('-enditem', target, item.name, '[from] move: Knock Off', `[of] ${source}`);
    //     }
    if let Some(item_id) = taken_item {
        let item_name = battle
            .dex
            .items()
            .get_by_id(&item_id)
            .map(|i| i.name.clone())
            .unwrap_or_else(|| item_id.to_string());

        let target_arg = {
            let pokemon = match battle.pokemon_at(target.0, target.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon.get_slot()
        };
        let source_arg = {
            let pokemon = match battle.pokemon_at(source.0, source.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon.get_slot()
        };
        battle.add(
            "-enditem",
            &[
                target_arg.clone().into(),
                item_name.into(),
                "[from] move: Knock Off".into(),
                format!("[of] {}", source_arg).into(),
            ],
        );
    }

    EventResult::Continue
}
