//! Take Heart Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

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
    let boost_success = battle.boost(&[("spa", 1), ("spd", 1)], pokemon, None, None);

    // return pokemon.cureStatus() || success;
    let cure_success = battle.cure_status(pokemon);

    if cure_success || boost_success {
        EventResult::Continue
    } else {
        EventResult::NotFail
    }
}
