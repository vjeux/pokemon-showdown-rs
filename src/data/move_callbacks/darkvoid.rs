//! Dark Void Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onTry(source, target, move) {
///     if (source.species.name === 'Darkrai' || move.hasBounced) {
///         return;
///     }
///     this.add('-fail', source, 'move: Dark Void');
///     this.hint("Only a Pokemon whose form is Darkrai can use this move.");
///     return null;
/// }
pub fn on_try(
    battle: &mut Battle,
    source_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
) -> EventResult {
    // if (source.species.name === 'Darkrai' || move.hasBounced) {
    //     return;
    // }
    let source = source_pos;

    let is_darkrai = {
        let source_pokemon = match battle.pokemon_at(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        // species_id is lowercase, so compare to "darkrai"
        source_pokemon.species_id.as_str() == "darkrai"
    };

    let has_bounced = battle
        .active_move
        .as_ref()
        .map(|m| m.has_bounced)
        .unwrap_or(false);

    if is_darkrai || has_bounced {
        // return;
        return EventResult::Continue;
    }

    // this.add('-fail', source, 'move: Dark Void');
    let source_ident = {
        let source_pokemon = match battle.pokemon_at(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        source_pokemon.get_slot()
    };

    battle.add(
        "-fail",
        &[source_ident.as_str().into(), "move: Dark Void".into()],
    );

    // this.hint("Only a Pokemon whose form is Darkrai can use this move.");
    battle.hint(
        "Only a Pokemon whose form is Darkrai can use this move.",
        true,
        None,
    );

    // return null;
    EventResult::Null
}
