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
    _battle: &mut Battle,
    _pokemon_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
) -> EventResult {
    // TODO: Implement 1-to-1 from JS
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
