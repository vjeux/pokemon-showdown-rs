//! Double Shock Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onTryMove(pokemon, target, move) {
///     if (pokemon.hasType('Electric')) return;
///     this.add('-fail', pokemon, 'move: Double Shock');
///     this.attrLastMove('[still]');
///     return null;
/// }
pub fn on_try_move(battle: &mut Battle, source_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    use crate::dex_data::ID;

    // if (pokemon.hasType('Electric')) return;
    let pokemon = source_pos;

    let has_electric = {
        let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon_pokemon.has_type("electric")
    };

    if has_electric {
        // return;
        return EventResult::Continue;
    }

    // this.add('-fail', pokemon, 'move: Double Shock');
    let pokemon_arg = {
        let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        crate::battle::Arg::from(pokemon_pokemon)
    };

    battle.add("-fail", &[pokemon_arg, "move: Double Shock".into()]);

    // this.attrLastMove('[still]');
    battle.attr_last_move(&["[still]"]);

    // return null;
    EventResult::Stop
}

