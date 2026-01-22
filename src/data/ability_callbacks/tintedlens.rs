//! Tinted Lens Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::battle::Effect;
use crate::event::EventResult;

/// onModifyDamage(damage, source, target, move) {
///     if (target.getMoveHitData(move).typeMod < 0) {
///         this.debug('Tinted Lens boost');
///         return this.chainModify(2);
///     }
/// }
pub fn on_modify_damage(battle: &mut Battle, _damage: i32, _source_pos: (usize, usize), target_pos: (usize, usize), _active_move: Option<&crate::battle_actions::ActiveMove>) -> EventResult {
    // if (target.getMoveHitData(move).typeMod < 0)
    if let Some(hit_data) = battle.get_move_hit_data(target_pos) {
        if hit_data.type_mod < 0 {
            debug_elog!("Tinted Lens boost");
            battle.chain_modify(2.0); return EventResult::Continue;
        }
    }
    EventResult::Continue
}

