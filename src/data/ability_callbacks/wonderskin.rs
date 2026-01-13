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
pub fn on_modify_accuracy(_battle: &mut Battle, _accuracy: i32, _target_pos: (usize, usize), _source_pos: (usize, usize), active_move: Option<&crate::battle_actions::ActiveMove>) -> EventResult {
    // JavaScript checks move.category (the active move's category, not the dex category)
    if active_move.map(|m| m.category == "Status").unwrap_or(false) {
        return EventResult::Number(50);
    }
    EventResult::Continue
}

