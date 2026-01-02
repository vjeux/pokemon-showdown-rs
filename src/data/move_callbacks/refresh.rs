//! Refresh Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onHit(pokemon) {
///     if (['', 'slp', 'frz'].includes(pokemon.status)) return false;
///     pokemon.cureStatus();
/// }
pub fn on_hit(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
) -> EventResult {
    use crate::dex_data::ID;

    let pokemon = pokemon_pos;

    // if (['', 'slp', 'frz'].includes(pokemon.status)) return false;
    let (status, pokemon_ident, pokemon_name) = {
        let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        (pokemon_pokemon.status.clone(), pokemon_pokemon.get_slot(), pokemon_pokemon.name.clone())
    };

    if status == ID::from("") || status == ID::from("slp") || status == ID::from("frz") {
        return EventResult::Boolean(false);
    }

    // pokemon.cureStatus();
    let pokemon_mut = match battle.pokemon_at_mut(pokemon.0, pokemon.1) {
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

    EventResult::Continue
}
