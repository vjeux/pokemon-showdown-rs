//! Storm Drain Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onTryHit(target, source, move) {
///     if (target !== source && move.type === 'Water') {
///         if (!this.boost({ spa: 1 })) {
///             this.add('-immune', target, '[from] ability: Storm Drain');
///         }
///         return null;
///     }
/// }
pub fn on_try_hit(battle: &mut Battle, target_pos: (usize, usize), source_pos: (usize, usize), move_id: &str) -> EventResult {
    // Immune to Water-type moves and boost Special Attack by 1
    if target_pos != source_pos {
        // Check if the move is Water-type
        let is_water = {
            let move_data = match battle.dex.moves().get(move_id) {
                Some(m) => m,
                None => return EventResult::Continue,
            };
            move_data.move_type == "Water"
        };

        if is_water {
            // Boost Special Attack by 1
            battle.boost(&[("spa", 1)], target_pos, None, None, false, false);
            // Return Null to prevent the move from hitting
            return EventResult::Null;
        }
    }
    EventResult::Continue
}

/// onAnyRedirectTarget(target, source, source2, move) {
///     if (move.type !== 'Water' || move.flags['pledgecombo']) return;
///     const redirectTarget = ['randomNormal', 'adjacentFoe'].includes(move.target) ? 'normal' : move.target;
///     if (this.validTarget(this.effectState.target, source, redirectTarget)) {
///         if (move.smartTarget) move.smartTarget = false;
///         if (this.effectState.target !== target) {
///             this.add('-activate', this.effectState.target, 'ability: Storm Drain');
///         }
///         return this.effectState.target;
///     }
/// }
pub fn on_any_redirect_target(_battle: &mut Battle, _target_pos: Option<(usize, usize)>, _source_pos: Option<(usize, usize)>, _move_id: &str) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

