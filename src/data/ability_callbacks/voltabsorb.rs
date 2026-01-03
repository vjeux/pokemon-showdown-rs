//! Volt Absorb Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onTryHit(target, source, move) {
///     if (target !== source && move.type === 'Electric') {
///         if (!this.heal(target.baseMaxhp / 4)) {
///             this.add('-immune', target, '[from] ability: Volt Absorb');
///         }
///         return null;
///     }
/// }
pub fn on_try_hit(battle: &mut Battle, target_pos: (usize, usize), source_pos: (usize, usize), move_id: &str) -> EventResult {
    // Immune to Electric-type moves and heal 1/4 max HP
    if target_pos != source_pos {
        // Check if the move is Electric-type
        let is_electric = {
            let move_data = match battle.dex.moves().get(move_id) {
                Some(m) => m,
                None => return EventResult::Continue,
            };
            move_data.move_type == "Electric"
        };

        if is_electric {
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
    }
    EventResult::Continue
}

