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
pub fn on_source_modify_damage(battle: &mut Battle, _damage: i32, _source_pos: (usize, usize), _target_pos: (usize, usize), active_move: Option<&crate::battle_actions::ActiveMove>) -> EventResult { let move_id = active_move.map(|m| m.id.as_str()).unwrap_or("");
    if let Some(move_data) = battle.dex.moves().get(move_id) {
        if move_data.category == "Special" {
            battle.chain_modify(0.5);
            return EventResult::Continue;
        }
    }
    EventResult::Continue
}

