//! Syrup Bomb Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;
use crate::Pokemon;

pub mod condition {
    use super::*;

    /// onStart(pokemon) {
    ///     this.add('-start', pokemon, 'Syrup Bomb');
    /// }
    pub fn on_start(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
        let pokemon = pokemon_pos;

        // this.add('-start', pokemon, 'Syrup Bomb');
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
                crate::battle::Arg::from("Syrup Bomb"),
            ],
        );

        EventResult::Continue
    }

    /// onUpdate(pokemon) {
    ///     if (this.effectState.source && !this.effectState.source.isActive) {
    ///         pokemon.removeVolatile('syrupbomb');
    ///     }
    /// }
    pub fn on_update(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
        use crate::dex_data::ID;

        let pokemon = pokemon_pos;

        // if (this.effectState.source && !this.effectState.source.isActive)
        let should_remove = battle.with_effect_state_ref(|state| {
            if let Some(source_pos) = state.source {
                // Need to check if source is active - we'll return the source_pos
                Some(source_pos)
            } else {
                None
            }
        }).flatten();

        let should_remove = if let Some(source_pos) = should_remove {
            let source_pokemon = match battle.pokemon_at(source_pos.0, source_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            !source_pokemon.is_active
        } else {
            false
        };

        if should_remove {
            // pokemon.removeVolatile('syrupbomb');
            Pokemon::remove_volatile(battle, pokemon, &ID::from("syrupbomb"));
        }

        EventResult::Continue
    }

    /// onResidual(pokemon) {
    ///     this.boost({ spe: -1 }, pokemon, this.effectState.source);
    /// }
    pub fn on_residual(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
        let pokemon = pokemon_pos;

        // this.boost({ spe: -1 }, pokemon, this.effectState.source);
        let source_pos = battle.with_effect_state_ref(|state| state.source).flatten();

        // Boost spe by -1
        let boosts = [("spe", -1i8)];

        battle.boost(&boosts, pokemon, source_pos, None, false, false);

        EventResult::Continue
    }

    /// onEnd(pokemon) {
    ///     this.add('-end', pokemon, 'Syrup Bomb', '[silent]');
    /// }
    pub fn on_end(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
        let pokemon = pokemon_pos;

        // this.add('-end', pokemon, 'Syrup Bomb', '[silent]');
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
                crate::battle::Arg::from("Syrup Bomb"),
                crate::battle::Arg::from("[silent]"),
            ],
        );

        EventResult::Continue
    }
}
