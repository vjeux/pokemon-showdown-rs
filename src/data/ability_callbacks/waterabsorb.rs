//! Water Absorb Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onTryHit(target, source, move) {
///     if (target !== source && move.type === 'Water') {
///         if (!this.heal(target.baseMaxhp / 4)) {
///             this.add('-immune', target, '[from] ability: Water Absorb');
///         }
///         return null;
///     }
/// }
pub fn on_try_hit(battle: &mut Battle, target_pos: (usize, usize), source_pos: (usize, usize), active_move: Option<&crate::battle_actions::ActiveMove>) -> EventResult { let move_id = active_move.map(|m| m.id.as_str()).unwrap_or("");
    eprintln!("[WATERABSORB_TRYHIT] target={:?}, source={:?}, move={}", target_pos, source_pos, move_id);

    // Immune to Water-type moves and heal 1/4 max HP
    if target_pos != source_pos {
        eprintln!("[WATERABSORB_TRYHIT] target != source, checking move type");
        // Check if the move is Water-type
        let is_water = {
            let move_data = match battle.dex.moves().get(move_id) {
                Some(m) => m,
                None => {
                    eprintln!("[WATERABSORB_TRYHIT] Move not found in dex, returning Continue");
                    return EventResult::Continue;
                }
            };
            move_data.move_type == "Water"
        };
        eprintln!("[WATERABSORB_TRYHIT] is_water={}", is_water);

        if is_water {
            eprintln!("[WATERABSORB_TRYHIT] Water move detected, healing and returning Null");
            // Heal 1/4 max HP
            let heal_amount = {
                let target_pokemon = match battle.pokemon_at(target_pos.0, target_pos.1) {
                    Some(p) => p,
                    None => return EventResult::Continue,
                };
                target_pokemon.base_maxhp / 4
            };
            battle.heal(heal_amount, Some(target_pos), None, None);
            // Return Null to prevent the move from hitting
            return EventResult::Null;
        }
    } else {
        eprintln!("[WATERABSORB_TRYHIT] target == source, returning Continue");
    }
    eprintln!("[WATERABSORB_TRYHIT] Returning Continue");
    EventResult::Continue
}

