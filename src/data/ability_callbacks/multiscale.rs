//! Multiscale Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onSourceModifyDamage(damage, source, target, move) {
///     if (target.hp >= target.maxhp) {
///         this.debug('Multiscale weaken');
///         return this.chainModify(0.5);
///     }
/// }
pub fn on_source_modify_damage(battle: &mut Battle, _damage: i32, _source_pos: (usize, usize), target_pos: (usize, usize), _active_move: Option<&crate::battle_actions::ActiveMove>) -> EventResult {
    if let Some(target) = battle.pokemon_at(target_pos.0, target_pos.1) {
        if target.hp >= target.maxhp {
            let modified = battle.chain_modify(0.5);
            return EventResult::Number(modified);
        }
    }
    EventResult::Continue
}

