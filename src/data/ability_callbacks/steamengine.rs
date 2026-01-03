//! Steam Engine Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onDamagingHit(damage, target, source, move) {
///     if (['Water', 'Fire'].includes(move.type)) {
///         this.boost({ spe: 6 });
///     }
/// }
pub fn on_damaging_hit(battle: &mut Battle, _damage: i32, target_pos: Option<(usize, usize)>, _source_pos: Option<(usize, usize)>, move_id: &str) -> EventResult {
    // Boost Speed by 6 stages when hit by a Water or Fire-type move
    if let Some(target) = target_pos {
        // Check if the move is Water or Fire-type
        let is_water_or_fire = {
            let move_data = match battle.dex.moves().get(move_id) {
                Some(m) => m,
                None => return EventResult::Continue,
            };
            move_data.move_type == "Water" || move_data.move_type == "Fire"
        };

        if is_water_or_fire {
            battle.boost(&[("spe", 6)], target, None, None, false, false);
        }
    }
    EventResult::Continue
}

