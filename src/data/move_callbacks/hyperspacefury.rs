//! Hyperspace Fury Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onTry(source) {
///     if (source.species.name === 'Hoopa-Unbound') {
///         return;
///     }
///     this.hint("Only a Pokemon whose form is Hoopa Unbound can use this move.");
///     if (source.species.name === 'Hoopa') {
///         this.attrLastMove('[still]');
///         this.add('-fail', source, 'move: Hyperspace Fury', '[forme]');
///         return null;
///     }
///     this.attrLastMove('[still]');
///     this.add('-fail', source, 'move: Hyperspace Fury');
///     return null;
/// }
pub fn on_try(
    battle: &mut Battle,
    source_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
) -> EventResult {
    use crate::dex_data::ID;

    let source = source_pos;

    // if (source.species.name === 'Hoopa-Unbound') {
    //     return;
    // }
    // Note: JavaScript checks source.species.name (formatted name like "Hoopa-Unbound")
    // but we check species_id (lowercase ID like "hoopaunbound")
    let (species_id, species_is_hoopa) = {
        let source_pokemon = match battle.pokemon_at(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        (
            source_pokemon.species_id.clone(),
            source_pokemon.species_id == ID::from("hoopa"),
        )
    };

    if species_id == ID::from("hoopaunbound") {
        return EventResult::Continue;
    }

    // this.hint("Only a Pokemon whose form is Hoopa Unbound can use this move.");
    battle.hint(
        "Only a Pokemon whose form is Hoopa Unbound can use this move.",
        true,
        None,
    );

    // if (source.species.name === 'Hoopa') {
    if species_is_hoopa {
        // this.attrLastMove('[still]');
        battle.attr_last_move(&["[still]"]);

        // this.add('-fail', source, 'move: Hyperspace Fury', '[forme]');
        let source_ident = {
            let pokemon = match battle.pokemon_at(source.0, source.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon.get_slot()
        };
        battle.add(
            "-fail",
            &[
                source_ident.as_str().into(),
                "move: Hyperspace Fury".into(),
                "[forme]".into(),
            ],
        );

        // return null; - prevents the move from executing
        return EventResult::Null;
    }

    // this.attrLastMove('[still]');
    battle.attr_last_move(&["[still]"]);

    // this.add('-fail', source, 'move: Hyperspace Fury');
    let source_ident = {
        let pokemon = match battle.pokemon_at(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.get_slot()
    };
    battle.add(
        "-fail",
        &[source_ident.as_str().into(), "move: Hyperspace Fury".into()],
    );

    // return null; - prevents the move from executing
    EventResult::Null
}
