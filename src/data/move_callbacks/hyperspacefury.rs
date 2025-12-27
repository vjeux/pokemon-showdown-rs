//! Hyperspace Fury Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::{Arg, Battle};
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
    let source = match battle.pokemon_at(source_pos.0, source_pos.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };

    // Get species name from Dex
    let species_name = battle.dex.species.get(&source.species_id.to_string())
        .map(|s| s.name.as_str())
        .unwrap_or("");

    // Only Hoopa-Unbound can use this move
    if species_name == "Hoopa-Unbound" {
        return EventResult::Continue;
    }

    battle.hint("Only a Pokemon whose form is Hoopa Unbound can use this move.");

    let source = match battle.pokemon_at(source_pos.0, source_pos.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };

    if species_name == "Hoopa" {
        battle.attr_last_move("[still]");
        battle.add("-fail", &[
            Arg::Pokemon(source),
            Arg::Str("move: Hyperspace Fury"),
            Arg::Str("[forme]"),
        ]);
        return EventResult::Null;
    }

    battle.attr_last_move("[still]");
    battle.add("-fail", &[Arg::Pokemon(source), Arg::Str("move: Hyperspace Fury")]);
    EventResult::Null
}

