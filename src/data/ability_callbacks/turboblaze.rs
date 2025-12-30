//! Turboblaze Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onStart(pokemon) {
///     this.add('-ability', pokemon, 'Turboblaze');
/// }
pub fn on_start(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    let pokemon_ident = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.get_slot()
    };

    battle.add(
        "-ability",
        &[
            pokemon_ident.as_str().into(),
            "Turboblaze".into(),
        ],
    );
    EventResult::Continue
}

/// onModifyMove(move) {
///     move.ignoreAbility = true;
/// }
pub fn on_modify_move(_battle: &mut Battle, _move_id: &str) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

