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
    pub fn on_update(_battle: &mut Battle, _pokemon_pos: (usize, usize)) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }
}
