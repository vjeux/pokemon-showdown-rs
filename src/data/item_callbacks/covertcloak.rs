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
    secondaries: &mut Vec<crate::battle_actions::SecondaryEffect>,
) -> EventResult {
    // this.debug('Covert Cloak prevent secondary');
    battle.debug("Covert Cloak prevent secondary");

    // return secondaries.filter(effect => !!effect.self);
    // In Rust with mutable ref, use retain() to modify in place
    secondaries.retain(|effect| effect.self_effect);

    EventResult::Continue
}
