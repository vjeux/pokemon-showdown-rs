//! Gem Condition
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! JavaScript source: data/conditions.ts

use crate::battle::Battle;
use crate::battle::Effect;
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
    _base_power: i32,
    _pokemon_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
    _active_move: Option<&crate::battle_actions::ActiveMove>,
) -> EventResult {
    // this.debug('Gem Boost');
    // In Rust, we use eprintln for debug messages (similar to JavaScript implementation)

    // return this.chainModify([5325, 4096]);
    { battle.chain_modify_fraction(5325, 4096); EventResult::Continue }
}

