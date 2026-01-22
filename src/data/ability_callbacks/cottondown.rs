//! Cotton Down Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::battle::Effect;
use crate::event::EventResult;

/// onDamagingHit(damage, target, source, move) {
///     let activated = false;
///     for (const pokemon of this.getAllActive()) {
///         if (pokemon === target || pokemon.fainted) continue;
///         if (!activated) {
///             this.add('-ability', target, 'Cotton Down');
///             activated = true;
///         }
///         this.boost({ spe: -1 }, pokemon, target, null, true);
///     }
/// }
pub fn on_damaging_hit(battle: &mut Battle, _damage: i32, target_pos: Option<(usize, usize)>, _source_pos: Option<(usize, usize)>, _active_move: Option<&crate::battle_actions::ActiveMove>) -> EventResult {
    // Lower Speed of all other active Pokemon when hit
    if let Some(target) = target_pos {
        // Get all active Pokemon (not including fainted)
        let all_active = battle.get_all_active(false);

        // Track if ability was activated (for message display)
        let mut activated = false;

        // Loop through all active Pokemon
        for pokemon_pos in all_active {
            // Skip the target itself and check if already fainted
            if pokemon_pos == target {
                continue;
            }

            let is_fainted = {
                let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                    Some(p) => p,
                    None => continue,
                };
                pokemon.fainted
            };

            if is_fainted {
                continue;
            }

            // Activate ability message on first boost (not done yet - would need battle.add() method)
            if !activated {
                activated = true;
            }

            // Lower Speed by 1 stage
            battle.boost(&[("spe", -1)], pokemon_pos, Some(target), None, true, false);
        }
    }
    EventResult::Continue
}

