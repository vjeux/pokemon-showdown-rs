//! Burn Up Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onTryMove(pokemon, target, move) {
///     if (pokemon.hasType('Fire')) return;
///     this.add('-fail', pokemon, 'move: Burn Up');
///     this.attrLastMove('[still]');
///     return null;
/// }
pub fn on_try_move(
    battle: &mut Battle,
    source_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
) -> EventResult {
    // if (pokemon.hasType('Fire')) return;
    let has_fire = {
        let pokemon = match battle.pokemon_at(source_pos.0, source_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.has_type(battle, "Fire")
    };

    if has_fire {
        // return;
        return EventResult::Continue;
    }

    // this.add('-fail', pokemon, 'move: Burn Up');
    let pokemon_ident = {
        let pokemon = match battle.pokemon_at(source_pos.0, source_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.get_slot()
    };

    battle.add(
        "-fail",
        &[pokemon_ident.as_str().into(), "move: Burn Up".into()],
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
    ///                 pokemon.setType(pokemon.getTypes(true).map((type) => type === "Fire" ? "???" : type));
    ///                 this.add("-start", pokemon, "typechange", pokemon.getTypes().join("/"), "[from] move: Burn Up");
    ///               }
    ///     },
    /// }
    /// ```
    pub fn on_hit(
        battle: &mut Battle,
        _target_pos: (usize, usize),
        source_pos: Option<(usize, usize)>,
    ) -> EventResult {
        // pokemon.setType(pokemon.getTypes(true).map((type) => type === "Fire" ? "???" : type));
        let source = match source_pos {
            Some(pos) => pos,
            None => return EventResult::Continue,
        };

        // Get types with exclude_added=true, map "Fire" to "???"
        let new_types = {
            let pokemon = match battle.pokemon_at(source.0, source.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon.get_types(battle, true)
                .into_iter()
                .map(|t| if t == "Fire" { "???".to_string() } else { t })
                .collect::<Vec<String>>()
        };

        // Set the new types
        crate::pokemon::Pokemon::set_type(battle, source, new_types, false);

        // this.add("-start", pokemon, "typechange", pokemon.getTypes().join("/"), "[from] move: Burn Up");
        let (pokemon_ident, types_str) = {
            let pokemon = match battle.pokemon_at(source.0, source.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            let types = pokemon.get_types(battle, false);
            (pokemon.get_slot(), types.join("/"))
        };

        battle.add(
            "-start",
            &[
                pokemon_ident.as_str().into(),
                "typechange".into(),
                types_str.into(),
                "[from] move: Burn Up".into(),
            ],
        );

        EventResult::Continue
    }
}
