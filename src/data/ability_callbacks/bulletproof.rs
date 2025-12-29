//! Bulletproof Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onTryHit(pokemon, target, move) {
///     if (move.flags['bullet']) {
///         this.add('-immune', pokemon, '[from] ability: Bulletproof');
///         return null;
///     }
/// }
pub fn on_try_hit(battle: &mut Battle, target_pos: (usize, usize), source_pos: (usize, usize), move_id: &str) -> EventResult {
    if let Some(move_data) = battle.dex.get_move(move_id) {
        if move_data.flags.contains_key("bullet") {
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
                    "[from] ability: Bulletproof".into(),
                ],
            );
            return EventResult::Null;
        }
    }
    EventResult::Continue
}

