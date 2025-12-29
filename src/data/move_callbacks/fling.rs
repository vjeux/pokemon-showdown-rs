//! Fling Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

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
    use crate::dex_data::ID;

    let pokemon = pokemon_pos;

    // if (source.ignoringItem(true)) return false;
    let ignoring_item = {
        let pokemon_ref = match battle.pokemon_at(pokemon.0, pokemon.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon_ref.ignoring_item()
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
        battle.dex.get_item_by_id(&item_id)
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
    // Debug output would go here, but we don't have a debug method on Battle yet

    // TODO: The following logic requires dynamic callback assignment which is incompatible with Rust's type system:
    // - if (item.isBerry) { move.onHit = function (foe) { ... } }
    // - else if (item.fling.effect) { move.onHit = item.fling.effect; }
    // - else { modify move.secondaries with status/volatileStatus }
    //
    // This would require either:
    // 1. Adding callback storage to ActiveMove (major architectural change)
    // 2. Handling berry eating/effect logic directly in fling's onHit callback
    // 3. Using a callback registry system
    //
    // For now, we can at least handle the secondaries case by checking the fields we added:
    if let Some(ref mut active_move) = battle.active_move {
        // Skip berry and effect cases (require dynamic callbacks)
        if fling.status.is_some() || fling.volatile_status.is_some() {
            // TODO: Need to check if ActiveMove has a secondaries field to modify
            // In TypeScript: if (!move.secondaries) move.secondaries = [];
            // Then push { status: ... } or { volatileStatus: ... }
            // This requires ActiveMove.secondaries field which may not exist yet
        }
    }

    // source.addVolatile('fling');
    {
        let pokemon_mut = match battle.pokemon_at_mut(pokemon.0, pokemon.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon_mut.add_volatile(ID::from("fling"));
    }

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
            let item_data = battle.dex.get_item_by_id(&item_id);
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
