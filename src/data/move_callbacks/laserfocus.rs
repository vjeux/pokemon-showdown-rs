//! Laser Focus Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

pub mod condition {
    use super::*;

    /// onStart(pokemon, source, effect) {
    ///     if (effect && (['costar', 'imposter', 'psychup', 'transform'].includes(effect.id))) {
    ///         this.add('-start', pokemon, 'move: Laser Focus', '[silent]');
    ///     } else {
    ///         this.add('-start', pokemon, 'move: Laser Focus');
    ///     }
    /// }
    pub fn on_start(
        battle: &mut Battle,
        pokemon_pos: (usize, usize),
        _source_pos: Option<(usize, usize)>,
        effect_id: Option<&str>,
    ) -> EventResult {
        use crate::dex_data::ID;

        let pokemon = pokemon_pos;

        // if (effect && (['costar', 'imposter', 'psychup', 'transform'].includes(effect.id))) {
        //     this.add('-start', pokemon, 'move: Laser Focus', '[silent]');
        // } else {
        //     this.add('-start', pokemon, 'move: Laser Focus');
        // }
        let is_silent_effect = if let Some(eff_id) = effect_id {
            let id = ID::from(eff_id);
            id == ID::from("costar")
                || id == ID::from("imposter")
                || id == ID::from("psychup")
                || id == ID::from("transform")
        } else {
            false
        };

        let pokemon_arg = {
            let pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon.get_slot()
        };
        if is_silent_effect {
            battle.add(
                "-start",
                &[
                    pokemon_arg.clone().into(),
                    "move: Laser Focus".into(),
                    "[silent]".into(),
                ],
            );
        } else {
            battle.add("-start", &[pokemon_arg.into(), "move: Laser Focus".into()]);
        }

        EventResult::Continue
    }

    /// onRestart(pokemon) {
    ///     this.effectState.duration = 2;
    ///     this.add('-start', pokemon, 'move: Laser Focus');
    /// }
    pub fn on_restart(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
        let pokemon = pokemon_pos;

        // this.effectState.duration = 2;
        battle.with_effect_state(|state| {
            state.duration = Some(2);
        });

        // this.add('-start', pokemon, 'move: Laser Focus');
        let pokemon_arg = {
            let pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon.get_slot()
        };
        battle.add("-start", &[pokemon_arg.into(), "move: Laser Focus".into()]);

        EventResult::Continue
    }

    /// onModifyCritRatio(critRatio) {
    ///     return 5;
    /// }
    pub fn on_modify_crit_ratio(_battle: &mut Battle) -> EventResult {
        // return 5;
        EventResult::Number(5)
    }

    /// onEnd(pokemon) {
    ///     this.add('-end', pokemon, 'move: Laser Focus', '[silent]');
    /// }
    pub fn on_end(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
        let pokemon = pokemon_pos;

        // this.add('-end', pokemon, 'move: Laser Focus', '[silent]');
        let pokemon_arg = {
            let pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon.get_slot()
        };
        battle.add(
            "-end",
            &[
                pokemon_arg.into(),
                "move: Laser Focus".into(),
                "[silent]".into(),
            ],
        );

        EventResult::Continue
    }
}
