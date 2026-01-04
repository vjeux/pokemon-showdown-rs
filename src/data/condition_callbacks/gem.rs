//! Gem Condition
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! JavaScript source: data/conditions.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onBasePower
/// JavaScript source (data/conditions.ts):
/// ```js
/// onBasePowerPriority: 14,
/// onBasePower(basePower, user, target, move) {
///     this.debug('Gem Boost');
///     return this.chainModify([5325, 4096]);
/// }
/// ```
pub fn on_base_power(
    battle: &mut Battle,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    // this.debug('Gem Boost');
    // In Rust, we use eprintln for debug messages (similar to JavaScript implementation)

    // return this.chainModify([5325, 4096]);
    EventResult::Number(battle.chain_modify_fraction(5325, 4096))
}

