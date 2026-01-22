//! Stamina Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::battle::Effect;
use crate::event::EventResult;

/// onDamagingHit(damage, target, source, effect) {
///     this.boost({ def: 1 });
/// }
pub fn on_damaging_hit(battle: &mut Battle, _damage: i32, target_pos: Option<(usize, usize)>, _source_pos: Option<(usize, usize)>, _active_move: Option<&crate::battle_actions::ActiveMove>) -> EventResult {
    // Boost Defense by 1 stage when hit by a damaging move
    if let Some(target) = target_pos {
        battle.boost(&[("def", 1)], target, None, None, false, false);
    }
    EventResult::Continue
}

