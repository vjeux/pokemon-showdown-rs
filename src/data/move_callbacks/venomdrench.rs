//! Venom Drench Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onHit(target, source, move) {
///     if (target.status === 'psn' || target.status === 'tox') {
///         return !!this.boost({ atk: -1, spa: -1, spe: -1 }, target, source, move);
///     }
///     return false;
/// }
pub fn on_hit(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
    target_pos: Option<(usize, usize)>,
) -> EventResult {
    use crate::dex_data::ID;

    let source = pokemon_pos;
    let target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // if (target.status === 'psn' || target.status === 'tox')
    let status = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target_pokemon.status.clone()
    };

    if status == ID::from("psn") || status == ID::from("tox") {
        // return !!this.boost({ atk: -1, spa: -1, spe: -1 }, target, source, move);
        let success = battle.boost(&[("atk", -1), ("spa", -1), ("spe", -1)], target, Some(source), None);

        if success {
            return EventResult::Continue;
        } else {
            return EventResult::NotFail;
        }
    }

    // return false;
    EventResult::NotFail
}
