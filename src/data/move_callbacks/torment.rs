//! Torment Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;
use crate::Pokemon;

pub mod condition {
    use super::*;

    /// onStart(pokemon, source, effect) {
    ///     if (pokemon.volatiles['dynamax']) {
    ///         delete pokemon.volatiles['torment'];
    ///         return false;
    ///     }
    ///     if (effect?.id === 'gmaxmeltdown') this.effectState.duration = 3;
    ///     this.add('-start', pokemon, 'Torment');
    /// }
    pub fn on_start(
        battle: &mut Battle,
        pokemon_pos: (usize, usize),
        _source_pos: Option<(usize, usize)>,
        effect: Option<&crate::battle::Effect>,
    ) -> EventResult {
        use crate::dex_data::ID;

        let pokemon = pokemon_pos;

        // if (pokemon.volatiles['dynamax']) {
        //     delete pokemon.volatiles['torment'];
        //     return false;
        // }
        let has_dynamax = {
            let pokemon_ref = match battle.pokemon_at(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon_ref.has_volatile(&ID::from("dynamax"))
        };

        if has_dynamax {
            Pokemon::remove_volatile(battle, pokemon, &ID::from("torment"));
            return EventResult::NotFail;
        }

        // if (effect?.id === 'gmaxmeltdown') this.effectState.duration = 3;
        if let Some(eff) = effect {
            if eff.id.as_str() == "gmaxmeltdown" {
                battle.with_effect_state(|state| {
                    state.duration = Some(3);
                });
            }
        }

        // this.add('-start', pokemon, 'Torment');
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
                crate::battle::Arg::from("Torment"),
            ],
        );

        EventResult::Continue
    }

    /// onEnd(pokemon) {
    ///     this.add('-end', pokemon, 'Torment');
    /// }
    pub fn on_end(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
        let pokemon = pokemon_pos;

        // this.add('-end', pokemon, 'Torment');
        let pokemon_slot = {
            let pokemon_ref = match battle.pokemon_at(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon_ref.get_slot()
        };

        battle.add(
            "-end",
            &[
                crate::battle::Arg::from(pokemon_slot),
                crate::battle::Arg::from("Torment"),
            ],
        );

        EventResult::Continue
    }

    /// onDisableMove(pokemon) {
    ///     if (pokemon.lastMove && pokemon.lastMove.id !== 'struggle') pokemon.disableMove(pokemon.lastMove.id);
    /// }
    pub fn on_disable_move(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
        use crate::dex_data::ID;

        let pokemon = pokemon_pos;

        // if (pokemon.lastMove && pokemon.lastMove.id !== 'struggle') pokemon.disableMove(pokemon.lastMove.id);
        let last_move_id = {
            let pokemon_ref = match battle.pokemon_at(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };

            if let Some(ref last_move) = pokemon_ref.last_move {
                if *last_move != ID::from("struggle") {
                    Some(last_move.clone())
                } else {
                    None
                }
            } else {
                None
            }
        };

        if let Some(move_id) = last_move_id {
            let pokemon_mut = match battle.pokemon_at_mut(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };

            pokemon_mut.disable_move(&move_id.to_string(), false, None);
        }

        EventResult::Continue
    }
}
