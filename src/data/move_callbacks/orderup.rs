//! Order Up Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onAfterMoveSecondarySelf(pokemon, target, move) {
///     if (!pokemon.volatiles['commanded']) return;
///     const tatsugiri = pokemon.volatiles['commanded'].source;
///     if (tatsugiri.baseSpecies.baseSpecies !== 'Tatsugiri') return; // Should never happen
///     switch (tatsugiri.baseSpecies.forme) {
///     case 'Droopy':
///         this.boost({ def: 1 }, pokemon, pokemon);
///         break;
///     case 'Stretchy':
///         this.boost({ spe: 1 }, pokemon, pokemon);
///         break;
///     default:
///         this.boost({ atk: 1 }, pokemon, pokemon);
///         break;
///     }
/// }
pub fn on_after_move_secondary_self(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>, move_id: &str) -> EventResult {
    use crate::dex_data::ID;

    let pokemon = pokemon_pos;

    // if (!pokemon.volatiles['commanded']) return;
    let commanded_source = {
        let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon_pokemon.volatiles.get(&ID::from("commanded"))
            .and_then(|v| v.source)
    };

    let tatsugiri_pos = match commanded_source {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // const tatsugiri = pokemon.volatiles['commanded'].source;
    // if (tatsugiri.baseSpecies.baseSpecies !== 'Tatsugiri') return;
    let forme = {
        let tatsugiri = match battle.pokemon_at(tatsugiri_pos.0, tatsugiri_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        if tatsugiri.base_species.base_species != "Tatsugiri" {
            return EventResult::Continue;
        }

        // switch (tatsugiri.baseSpecies.forme)
        tatsugiri.base_species.forme.clone()
    };

    // switch (tatsugiri.baseSpecies.forme) {
    use std::collections::HashMap;
    let mut boosts = HashMap::new();

    match forme.as_deref() {
        // case 'Droopy':
        Some("Droopy") => {
            // this.boost({ def: 1 }, pokemon, pokemon);
            boosts.insert("def".to_string(), 1);
        }
        // case 'Stretchy':
        Some("Stretchy") => {
            // this.boost({ spe: 1 }, pokemon, pokemon);
            boosts.insert("spe".to_string(), 1);
        }
        // default:
        _ => {
            // this.boost({ atk: 1 }, pokemon, pokemon);
            boosts.insert("atk".to_string(), 1);
        }
    }

    battle.boost(boosts, pokemon, Some(pokemon), None);

    EventResult::Continue
}

