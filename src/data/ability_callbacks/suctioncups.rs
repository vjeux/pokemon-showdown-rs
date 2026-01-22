//! Suction Cups Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::battle::Effect;
use crate::event::EventResult;

/// onDragOut(pokemon) {
///     this.add('-activate', pokemon, 'ability: Suction Cups');
///     return null;
/// }
pub fn on_drag_out(battle: &mut Battle, pokemon_pos: (usize, usize), _source_pos: Option<(usize, usize)>, _active_move: Option<&crate::battle_actions::ActiveMove>) -> EventResult {
    let pokemon_ident = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Null,
        };
        pokemon.get_slot()
    };

    battle.add(
        "-activate",
        &[
            pokemon_ident.as_str().into(),
            "ability: Suction Cups".into(),
        ],
    );
    EventResult::Null
}

