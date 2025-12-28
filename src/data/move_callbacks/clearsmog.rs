//! Clear Smog Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::{Battle, Arg};
use crate::event::EventResult;

/// onHit(target) {
///     target.clearBoosts();
///     this.add('-clearboost', target);
/// }
pub fn on_hit(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    // Get the target
    let target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // target.clearBoosts();
    {
        let target_pokemon = match battle.pokemon_at_mut(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target_pokemon.clear_boosts();
    }

    // this.add('-clearboost', target);
    let target_ident = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target_pokemon.get_slot()
    };

    battle.add("-clearboost", &[target_ident.as_str().into()]);

    EventResult::Continue
}
