//! Ice Scales Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onSourceModifyDamage(damage, source, target, move) {
///     if (move.category === 'Special') {
///         return this.chainModify(0.5);
///     }
/// }
pub fn on_source_modify_damage(battle: &mut Battle, _damage: i32, _source_pos: (usize, usize), _target_pos: (usize, usize), active_move: Option<&crate::battle_actions::ActiveMove>) -> EventResult {
    // JavaScript checks move.category (the active move's category, not the dex category)
    // This is important for moves like Shell Side Arm which can change category
    if active_move.map(|m| m.category == "Special").unwrap_or(false) {
        battle.chain_modify(0.5);
    }
    EventResult::Continue
}
