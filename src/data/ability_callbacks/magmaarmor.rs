//! Magma Armor Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onUpdate(pokemon) {
///     if (pokemon.status === 'frz') {
///         this.add('-activate', pokemon, 'ability: Magma Armor');
///         pokemon.cureStatus();
///     }
/// }
pub fn on_update(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    let (is_frozen, pokemon_ident) = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        (pokemon.status == "frz".into(), pokemon.get_slot())
    };

    if is_frozen {
        battle.add(
            "-activate",
            &[
                pokemon_ident.as_str().into(),
                "ability: Magma Armor".into(),
            ],
        );
        battle.cure_status(pokemon_pos);
    }

    EventResult::Continue
}

/// onImmunity(type, pokemon) {
///     if (type === 'frz') return false;
/// }
pub fn on_immunity(battle: &mut Battle, type_or_status: &str, pokemon_pos: (usize, usize)) -> EventResult {
    if type_or_status == "frz" {
        return EventResult::Boolean(false);
    }
    EventResult::Continue
}

