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
pub fn on_try_hit(battle: &mut Battle, target_pos: (usize, usize), source_pos: (usize, usize), active_move: Option<&crate::battle_actions::ActiveMove>) -> EventResult { let move_id = active_move.map(|m| m.id.as_str()).unwrap_or("");
    // Immune to Electric-type moves and boost Speed by 1
    if target_pos != source_pos {
        // Check if the move is Electric-type
        let is_electric = {
            let move_data = match battle.dex.moves().get(move_id) {
                Some(m) => m,
                None => return EventResult::Continue,
            };
            move_data.move_type == "Electric"
        };

        if is_electric {
            // Try to boost Speed
            battle.boost(&[("spe", 1)], target_pos, None, None, false, false);
            // Return Null to prevent the move from hitting
            return EventResult::Null;
        }
    }
    EventResult::Continue
}

