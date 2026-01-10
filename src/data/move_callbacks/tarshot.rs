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
    pub fn on_start(
        battle: &mut Battle,
        pokemon_pos: (usize, usize),
        _source_pos: Option<(usize, usize)>,
        _effect: Option<&crate::battle::Effect>,
    ) -> EventResult {
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
        battle: &mut Battle,
        type_mod: i32,
        target_type: &str,
        target_pos: Option<(usize, usize)>,
    ) -> EventResult {
        // if (move.type !== 'Fire') return;
        let move_type = match &battle.active_move {
            Some(m) => m.move_type.clone(),
            None => return EventResult::Continue,
        };

        if move_type != "Fire" {
            return EventResult::Continue;
        }

        // if (!target) return;
        let target = match target_pos {
            Some(pos) => pos,
            None => return EventResult::Continue,
        };

        // if (type !== target.getTypes()[0]) return;
        let first_type = {
            let target_pokemon = match battle.pokemon_at(target.0, target.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            let types = target_pokemon.get_types(battle, false);
            types.get(0).cloned().unwrap_or_default()
        };

        if target_type != first_type {
            return EventResult::Continue;
        }

        // return typeMod + 1;
        EventResult::Number(type_mod + 1)
    }
}
