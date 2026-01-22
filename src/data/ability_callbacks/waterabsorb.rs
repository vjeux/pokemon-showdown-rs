//! Water Absorb Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::{Battle, hp_fraction};
use crate::event::EventResult;

/// onTryHit(target, source, move) {
///     if (target !== source && move.type === 'Water') {
///         if (!this.heal(target.baseMaxhp / 4)) {
///             this.add('-immune', target, '[from] ability: Water Absorb');
///         }
///         return null;
///     }
/// }
pub fn on_try_hit(battle: &mut Battle, target_pos: (usize, usize), source_pos: (usize, usize), active_move: Option<&crate::battle_actions::ActiveMove>) -> EventResult {
    // Immune to Water-type moves and heal 1/4 max HP
    // if (target !== source && move.type === 'Water') {
    if target_pos != source_pos {
        // Check if the move is Water-type
        // JavaScript checks move.type (the active move's type, not the dex type)
        // This is important for moves like Soak or type-changing abilities
        let is_water = active_move.map(|m| m.move_type == "Water").unwrap_or(false);

        if is_water {
            // Heal 1/4 max HP
            // if (!this.heal(target.baseMaxhp / 4)) {
            //     this.add('-immune', target, '[from] ability: Water Absorb');
            // }
            let heal_amount = {
                let target_pokemon = match battle.pokemon_at(target_pos.0, target_pos.1) {
                    Some(p) => p,
                    None => return EventResult::Continue,
                };
                hp_fraction(target_pokemon.base_maxhp, 4)
            };
            battle.heal(heal_amount, Some(target_pos), None, None);
            // Return Null to prevent the move from hitting
            // return null;
            return EventResult::Null;
        }
    }
    EventResult::Continue
}

