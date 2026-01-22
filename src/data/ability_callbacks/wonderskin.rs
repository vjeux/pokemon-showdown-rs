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
pub fn on_modify_accuracy(_battle: &mut Battle, accuracy: i32, _target_pos: (usize, usize), _source_pos: (usize, usize), active_move: Option<&crate::battle_actions::ActiveMove>) -> EventResult {
    // JavaScript checks typeof accuracy === 'number' to skip boolean true values
    // In Rust, accuracy=0 represents boolean true (e.g., accuracy: true on status moves)
    // If accuracy is 0 (boolean true), don't apply the modifier
    if accuracy == 0 {
        return EventResult::Continue;
    }
    // JavaScript checks move.category (the active move's category, not the dex category)
    if active_move.map(|m| m.category == "Status").unwrap_or(false) {
        return EventResult::Number(50);
    }
    EventResult::Continue
}

