//! Guardian of Alola Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// damageCallback(pokemon, target) {
///     const hp75 = Math.floor(target.getUndynamaxedHP() * 3 / 4);
///     if (
///         target.volatiles['protect'] || target.volatiles['banefulbunker'] || target.volatiles['kingsshield'] ||
///         target.volatiles['spikyshield'] || target.side.getSideCondition('matblock')
///     ) {
///         this.add('-zbroken', target);
///         return this.clampIntRange(Math.ceil(hp75 / 4 - 0.5), 1);
///     }
///     return this.clampIntRange(hp75, 1);
/// }
pub fn damage_callback(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

