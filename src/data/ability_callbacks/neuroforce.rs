//! Neuroforce Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onModifyDamage(damage, source, target, move) {
///     if (move && target.getMoveHitData(move).typeMod > 0) {
///         return this.chainModify([5120, 4096]);
///     }
/// }
pub fn on_modify_damage(battle: &mut Battle, _damage: i32, _source_pos: (usize, usize), target_pos: (usize, usize), _move_id: &str) -> EventResult {
    // if (move && target.getMoveHitData(move).typeMod > 0)
    if let Some(hit_data) = battle.get_move_hit_data(target_pos) {
        if hit_data.type_mod > 0 {
            return EventResult::Number(battle.chain_modify_fraction(5120, 4096));
        }
    }

    EventResult::Continue
}

