//! Wonder Skin Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onModifyAccuracy(accuracy, target, source, move) {
///     if (move.category === 'Status' && typeof accuracy === 'number') {
///         this.debug('Wonder Skin - setting accuracy to 50');
///         return 50;
///     }
/// }
pub fn on_modify_accuracy(battle: &mut Battle, _accuracy: i32, _target_pos: (usize, usize), _source_pos: (usize, usize), move_id: &str) -> EventResult {
    if let Some(move_data) = battle.dex.moves().get(move_id) {
        if move_data.category == "Status" {
            return EventResult::Number(50);
        }
    }
    EventResult::Continue
}

