//! Motor Drive Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onTryHit(target, source, move) {
///     if (target !== source && move.type === 'Electric') {
///         if (!this.boost({ spe: 1 })) {
///             this.add('-immune', target, '[from] ability: Motor Drive');
///         }
///         return null;
///     }
/// }
pub fn on_try_hit(battle: &mut Battle, target_pos: (usize, usize), source_pos: (usize, usize), active_move: Option<&crate::battle_actions::ActiveMove>) -> EventResult {
    // Immune to Electric-type moves and boost Speed by 1
    // if (target !== source && move.type === 'Electric') {
    if target_pos != source_pos {
        // Check if the move is Electric-type
        // JavaScript checks move.type (the active move's type, not the dex type)
        // This is important for moves like Electrify which change the move type at runtime
        let is_electric = active_move.map(|m| m.move_type == "Electric").unwrap_or(false);

        if is_electric {
            // Try to boost Speed
            // if (!this.boost({ spe: 1 })) {
            //     this.add('-immune', target, '[from] ability: Motor Drive');
            // }
            battle.boost(&[("spe", 1)], target_pos, None, None, false, false);
            // Return Null to prevent the move from hitting
            // return null;
            return EventResult::Null;
        }
    }
    EventResult::Continue
}

