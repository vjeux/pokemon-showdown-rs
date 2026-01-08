//! Sniper Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onModifyDamage(damage, source, target, move) {
///     if (target.getMoveHitData(move).crit) {
///         this.debug('Sniper boost');
///         return this.chainModify(1.5);
///     }
/// }
pub fn on_modify_damage(battle: &mut Battle, _damage: i32, _source_pos: (usize, usize), target_pos: (usize, usize), _active_move: Option<&crate::battle_actions::ActiveMove>) -> EventResult {
    // if (target.getMoveHitData(move).crit)
    if let Some(hit_data) = battle.get_move_hit_data(target_pos) {
        if hit_data.crit {
            eprintln!("Sniper boost");
            battle.chain_modify(1.5); return EventResult::Continue;
        }
    }
    EventResult::Continue
}

