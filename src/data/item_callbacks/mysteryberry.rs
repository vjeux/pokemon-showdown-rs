//! Mystery Berry Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onUpdate(pokemon) {
///     if (!pokemon.hp) return;
///     const moveSlot = pokemon.lastMove && pokemon.getMoveData(pokemon.lastMove.id);
///     if (moveSlot && moveSlot.pp === 0) {
///         pokemon.addVolatile('leppaberry');
///         pokemon.volatiles['leppaberry'].moveSlot = moveSlot;
///         pokemon.eatItem();
///     }
/// }
pub fn on_update(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

/// onEat(pokemon) {
///     let moveSlot;
///     if (pokemon.volatiles['leppaberry']) {
///         moveSlot = pokemon.volatiles['leppaberry'].moveSlot;
///         pokemon.removeVolatile('leppaberry');
///     } else {
///         let pp = 99;
///         for (const possibleMoveSlot of pokemon.moveSlots) {
///             if (possibleMoveSlot.pp < pp) {
///                 moveSlot = possibleMoveSlot;
///                 pp = moveSlot.pp;
///             }
///         }
///     }
///     moveSlot.pp += 5;
///     if (moveSlot.pp > moveSlot.maxpp) moveSlot.pp = moveSlot.maxpp;
///     this.add('-activate', pokemon, 'item: Mystery Berry', moveSlot.move);
/// }
pub fn on_eat(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}
