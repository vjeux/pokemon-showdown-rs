//! Sap Sipper Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onTryHit(target, source, move) {
///     if (target !== source && move.type === 'Grass') {
///         if (!this.boost({ atk: 1 })) {
///             this.add('-immune', target, '[from] ability: Sap Sipper');
///         }
///         return null;
///     }
/// }
pub fn on_try_hit(battle: &mut Battle, target_pos: (usize, usize), source_pos: (usize, usize), move_id: &str) -> EventResult {
    // Immune to Grass-type moves and boost Attack by 1
    if target_pos != source_pos {
        // Check if the move is Grass-type
        let is_grass = {
            let move_data = match battle.dex.moves().get(move_id) {
                Some(m) => m,
                None => return EventResult::Continue,
            };
            move_data.move_type == "Grass"
        };

        if is_grass {
            // Try to boost Attack
            battle.boost(&[("atk", 1)], target_pos, None, None, false, false);
            // Return Null to prevent the move from hitting
            return EventResult::Null;
        }
    }
    EventResult::Continue
}

/// onAllyTryHitSide(target, source, move) {
///     if (source === this.effectState.target || !target.isAlly(source)) return;
///     if (move.type === 'Grass') {
///         this.boost({ atk: 1 }, this.effectState.target);
///     }
/// }
pub fn on_ally_try_hit_side(_battle: &mut Battle, _target_pos: Option<(usize, usize)>, _source_pos: Option<(usize, usize)>, _move_id: &str) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

