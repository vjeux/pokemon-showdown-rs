//! Freezy Frost Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onHit() {
///     this.add('-clearallboost');
///     for (const pokemon of this.getAllActive()) {
///         pokemon.clearBoosts();
///     }
/// }
pub fn on_hit(battle: &mut Battle, _pokemon_pos: (usize, usize), _target_pos: Option<(usize, usize)>) -> EventResult {
    // this.add('-clearallboost');
    battle.add("-clearallboost", &[]);

    // for (const pokemon of this.getAllActive()) {
    //     pokemon.clearBoosts();
    // }
    let all_active = battle.get_all_active(false);

    for pokemon_pos in all_active {
        let pokemon = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => continue,
        };
        pokemon.clear_boosts();
    }

    EventResult::Continue
}

