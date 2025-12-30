//! Good as Gold Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onTryHit(target, source, move) {
///     if (move.category === 'Status' && target !== source) {
///         this.add('-immune', target, '[from] ability: Good as Gold');
///         return null;
///     }
/// }
pub fn on_try_hit(battle: &mut Battle, target_pos: (usize, usize), source_pos: (usize, usize), move_id: &str) -> EventResult {
    if target_pos != source_pos {
        if let Some(move_data) = battle.dex.moves().get(move_id) {
            if move_data.category == "Status" {
                let target_ident = {
                    let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
                        Some(p) => p,
                        None => return EventResult::Null,
                    };
                    target.get_slot()
                };

                battle.add(
                    "-immune",
                    &[
                        target_ident.as_str().into(),
                        "[from] ability: Good as Gold".into(),
                    ],
                );
                return EventResult::Null;
            }
        }
    }
    EventResult::Continue
}

