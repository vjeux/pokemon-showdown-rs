//! Wonder Guard Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onTryHit(target, source, move) {
///     if (target === source || move.category === 'Status' || move.id === 'struggle') return;
///     if (move.id === 'skydrop' && !source.volatiles['skydrop']) return;
///     this.debug('Wonder Guard immunity: ' + move.id);
///     if (target.runEffectiveness(move) <= 0 || !target.runImmunity(move)) {
///         if (move.smartTarget) {
///             move.smartTarget = false;
///         } else {
///             this.add('-immune', target, '[from] ability: Wonder Guard');
///         }
///         return null;
///     }
/// }
pub fn on_try_hit(battle: &mut Battle, target_pos: (usize, usize), source_pos: (usize, usize), move_id: &str) -> EventResult {
    use crate::battle::Arg;

    // if (target === source || move.category === 'Status' || move.id === 'struggle') return;
    if target_pos == source_pos {
        return EventResult::Continue;
    }

    let (is_status, is_struggle, is_skydrop) = {
        let move_data = match battle.dex.moves().get(move_id) {
            Some(m) => m,
            None => return EventResult::Continue,
        };
        (move_data.category == "Status", move_id == "struggle", move_id == "skydrop")
    };

    if is_status || is_struggle {
        return EventResult::Continue;
    }

    // if (move.id === 'skydrop' && !source.volatiles['skydrop']) return;
    if is_skydrop {
        let has_skydrop_volatile = {
            let source = match battle.pokemon_at(source_pos.0, source_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            use crate::dex_data::ID;
            let skydrop_id = ID::from("skydrop");
            source.volatiles.contains_key(&skydrop_id)
        };
        if !has_skydrop_volatile {
            return EventResult::Continue;
        }
    }

    // this.debug('Wonder Guard immunity: ' + move.id);
    // Note: debug logging not implemented yet

    // if (target.runEffectiveness(move) <= 0 || !target.runImmunity(move))
    // Note: We need to check type effectiveness and immunity
    // For now, we'll use a simplified version checking if the move would be not very effective or immune
    let not_super_effective = {
        // Check type effectiveness - rely on the move hit data if available
        if let Some(hit_data) = battle.get_move_hit_data(target_pos) {
            hit_data.type_mod <= 0
        } else {
            // Can't determine effectiveness without hit data
            false
        }
    };

    if not_super_effective {
        // if (move.smartTarget) { move.smartTarget = false; }
        let should_show_immune = if let Some(active_move) = &mut battle.active_move {
            if active_move.smart_target.unwrap_or(false) {
                active_move.smart_target = Some(false);
                false
            } else {
                true
            }
        } else {
            true
        };

        if should_show_immune {
            let target_id = {
                let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
                    Some(p) => p,
                    None => return EventResult::Null,
                };
                let side_id = format!("p{}", target.side_index + 1);
                if target.is_active {
                    let pos_letter = (b'a' + target.position as u8) as char;
                    format!("{}{}: {}", side_id, pos_letter, target.name)
                } else {
                    format!("{}: {}", side_id, target.name)
                }
            };

            // this.add('-immune', target, '[from] ability: Wonder Guard');
            battle.add("-immune", &[
                Arg::String(target_id),
                Arg::Str("[from] ability: Wonder Guard"),
            ]);
        }

        // return null;
        return EventResult::Null;
    }

    EventResult::Continue
}

