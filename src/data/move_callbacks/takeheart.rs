//! Take Heart Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;
use crate::Pokemon;

/// onHit(pokemon) {
///     const success = !!this.boost({ spa: 1, spd: 1 });
///     return pokemon.cureStatus() || success;
/// }
pub fn on_hit(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
) -> EventResult {
    let pokemon = pokemon_pos;

    // const success = !!this.boost({ spa: 1, spd: 1 });
    let boost_success = battle.boost(&[("spa", 1), ("spd", 1)], pokemon, None, None, false, false);

    // return pokemon.cureStatus() || success;
    let (pokemon_ident, pokemon_name) = {
        let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        (pokemon_pokemon.get_slot(), pokemon_pokemon.name.clone())
    };

    let pokemon_mut = match battle.pokemon_at_mut(pokemon.0, pokemon.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };

    let cure_status_result = Pokemon::cure_status(battle, pokemon_pos, false);
    let cure_success = cure_status_result.is_some();

    if let Some((status, removed_nightmare, _silent)) = cure_status_result {
        let full_name = format!("{}: {}", pokemon_ident, pokemon_name);
        battle.add("-curestatus", &[full_name.as_str().into(), status.as_str().into(), "[msg]".into()]);
        if removed_nightmare {
            battle.add("-end", &[full_name.as_str().into(), "Nightmare".into(), "[silent]".into()]);
        }
    }

    if cure_success || boost_success {
        EventResult::Continue
    } else {
        EventResult::NotFail
    }
}
