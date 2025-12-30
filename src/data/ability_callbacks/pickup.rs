//! Pickup Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onResidual(pokemon) {
///     if (pokemon.item) return;
///     const pickupTargets = this.getAllActive().filter(target => (
///         target.lastItem && target.usedItemThisTurn && pokemon.isAdjacent(target)
///     ));
///     if (!pickupTargets.length) return;
///     const randomTarget = this.sample(pickupTargets);
///     const item = randomTarget.lastItem;
///     randomTarget.lastItem = '';
///     this.add('-item', pokemon, this.dex.items.get(item), '[from] ability: Pickup');
///     pokemon.setItem(item);
/// }
pub fn on_residual(_battle: &mut Battle, _pokemon_pos: (usize, usize)) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

