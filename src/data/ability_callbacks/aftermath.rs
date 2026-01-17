//! Aftermath Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::{Battle, hp_fraction};
use crate::event::EventResult;

/// onDamagingHit(damage, target, source, move) {
///     if (!target.hp && this.checkMoveMakesContact(move, source, target, true)) {
///         this.damage(source.baseMaxhp / 4, source, target);
///     }
/// }
pub fn on_damaging_hit(battle: &mut Battle, _damage: i32, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, active_move: Option<&crate::battle_actions::ActiveMove>) -> EventResult {
    use crate::battle::Effect;

    // If target fainted and move makes contact, damage the attacker
    if let (Some(target), Some(source)) = (target_pos, source_pos) {
        // Check if target has 0 HP
        let target_hp = {
            let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            target_pokemon.hp
        };

        // IMPORTANT: Use the ActiveMove directly to get the correct flags (including inherited flags for G-Max moves)
        if target_hp == 0 && battle.check_move_makes_contact_with_active_move(active_move, source, target, true) {
            // Damage attacker by 1/4 of their max HP
            let damage_amount = {
                let source_pokemon = match battle.pokemon_at(source.0, source.1) {
                    Some(p) => p,
                    None => return EventResult::Continue,
                };
                hp_fraction(source_pokemon.base_maxhp, 4)
            };
            // Pass the Aftermath ability effect so Damp's onAnyDamage can detect and block it
            // JavaScript: this.damage() uses the current ability effect (this.effect) implicitly
            battle.damage(damage_amount, Some(source), Some(target), Some(&Effect::ability("aftermath")), false);
        }
    }
    EventResult::Continue
}

