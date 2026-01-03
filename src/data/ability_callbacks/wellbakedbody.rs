//! Well-Baked Body Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onTryHit(target, source, move) {
///     if (target !== source && move.type === 'Fire') {
///         if (!this.boost({ def: 2 })) {
///             this.add('-immune', target, '[from] ability: Well-Baked Body');
///         }
///         return null;
///     }
/// }
pub fn on_try_hit(battle: &mut Battle, target_pos: (usize, usize), source_pos: (usize, usize), move_id: &str) -> EventResult {
    // Immune to Fire-type moves and boost Defense by 2
    if target_pos != source_pos {
        // Check if the move is Fire-type
        let is_fire = {
            let move_data = match battle.dex.moves().get(move_id) {
                Some(m) => m,
                None => return EventResult::Continue,
            };
            move_data.move_type == "Fire"
        };

        if is_fire {
            // Try to boost Defense by 2
            battle.boost(&[("def", 2)], target_pos, None, None, false, false);
            // Return Null to prevent the move from hitting
            return EventResult::Null;
        }
    }
    EventResult::Continue
}

