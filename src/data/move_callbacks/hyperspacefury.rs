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
pub fn on_try(battle: &mut Battle, source_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    let source = source_pos;

    // if (source.species.name === 'Hoopa-Unbound') {
    //     return;
    // }
    let species_name = {
        let source_pokemon = match battle.pokemon_at(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        source_pokemon.species_id.name.clone()
    };

    if species_name == "Hoopa-Unbound" {
        return EventResult::Continue;
    }

    // this.hint("Only a Pokemon whose form is Hoopa Unbound can use this move.");
    battle.hint("Only a Pokemon whose form is Hoopa Unbound can use this move.");

    // if (source.species.name === 'Hoopa') {
    if species_name == "Hoopa" {
        // this.attrLastMove('[still]');
        battle.attr_last_move("[still]");

        // this.add('-fail', source, 'move: Hyperspace Fury', '[forme]');
        let source_arg = crate::battle::Arg::Pos(source.0, source.1);
        battle.add("-fail", &[source_arg, "move: Hyperspace Fury".into(), "[forme]".into()]);

        // return null;
        return EventResult::Stop;
    }

    // this.attrLastMove('[still]');
    battle.attr_last_move("[still]");

    // this.add('-fail', source, 'move: Hyperspace Fury');
    let source_arg = crate::battle::Arg::Pos(source.0, source.1);
    battle.add("-fail", &[source_arg, "move: Hyperspace Fury".into()]);

    // return null;
    EventResult::Stop
}

