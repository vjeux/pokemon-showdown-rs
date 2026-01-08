//! Justified Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onDamagingHit(damage, target, source, move) {
///     if (move.type === 'Dark') {
///         this.boost({ atk: 1 });
///     }
/// }
pub fn on_damaging_hit(battle: &mut Battle, _damage: i32, target_pos: Option<(usize, usize)>, _source_pos: Option<(usize, usize)>, active_move: Option<&crate::battle_actions::ActiveMove>) -> EventResult { let move_id = active_move.map(|m| m.id.as_str()).unwrap_or("");
    // Boost Attack by 1 stage when hit by a Dark-type move
    if let Some(target) = target_pos {
        // Check if the move is Dark-type
        let is_dark_type = {
            let move_data = match battle.dex.moves().get(move_id) {
                Some(m) => m,
                None => return EventResult::Continue,
            };
            move_data.move_type == "Dark"
        };

        if is_dark_type {
            battle.boost(&[("atk", 1)], target, None, None, false, false);
        }
    }
    EventResult::Continue
}

