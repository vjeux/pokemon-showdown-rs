//! Grim Neigh Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::battle::Effect;
use crate::battle::EffectType;
use crate::event::EventResult;

/// onSourceAfterFaint(length, target, source, effect) {
///     if (effect && effect.effectType === 'Move') {
///         this.boost({ spa: length }, source);
///     }
/// }
pub fn on_source_after_faint(battle: &mut Battle, _length: i32, _target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, effect: Option<&Effect>) -> EventResult {
    // Check if effect exists and effectType === 'Move'
    let is_move_effect = effect
        .map(|e| e.effect_type == EffectType::Move)
        .unwrap_or(false);

    if is_move_effect {
        // Effect is a move, boost Special Attack by 1
        if let Some(src_pos) = source_pos {
            battle.boost(&[("spa", 1)], src_pos, None, None, false, false);
        }
    }
    EventResult::Continue
}
