//! Telepathy Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onTryHit(target, source, move) {
///     if (target !== source && target.isAlly(source) && move.category !== 'Status') {
///         this.add('-activate', target, 'ability: Telepathy');
///         return null;
///     }
/// }
pub fn on_try_hit(battle: &mut Battle, target_pos: (usize, usize), source_pos: (usize, usize), active_move: Option<&crate::battle_actions::ActiveMove>) -> EventResult { let move_id = active_move.map(|m| m.id.as_str()).unwrap_or("");
    use crate::battle::Arg;

    // if (target !== source && target.isAlly(source) && move.category !== 'Status')
    if target_pos != source_pos && battle.is_ally(target_pos, source_pos) {
        if let Some(move_data) = battle.dex.moves().get(move_id) {
            if move_data.category != "Status" {
                let target_id = {
                    let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
                        Some(p) => p,
                        None => return EventResult::Continue,
                    };
                    let side_id = format!("p{}", target.side_index + 1);
                    if target.is_active {
                        let pos_letter = (b'a' + target.position as u8) as char;
                        format!("{}{}: {}", side_id, pos_letter, target.name)
                    } else {
                        format!("{}: {}", side_id, target.name)
                    }
                };

                battle.add("-activate", &[
                    Arg::String(target_id),
                    Arg::Str("ability: Telepathy"),
                ]);

                // return null;
                return EventResult::Null;
            }
        }
    }

    EventResult::Continue
}

