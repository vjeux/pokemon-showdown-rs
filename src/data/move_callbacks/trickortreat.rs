//! Trick-or-Treat Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// ```ignore
/// onHit(target) {
///     if (target.hasType('Ghost')) return false;
///     if (!target.addType('Ghost')) return false;
///     this.add('-start', target, 'typeadd', 'Ghost', '[from] move: Trick-or-Treat');
///
///     if (target.side.active.length === 2 && target.position === 1) {
///         // Curse Glitch
///         const action = this.queue.willMove(target);
///         if (action && action.move.id === 'curse') {
///             action.targetLoc = -1;
///         }
///     }
/// }
/// ```
pub fn on_hit(
    battle: &mut Battle,
    _pokemon_pos: (usize, usize),
    target_pos: Option<(usize, usize)>,
) -> EventResult {
    let target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // if (target.hasType('Ghost')) return false;
    let already_has_ghost = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target_pokemon.has_type(battle, "Ghost")
    };

    if already_has_ghost {
        return EventResult::NotFail;
    }

    // if (!target.addType('Ghost')) return false;
    // Note: In Rust, add_type doesn't return a bool, it just adds the type
    // We already checked that the target doesn't have Ghost type above
    {
        let target_pokemon = match battle.pokemon_at_mut(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target_pokemon.add_type("Ghost".to_string());
    }

    // this.add('-start', target, 'typeadd', 'Ghost', '[from] move: Trick-or-Treat');
    let target_arg = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target_pokemon.get_slot()
    };

    battle.add(
        "-start",
        &[
            target_arg.into(),
            "typeadd".into(),
            "Ghost".into(),
            "[from] move: Trick-or-Treat".into(),
        ],
    );

    // if (target.side.active.length === 2 && target.position === 1) {
    //     // Curse Glitch
    //     const action = this.queue.willMove(target);
    //     if (action && action.move.id === 'curse') {
    //         action.targetLoc = -1;
    //     }
    // }
    // TODO: Implement Curse Glitch - requires modifying queue action targetLoc
    // This is a complex edge case that requires queue manipulation infrastructure

    EventResult::Continue
}
