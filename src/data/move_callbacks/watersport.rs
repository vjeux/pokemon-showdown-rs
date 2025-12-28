//! Water Sport Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

pub mod condition {
    use super::*;

    /// onFieldStart(field, source) {
    ///     this.add('-fieldstart', 'move: Water Sport', `[of] ${source}`);
    /// }
    pub fn on_field_start(
        _battle: &mut Battle,
        _field_pos: Option<(usize, usize)>,
        _source_pos: Option<(usize, usize)>,
    ) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }

    /// onBasePower(basePower, attacker, defender, move) {
    ///     if (move.type === 'Fire') {
    ///         this.debug('water sport weaken');
    ///         return this.chainModify([1352, 4096]);
    ///     }
    /// }
    pub fn on_base_power(
        _battle: &mut Battle,
        _base_power: i32,
        _pokemon_pos: (usize, usize),
        _target_pos: Option<(usize, usize)>,
    ) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }

    /// onFieldEnd() {
    ///     this.add('-fieldend', 'move: Water Sport');
    /// }
    pub fn on_field_end(_battle: &mut Battle) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }
}
