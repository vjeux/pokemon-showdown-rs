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
    let (has_hp, has_status, pokemon_ident) = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        (pokemon.hp > 0, !pokemon.status.is_empty(), pokemon.get_slot())
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
        battle.cure_status(pokemon_pos);
    }

    EventResult::Continue
}

