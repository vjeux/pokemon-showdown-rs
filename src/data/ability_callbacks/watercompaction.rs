//! Water Compaction Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onDamagingHit(damage, target, source, move) {
///     if (move.type === 'Water') {
///         this.boost({ def: 2 });
///     }
/// }
pub fn on_damaging_hit(battle: &mut Battle, _damage: i32, target_pos: Option<(usize, usize)>, _source_pos: Option<(usize, usize)>, active_move: Option<&crate::battle_actions::ActiveMove>) -> EventResult { let move_id = active_move.map(|m| m.id.as_str()).unwrap_or("");
    // Boost Defense by 2 stages when hit by a Water-type move
    if let Some(target) = target_pos {
        // Check if the move is Water-type
        let is_water_type = {
            let move_data = match battle.dex.moves().get(move_id) {
                Some(m) => m,
                None => return EventResult::Continue,
            };
            move_data.move_type == "Water"
        };

        if is_water_type {
            battle.boost(&[("def", 2)], target, None, None, false, false);
        }
    }
    EventResult::Continue
}

