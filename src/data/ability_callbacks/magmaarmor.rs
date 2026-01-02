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
    let (is_frozen, pokemon_ident, pokemon_name) = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        (pokemon.status == "frz".into(), pokemon.get_slot(), pokemon.name.clone())
    };

    if is_frozen {
        battle.add(
            "-activate",
            &[
                pokemon_ident.as_str().into(),
                "ability: Magma Armor".into(),
            ],
        );

        // pokemon.cureStatus()
        let pokemon_mut = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        if let Some((status, removed_nightmare, _silent)) = pokemon_mut.cure_status(false) {
            let full_name = format!("{}: {}", pokemon_ident, pokemon_name);
            battle.add("-curestatus", &[full_name.as_str().into(), status.as_str().into(), "[msg]".into()]);
            if removed_nightmare {
                battle.add("-end", &[full_name.as_str().into(), "Nightmare".into(), "[silent]".into()]);
            }
        }
    }

    EventResult::Continue
}

/// onImmunity(type, pokemon) {
///     if (type === 'frz') return false;
/// }
pub fn on_immunity(_battle: &mut Battle, type_or_status: &str, _pokemon_pos: (usize, usize)) -> EventResult {
    if type_or_status == "frz" {
        return EventResult::Boolean(false);
    }
    EventResult::Continue
}

