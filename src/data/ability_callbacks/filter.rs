//! Filter Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::battle::Effect;
use crate::event::EventResult;

/// onSourceModifyDamage(damage, source, target, move) {
///     if (target.getMoveHitData(move).typeMod > 0) {
///         this.debug('Filter neutralize');
///         return this.chainModify(0.75);
///     }
/// }
pub fn on_source_modify_damage(battle: &mut Battle, _damage: i32, _source_pos: (usize, usize), target_pos: (usize, usize), _active_move: Option<&crate::battle_actions::ActiveMove>) -> EventResult {
    // if (target.getMoveHitData(move).typeMod > 0) {
    //     this.debug('Filter neutralize');
    //     return this.chainModify(0.75);
    // }

    // Check if the move is super-effective
    if let Some(move_hit_data) = battle.get_move_hit_data(target_pos) {
        if move_hit_data.type_mod > 0 {
            debug_elog!("Filter neutralize");
            battle.chain_modify(0.75); return EventResult::Continue;
        }
    }

    EventResult::Continue
}

