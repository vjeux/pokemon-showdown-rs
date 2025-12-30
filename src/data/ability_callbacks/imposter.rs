//! Imposter Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onSwitchIn(pokemon) {
///     // Imposter does not activate when Skill Swapped or when Neutralizing Gas leaves the field
///     // Imposter copies across in doubles/triples
///     // (also copies across in multibattle and diagonally in free-for-all,
///     // but side.foe already takes care of those)
///     const target = pokemon.side.foe.active[pokemon.side.foe.active.length - 1 - pokemon.position];
///     if (target) {
///         pokemon.transformInto(target, this.dex.abilities.get('imposter'));
///     }
/// }
pub fn on_switch_in(_battle: &mut Battle, _pokemon_pos: (usize, usize)) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

