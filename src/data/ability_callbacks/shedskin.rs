//! Shed Skin Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onResidual(pokemon) {
///     if (pokemon.hp && pokemon.status && this.randomChance(33, 100)) {
///         this.debug('shed skin');
///         this.add('-activate', pokemon, 'ability: Shed Skin');
///         pokemon.cureStatus();
///     }
/// }
pub fn on_residual(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    let (has_hp, has_status, pokemon_ident, pokemon_name) = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        (pokemon.hp > 0, !pokemon.status.is_empty(), pokemon.get_slot(), pokemon.name.clone())
    };

    if has_hp && has_status && battle.random_chance(33, 100) {
        battle.debug("shed skin");
        battle.add(
            "-activate",
            &[
                pokemon_ident.as_str().into(),
                "ability: Shed Skin".into(),
            ],
        );

        // pokemon.cureStatus()
        let pokemon_mut = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        if let Some((status, removed_nightmare)) = pokemon_mut.cure_status() {
            let full_name = format!("{}: {}", pokemon_ident, pokemon_name);
            battle.add("-curestatus", &[full_name.as_str().into(), status.as_str().into(), "[msg]".into()]);
            if removed_nightmare {
                battle.add("-end", &[full_name.as_str().into(), "Nightmare".into(), "[silent]".into()]);
            }
        }
    }

    EventResult::Continue
}

