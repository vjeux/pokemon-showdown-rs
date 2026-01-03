//! Aftermath Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onDamagingHit(damage, target, source, move) {
///     if (!target.hp && this.checkMoveMakesContact(move, source, target, true)) {
///         this.damage(source.baseMaxhp / 4, source, target);
///     }
/// }
pub fn on_damaging_hit(battle: &mut Battle, _damage: i32, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, move_id: &str) -> EventResult {
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

        if target_hp == 0 && battle.check_move_makes_contact(&crate::ID::from(move_id), source, target, true) {
            // Damage attacker by 1/4 of their max HP
            let damage_amount = {
                let source_pokemon = match battle.pokemon_at(source.0, source.1) {
                    Some(p) => p,
                    None => return EventResult::Continue,
                };
                source_pokemon.base_maxhp / 4
            };
            battle.damage(damage_amount, Some(source), Some(target), None, false);
        }
    }
    EventResult::Continue
}

