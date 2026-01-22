//! Rough Skin Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::{Battle, hp_fraction};
use crate::event::EventResult;

/// onDamagingHit(damage, target, source, move) {
///     if (this.checkMoveMakesContact(move, source, target, true)) {
///         this.damage(source.baseMaxhp / 8, source, target);
///     }
/// }
pub fn on_damaging_hit(battle: &mut Battle, _damage: i32, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, active_move: Option<&crate::battle_actions::ActiveMove>) -> EventResult {
    // Damage attacker by 1/8 max HP on contact
    if let (Some(target), Some(source)) = (target_pos, source_pos) {
        // IMPORTANT: Use the ActiveMove directly to get the correct flags (including inherited flags for G-Max moves)
        if battle.check_move_makes_contact_with_active_move(active_move, source, target, true) {
            // Damage attacker by 1/8 of their max HP
            let damage_amount = {
                let source_pokemon = match battle.pokemon_at(source.0, source.1) {
                    Some(p) => p,
                    None => return EventResult::Continue,
                };
                hp_fraction(source_pokemon.base_maxhp, 8)
            };
            battle.damage(damage_amount, Some(source), Some(target), None, false);
        }
    }
    EventResult::Continue
}

