//! Bad Dreams Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onResidual(pokemon) {
///     if (!pokemon.hp) return;
///     for (const target of pokemon.foes()) {
///         if (target.status === 'slp' || target.hasAbility('comatose')) {
///             this.damage(target.baseMaxhp / 8, target, pokemon);
///         }
///     }
/// }
pub fn on_residual(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // if (!pokemon.hp) return;
    let pokemon_hp = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.hp
    };

    if pokemon_hp == 0 {
        return EventResult::Continue;
    }

    // for (const target of pokemon.foes())
    let foes = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.foes(battle, false)
    };

    for target_pos in foes {
        // if (target.status === 'slp' || target.hasAbility('comatose'))
        let (is_asleep, has_comatose, base_maxhp) = {
            let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
                Some(p) => p,
                None => continue,
            };
            let is_asleep = target.status.as_str() == "slp";
            let has_comatose = target.has_ability(battle, &["comatose"]);
            (is_asleep, has_comatose, target.base_maxhp)
        };

        if is_asleep || has_comatose {
            // this.damage(target.baseMaxhp / 8, target, pokemon);
            battle.damage(
                base_maxhp / 8,
                Some(target_pos),
                Some(pokemon_pos),
                None,
                false,
            );
        }
    }

    EventResult::Continue
}

