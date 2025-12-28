//! Quick Powder Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onModifySpe(spe, pokemon) {
///     if (pokemon.species.name === 'Ditto' && !pokemon.transformed) {
///         return this.chainModify(2);
///     }
/// }
pub fn on_modify_spe(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // if (pokemon.species.name === 'Ditto' && !pokemon.transformed) {
    //     return this.chainModify(2);
    // }
    // TODO: Need pokemon.species.name and pokemon.transformed to check if Ditto
    // Also need ability to modify speed value by chainModify(2)
    EventResult::Continue
}
