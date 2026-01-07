//! Grim Neigh Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onSourceAfterFaint(length, target, source, effect) {
///     if (effect && effect.effectType === 'Move') {
///         this.boost({ spa: length }, source);
///     }
/// }
pub fn on_source_after_faint(battle: &mut Battle, _length: i32, _target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, effect_id: Option<&str>) -> EventResult {
    // Check if effect is a Move
    if let Some(eff_id) = effect_id {
        if let Some(_move_data) = battle.dex.moves().get(eff_id) {
            // Effect is a move, boost Special Attack by 1
            if let Some(src_pos) = source_pos {
                battle.boost(&[("spa", 1)], src_pos, None, effect_id, false, false);
            }
        }
    }
    EventResult::Continue
}

