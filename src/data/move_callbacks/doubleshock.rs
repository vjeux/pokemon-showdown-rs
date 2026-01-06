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
pub fn on_try_move(
    battle: &mut Battle,
    source_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
) -> EventResult {
    // if (pokemon.hasType('Electric')) return;
    let pokemon = source_pos;

    let has_electric = {
        let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon_pokemon.has_type(battle, "electric")
    };

    if has_electric {
        // return;
        return EventResult::Continue;
    }

    // this.add('-fail', pokemon, 'move: Double Shock');
    let pokemon_ident = {
        let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon_pokemon.get_slot()
    };

    battle.add(
        "-fail",
        &[pokemon_ident.as_str().into(), "move: Double Shock".into()],
    );

    // this.attrLastMove('[still]');
    battle.attr_last_move(&["[still]"]);

    // return null;
    EventResult::Stop
}

/// Self-targeting callbacks
/// These callbacks target the move user (source), not the move target
pub mod self_callbacks {
    use super::*;

    /// self.onHit(source)
    ///
    /// ```text
    /// JS Source (data/moves.ts):
    /// self: {
    ///     onHit(source) {
    ///         onHit(pokemon) {
    ///                 pokemon.setType(pokemon.getTypes(true).map((type) => type === "Electric" ? "???" : type));
    ///                 this.add("-start", pokemon, "typechange", pokemon.getTypes().join("/"), "[from] move: Double Shock");
    ///               }
    ///     },
    /// }
    /// ```
    pub fn on_hit(
        _battle: &mut Battle,
        _target_pos: (usize, usize),
        _source_pos: Option<(usize, usize)>,
    ) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }
}
