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
    target_pos: (usize, usize),  // JavaScript: onHit(target) - target is first param
    _source_pos: Option<(usize, usize)>,
) -> EventResult {
    let target = target_pos;

    // if (target.hasType('Ghost')) return false;
    let already_has_ghost = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target_pokemon.has_type(battle, "Ghost")
    };

    if already_has_ghost {
        // JavaScript: return false - the hit failed
        return EventResult::Boolean(false);
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
    let (side_active_len, target_position) = {
        let target_side = &battle.sides[target.0];
        (target_side.active.len(), target.1)
    };

    if side_active_len == 2 && target_position == 1 {
        // Curse Glitch
        if let Some(action) = battle.queue.will_move_mut(target.0, target.1) {
            if action.move_id.as_str() == "curse" {
                action.target_loc = -1;
            }
        }
    }

    EventResult::Continue
}
