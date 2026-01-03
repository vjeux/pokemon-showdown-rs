//! Innards Out Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onDamagingHit(damage, target, source, move) {
///     if (!target.hp) {
///         this.damage(target.getUndynamaxedHP(damage), source, target);
///     }
/// }
pub fn on_damaging_hit(battle: &mut Battle, damage: i32, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, _move_id: &str) -> EventResult {
    // If target fainted, damage the attacker by the damage taken
    if let (Some(target), Some(source)) = (target_pos, source_pos) {
        // Check if target has 0 HP (fainted)
        let (target_hp, undynamaxed_damage) = {
            let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            // target.getUndynamaxedHP(damage)
            (target_pokemon.hp, target_pokemon.get_undynamaxed_hp(Some(damage)))
        };

        // if (!target.hp)
        if target_hp == 0 {
            // this.damage(target.getUndynamaxedHP(damage), source, target);
            battle.damage(undynamaxed_damage, Some(source), Some(target), None, false);
        }
    }
    EventResult::Continue
}

