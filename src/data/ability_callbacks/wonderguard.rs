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
pub fn on_try_hit(battle: &mut Battle, target_pos: (usize, usize), source_pos: (usize, usize), active_move: Option<&crate::battle_actions::ActiveMove>) -> EventResult { let move_id = active_move.map(|m| m.id.as_str()).unwrap_or("");
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
    use crate::Pokemon;

    // Check type effectiveness: runEffectiveness returns typeMod (0 = neutral, >0 = super effective, <0 = not very effective)
    // Need to get active_move reference for run_effectiveness
    let active_move_ref = match active_move {
        Some(m) => m,
        None => return EventResult::Continue,
    };
    let type_effectiveness = Pokemon::run_effectiveness(battle, target_pos, active_move_ref);

    // Check immunity: runImmunity returns true if NOT immune, false if immune
    let move_type = {
        let move_data = match battle.dex.moves().get(move_id) {
            Some(m) => m.move_type.clone(),
            None => return EventResult::Continue,
        };
        move_data
    };
    let is_immune = !Pokemon::run_immunity(battle, target_pos, &move_type, false);

    // Wonder Guard blocks if not super effective OR if immune
    let should_block = type_effectiveness <= 0 || is_immune;

    if should_block {
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

