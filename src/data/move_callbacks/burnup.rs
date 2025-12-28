//! Burn Up Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::{Battle, Arg};
use crate::event::EventResult;
use crate::dex_data::ID;

/// onTryMove(pokemon, target, move) {
///     if (pokemon.hasType('Fire')) return;
///     this.add('-fail', pokemon, 'move: Burn Up');
///     this.attrLastMove('[still]');
///     return null;
/// }
pub fn on_try_move(battle: &mut Battle, source_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    // if (pokemon.hasType('Fire')) return;
    let has_fire = {
        let pokemon = match battle.pokemon_at(source_pos.0, source_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.has_type(&ID::from("Fire"))
    };

    if has_fire {
        // return;
        return EventResult::Continue;
    }

    // this.add('-fail', pokemon, 'move: Burn Up');
    let pokemon_arg = {
        let pokemon = match battle.pokemon_at(source_pos.0, source_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        Arg::from(pokemon)
    };

    battle.add("-fail", &[pokemon_arg, "move: Burn Up".into()]);

    // this.attrLastMove('[still]');
    battle.attr_last_move("[still]");

    // return null;
    EventResult::Stop
}
