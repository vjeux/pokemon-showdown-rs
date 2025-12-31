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
pub fn on_source_modify_damage(battle: &mut Battle, _damage: i32, _source_pos: (usize, usize), _target_pos: (usize, usize), move_id: &str) -> EventResult {
    eprintln!("[FLUFFY] on_source_modify_damage called for move: {}", move_id);
    if let Some(move_data) = battle.dex.moves().get(move_id) {
        let mut mod_value = 1.0;
        if move_data.move_type == "Fire" {
            mod_value *= 2.0;
            eprintln!("[FLUFFY] Move is Fire type, mod_value={}", mod_value);
        }
        if move_data.flags.contains_key("contact") {
            mod_value /= 2.0;
            eprintln!("[FLUFFY] Move has contact flag, mod_value={}", mod_value);
        }
        // JavaScript chainModify returns undefined, so we should return Continue
        // The modifier is stored in event.modifier, not returned to the caller
        battle.chain_modify(mod_value);
        eprintln!("[FLUFFY] chain_modify({}) called, event.modifier updated", mod_value);
        return EventResult::Continue;
    }
    EventResult::Continue
}

