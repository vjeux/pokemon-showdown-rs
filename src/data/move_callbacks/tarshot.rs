//! Tar Shot Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

pub mod condition {
    use super::*;

    /// onStart(pokemon) {
    ///     if (pokemon.terastallized) return false;
    ///     this.add('-start', pokemon, 'Tar Shot');
    /// }
    pub fn on_start(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
        let pokemon = pokemon_pos;

        // if (pokemon.terastallized) return false;
        let is_terastallized = {
            let pokemon_ref = match battle.pokemon_at(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon_ref.terastallized.is_some()
        };

        if is_terastallized {
            return EventResult::NotFail;
        }

        // this.add('-start', pokemon, 'Tar Shot');
        let pokemon_slot = {
            let pokemon_ref = match battle.pokemon_at(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon_ref.get_slot()
        };

        battle.add(
            "-start",
            &[
                crate::battle::Arg::from(pokemon_slot),
                crate::battle::Arg::from("Tar Shot"),
            ],
        );

        EventResult::Continue
    }

    /// onEffectiveness(typeMod, target, type, move) {
    ///     if (move.type !== 'Fire') return;
    ///     if (!target) return;
    ///     if (type !== target.getTypes()[0]) return;
    ///     return typeMod + 1;
    /// }
    pub fn on_effectiveness(
        _battle: &mut Battle,
        _target_pos: Option<(usize, usize)>,
        _move_id: &str,
    ) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }
}
