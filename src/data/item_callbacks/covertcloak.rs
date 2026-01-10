//! Covert Cloak Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onModifySecondaries(secondaries) {
///     this.debug('Covert Cloak prevent secondary');
///     return secondaries.filter(effect => !!effect.self);
/// }
pub fn on_modify_secondaries(
    battle: &mut Battle,
    _pokemon_pos: (usize, usize),
    secondaries: &Vec<crate::dex::MoveSecondary>,
) -> EventResult {
    // return secondaries.filter(effect => !!effect.self);
    // Filter out opponent-targeting secondaries, keep only self-targeting ones
    let filtered: Vec<_> = secondaries
        .iter()
        .filter(|effect| effect.self_secondary.is_some())
        .cloned()
        .collect();

    if filtered.len() < secondaries.len() {
        // this.debug('Covert Cloak prevent secondary');
        battle.debug("Covert Cloak prevent secondary");
    }

    // Return the filtered secondaries so the caller uses them
    EventResult::Secondaries(filtered)
}
