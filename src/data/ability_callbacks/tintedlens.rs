//! Tinted Lens Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onModifyDamage(damage, source, target, move) {
///     if (target.getMoveHitData(move).typeMod < 0) {
///         this.debug('Tinted Lens boost');
///         return this.chainModify(2);
///     }
/// }
pub fn on_modify_damage(battle: &mut Battle, _damage: i32, _source_pos: (usize, usize), target_pos: (usize, usize), _move_id: &str) -> EventResult {
    // if (target.getMoveHitData(move).typeMod < 0)
    if let Some(hit_data) = battle.get_move_hit_data(target_pos) {
        if hit_data.type_mod < 0 {
            eprintln!("Tinted Lens boost");
            return EventResult::Number(battle.chain_modify(2.0));
        }
    }
    EventResult::Continue
}

