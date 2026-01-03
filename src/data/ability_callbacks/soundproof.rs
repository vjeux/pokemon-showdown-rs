//! Soundproof Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onTryHit(target, source, move) {
///     if (target !== source && move.flags['sound']) {
///         this.add('-immune', target, '[from] ability: Soundproof');
///         return null;
///     }
/// }
pub fn on_try_hit(battle: &mut Battle, target_pos: (usize, usize), source_pos: (usize, usize), move_id: &str) -> EventResult {
    if target_pos != source_pos {
        if let Some(move_data) = battle.dex.moves().get(move_id) {
            if move_data.flags.contains_key("sound") {
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
                        "[from] ability: Soundproof".into(),
                    ],
                );
                return EventResult::Null;
            }
        }
    }
    EventResult::Continue
}

/// onAllyTryHitSide(target, source, move) {
///     if (move.flags['sound']) {
///         this.add('-immune', this.effectState.target, '[from] ability: Soundproof');
///     }
/// }
pub fn on_ally_try_hit_side(battle: &mut Battle, holder_pos: Option<(usize, usize)>, _source_pos: Option<(usize, usize)>, move_id: &str) -> EventResult {
    // if (move.flags['sound'])
    if let Some(move_data) = battle.dex.moves().get(move_id) {
        if move_data.flags.contains_key("sound") {
            // this.add('-immune', this.effectState.target, '[from] ability: Soundproof');
            if let Some(pos) = holder_pos {
                let holder_slot = {
                    let holder = match battle.pokemon_at(pos.0, pos.1) {
                        Some(p) => p,
                        None => return EventResult::Continue,
                    };
                    holder.get_slot()
                };

                battle.add(
                    "-immune",
                    &[
                        crate::battle::Arg::from(holder_slot),
                        crate::battle::Arg::from("[from] ability: Soundproof"),
                    ],
                );
            }
        }
    }

    EventResult::Continue
}

