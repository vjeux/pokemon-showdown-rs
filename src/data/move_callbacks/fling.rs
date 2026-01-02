//! Fling Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::dex_data::ID;
use crate::event::EventResult;
use crate::pokemon::Pokemon;

/// onPrepareHit(target, source, move) {
///     if (source.ignoringItem(true)) return false;
///     const item = source.getItem();
///     if (!this.singleEvent('TakeItem', item, source.itemState, source, source, move, item)) return false;
///     if (!item.fling) return false;
///     move.basePower = item.fling.basePower;
///     this.debug(`BP: ${move.basePower}`);
///     if (item.isBerry) {
///         if (source.hasAbility('cudchew')) {
///             this.singleEvent('EatItem', source.getAbility(), source.abilityState, source, source, move, item);
///         }
///         move.onHit = function (foe) {
///             if (this.singleEvent('Eat', item, source.itemState, foe, source, move)) {
///                 this.runEvent('EatItem', foe, source, move, item);
///                 if (item.id === 'leppaberry') foe.staleness = 'external';
///             }
///             if (item.onEat) foe.ateBerry = true;
///         };
///     } else if (item.fling.effect) {
///         move.onHit = item.fling.effect;
///     } else {
///         if (!move.secondaries) move.secondaries = [];
///         if (item.fling.status) {
///             move.secondaries.push({ status: item.fling.status });
///         } else if (item.fling.volatileStatus) {
///             move.secondaries.push({ volatileStatus: item.fling.volatileStatus });
///         }
///     }
///     source.addVolatile('fling');
/// }
pub fn on_prepare_hit(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
) -> EventResult {
    let pokemon = pokemon_pos;

    // if (source.ignoringItem(true)) return false;
    let ignoring_item = {
        let pokemon_ref = match battle.pokemon_at(pokemon.0, pokemon.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon_ref.ignoring_item(battle, true)
    };

    if ignoring_item {
        return EventResult::Boolean(false);
    }

    // const item = source.getItem();
    let item_id = {
        let pokemon_ref = match battle.pokemon_at(pokemon.0, pokemon.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon_ref.get_item().clone()
    };

    // if (!this.singleEvent('TakeItem', item, source.itemState, source, source, move, item)) return false;
    let take_item_result = battle.single_event(
        "TakeItem",
        &item_id,
        Some(pokemon),
        Some(pokemon),
        Some(&item_id),
    );
    if take_item_result.boolean() == Some(false) {
        return EventResult::Boolean(false);
    }

    // if (!item.fling) return false;
    let fling_data = {
        battle.dex.items().get_by_id(&item_id)
            .and_then(|item| item.fling.clone())
    };

    let fling = match fling_data {
        Some(f) => f,
        None => return EventResult::Boolean(false),
    };

    // move.basePower = item.fling.basePower;
    if let Some(ref mut active_move) = battle.active_move {
        active_move.base_power = fling.base_power;
    }

    // this.debug(`BP: ${move.basePower}`);
    battle.debug(&format!("BP: {}", fling.base_power));

    // TODO: The following logic requires dynamic callback assignment which is incompatible with Rust's type system:
    // - if (item.isBerry) { move.onHit = function (foe) { ... } }
    // - else if (item.fling.effect) { move.onHit = item.fling.effect; }
    //
    // These would require either:
    // 1. Adding callback storage to ActiveMove (major architectural change)
    // 2. Handling berry eating/effect logic directly in fling's onHit callback
    // 3. Using a callback registry system
    //
    // However, we CAN handle the secondaries case:

    // Get item.isBerry to skip the secondaries logic for berries
    let is_berry = {
        battle.dex.items().get_by_id(&item_id)
            .map(|item| item.is_berry)
            .unwrap_or(false)
    };

    if let Some(ref mut active_move) = battle.active_move {
        // if (item.isBerry) { ... } - Skip, requires dynamic callback
        // else if (item.fling.effect) { ... } - Skip, requires dynamic callback
        // else { ... } - We can implement this part!
        if !is_berry && fling.effect.is_none() {
            // if (!move.secondaries) move.secondaries = [];
            // (secondaries already exists as Vec, so we just push to it)

            // if (item.fling.status) { move.secondaries.push({ status: item.fling.status }); }
            if let Some(status) = fling.status {
                active_move.secondaries.push(crate::battle_actions::SecondaryEffect {
                    status: Some(status),
                    ..Default::default()
                });
            }
            // else if (item.fling.volatileStatus) { move.secondaries.push({ volatileStatus: item.fling.volatileStatus }); }
            else if let Some(volatile_status) = fling.volatile_status {
                active_move.secondaries.push(crate::battle_actions::SecondaryEffect {
                    volatile_status: Some(volatile_status),
                    ..Default::default()
                });
            }
        }
    }

    // source.addVolatile('fling');
    Pokemon::add_volatile(battle, pokemon, ID::from("fling"), None);

    EventResult::Continue
}

pub mod condition {
    use super::*;

    /// onUpdate(pokemon) {
    ///     const item = pokemon.getItem();
    ///     pokemon.setItem('');
    ///     pokemon.lastItem = item.id;
    ///     pokemon.usedItemThisTurn = true;
    ///     this.add('-enditem', pokemon, item.name, '[from] move: Fling');
    ///     this.runEvent('AfterUseItem', pokemon, null, null, item);
    ///     pokemon.removeVolatile('fling');
    /// }
    pub fn on_update(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
        use crate::dex_data::ID;

        let pokemon = pokemon_pos;

        // const item = pokemon.getItem();
        let item_id = {
            let pokemon_ref = match battle.pokemon_at(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon_ref.get_item().clone()
        };

        // If no item, nothing to do
        if item_id.is_empty() {
            return EventResult::Continue;
        }

        // Get item name for the battle log
        let item_name = {
            let item_data = battle.dex.items().get_by_id(&item_id);
            item_data
                .map(|i| i.name.clone())
                .unwrap_or_else(|| item_id.to_string())
        };

        // Get pokemon slot for battle log
        let pokemon_slot = {
            let pokemon_ref = match battle.pokemon_at(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon_ref.get_slot()
        };

        // pokemon.setItem('');
        // pokemon.lastItem = item.id;
        // pokemon.usedItemThisTurn = true;
        {
            let pokemon_mut = match battle.pokemon_at_mut(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon_mut.set_item(ID::empty());
            pokemon_mut.last_item = item_id.clone();
            pokemon_mut.used_item_this_turn = true;
        }

        // this.add('-enditem', pokemon, item.name, '[from] move: Fling');
        battle.add(
            "-enditem",
            &[
                crate::battle::Arg::from(pokemon_slot),
                crate::battle::Arg::from(item_name),
                crate::battle::Arg::from("[from] move: Fling"),
            ],
        );

        // this.runEvent('AfterUseItem', pokemon, null, null, item);
        battle.run_event("AfterUseItem", Some(pokemon), None, Some(&item_id), None);

        // pokemon.removeVolatile('fling');
        let pokemon_mut = match battle.pokemon_at_mut(pokemon.0, pokemon.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon_mut.remove_volatile(&ID::from("fling"));

        EventResult::Continue
    }
}
