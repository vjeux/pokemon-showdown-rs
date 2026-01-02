//! Thousand Arrows Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onEffectiveness(typeMod, target, type, move) {
///     if (move.type !== 'Ground') return;
///     if (!target) return; // avoid crashing when called from a chat plugin
///     // ignore effectiveness if the target is Flying type and immune to Ground
///     if (!target.runImmunity('Ground')) {
///         if (target.hasType('Flying')) return 0;
///     }
/// }
pub fn on_effectiveness(
    battle: &mut Battle,
    _type_mod: i32,
    _target_type: &str,
    target_pos: Option<(usize, usize)>,
) -> EventResult {
    // if (move.type !== 'Ground') return;
    let move_type = match &battle.active_move {
        Some(m) => m.move_type.clone(),
        None => return EventResult::Continue,
    };

    if move_type != "Ground" {
        return EventResult::Continue;
    }

    // if (!target) return;
    let target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // if (!target.runImmunity('Ground'))
    let has_immunity = crate::Pokemon::run_immunity(battle, target, "Ground", false);
    let has_flying_type = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target_pokemon.has_type(battle, "Flying")
    };

    if !has_immunity {
        // if (target.hasType('Flying')) return 0;
        if has_flying_type {
            return EventResult::Number(0);
        }
    }

    EventResult::Continue
}
