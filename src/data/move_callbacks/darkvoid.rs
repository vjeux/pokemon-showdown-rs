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
pub fn on_try(battle: &mut Battle, source_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    // if (source.species.name === 'Darkrai' || move.hasBounced) {
    //     return;
    // }
    let source = source_pos;

    let is_darkrai = {
        let source_pokemon = match battle.pokemon_at(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        source_pokemon.species_id.as_str() == "Darkrai"
    };

    // TODO: Check move.hasBounced - this requires access to the current move state
    // For now, we'll only check if the source is Darkrai
    if is_darkrai {
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

    battle.add("-fail", &[source_ident.as_str().into(), "move: Dark Void".into()]);

    // this.hint("Only a Pokemon whose form is Darkrai can use this move.");
    battle.hint("Only a Pokemon whose form is Darkrai can use this move.", true, None);

    // return null;
    EventResult::Stop
}

