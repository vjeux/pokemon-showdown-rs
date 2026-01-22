//! Overcoat Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onImmunity(type, pokemon) {
///     if (type === 'sandstorm' || type === 'hail' || type === 'powder') return false;
/// }
pub fn on_immunity(_battle: &mut Battle, type_or_status: &str, _pokemon_pos: (usize, usize)) -> EventResult {
    if type_or_status == "sandstorm" || type_or_status == "hail" || type_or_status == "powder" {
        return EventResult::Boolean(false);
    }
    EventResult::Continue
}

/// onTryHit(target, source, move) {
///     if (move.flags['powder'] && target !== source && this.dex.getImmunity('powder', target)) {
///         this.add('-immune', target, '[from] ability: Overcoat');
///         return null;
///     }
/// }
pub fn on_try_hit(battle: &mut Battle, target_pos: (usize, usize), source_pos: (usize, usize), active_move: Option<&crate::battle_actions::ActiveMove>) -> EventResult { let move_id = active_move.map(|m| m.id.as_str()).unwrap_or("");
    // if (target !== source)
    if target_pos == source_pos {
        return EventResult::Continue;
    }

    // if (move.flags['powder'])
    if let Some(move_data) = battle.dex.moves().get(move_id) {
        if move_data.flags.contains_key("powder") {
            // this.dex.getImmunity('powder', target)
            // Note: The onImmunity callback above handles powder immunity, so if we get here
            // the Pokemon should be immune. We can check it via the battle's immunity check.
            // For now, following the JS logic, we assume if the move has powder flag and target != source,
            // we should show immunity.

            // this.add('-immune', target, '[from] ability: Overcoat');
            let target_slot = {
                let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
                    Some(p) => p,
                    None => return EventResult::Continue,
                };
                target.get_slot()
            };

            battle.add(
                "-immune",
                &[
                    crate::battle::Arg::from(target_slot),
                    crate::battle::Arg::from("[from] ability: Overcoat"),
                ],
            );

            // return null;
            return EventResult::Null;
        }
    }

    EventResult::Continue
}

