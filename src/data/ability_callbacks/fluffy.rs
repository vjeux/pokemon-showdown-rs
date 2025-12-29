//! Fluffy Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onSourceModifyDamage(damage, source, target, move) {
///     let mod = 1;
///     if (move.type === 'Fire') mod *= 2;
///     if (move.flags['contact']) mod /= 2;
///     return this.chainModify(mod);
/// }
pub fn on_source_modify_damage(battle: &mut Battle, damage: i32, source_pos: (usize, usize), target_pos: (usize, usize), move_id: &str) -> EventResult {
    if let Some(move_data) = battle.dex.get_move(move_id) {
        let mut mod_value = 1.0;
        if move_data.move_type == "Fire" {
            mod_value *= 2.0;
        }
        if move_data.flags.contains_key("contact") {
            mod_value /= 2.0;
        }
        let modified = battle.chain_modify(mod_value);
        return EventResult::Number(modified);
    }
    EventResult::Continue
}

