//! Octolock Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onTryImmunity(target) {
///     return this.dex.getImmunity('trapped', target);
/// }
pub fn on_try_immunity(battle: &mut Battle, target_pos: Option<(usize, usize)>) -> EventResult {
    let target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // return this.dex.getImmunity('trapped', target);
    let immune = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        battle.dex.get_immunity("trapped", &target_pokemon.types)
    };

    if immune {
        EventResult::Continue
    } else {
        EventResult::Boolean(false)
    }
}

pub mod condition {
    use super::*;

    /// onStart(pokemon, source) {
    ///     this.add('-start', pokemon, 'move: Octolock', `[of] ${source}`);
    /// }
    pub fn on_start(battle: &mut Battle, pokemon_pos: (usize, usize), source_pos: Option<(usize, usize)>) -> EventResult {
        let pokemon = pokemon_pos;

        // this.add('-start', pokemon, 'move: Octolock', `[of] ${source}`);
        let (pokemon_arg, source_arg) = {
            let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            let pokemon_arg = crate::battle::Arg::from(pokemon_pokemon);

            let source_arg = if let Some(source) = source_pos {
                let source_pokemon = match battle.pokemon_at(source.0, source.1) {
                    Some(p) => p,
                    None => return EventResult::Continue,
                };
                format!("[of] {}", crate::battle::Arg::from(source_pokemon))
            } else {
                "".to_string()
            };

            (pokemon_arg, source_arg)
        };

        battle.add("-start", &[
            pokemon_arg,
            "move: Octolock".into(),
            source_arg.into(),
        ]);

        EventResult::Continue
    }

    /// onResidual(pokemon) {
    ///     const source = this.effectState.source;
    ///     if (source && (!source.isActive || source.hp <= 0 || !source.activeTurns)) {
    ///         delete pokemon.volatiles['octolock'];
    ///         this.add('-end', pokemon, 'Octolock', '[partiallytrapped]', '[silent]');
    ///         return;
    ///     }
    ///     this.boost({ def: -1, spd: -1 }, pokemon, source, this.dex.getActiveMove('octolock'));
    /// }
    pub fn on_residual(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
        use crate::dex_data::ID;

        let pokemon = pokemon_pos;

        // const source = this.effectState.source;
        let source_pos = match &battle.current_effect_state {
            Some(es) => es.source,
            None => None,
        };

        // if (source && (!source.isActive || source.hp <= 0 || !source.activeTurns)) {
        if let Some(source) = source_pos {
            let (is_active, hp, active_turns) = {
                let source_pokemon = match battle.pokemon_at(source.0, source.1) {
                    Some(p) => p,
                    None => return EventResult::Continue,
                };
                (source_pokemon.is_active, source_pokemon.hp, source_pokemon.active_turns)
            };

            if !is_active || hp <= 0 || active_turns == 0 {
                // delete pokemon.volatiles['octolock'];
                {
                    let pokemon_pokemon = match battle.pokemon_at_mut(pokemon.0, pokemon.1) {
                        Some(p) => p,
                        None => return EventResult::Continue,
                    };
                    pokemon_pokemon.remove_volatile(&ID::from("octolock"));
                }

                // this.add('-end', pokemon, 'Octolock', '[partiallytrapped]', '[silent]');
                let pokemon_arg = {
                    let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
                        Some(p) => p,
                        None => return EventResult::Continue,
                    };
                    crate::battle::Arg::from(pokemon_pokemon)
                };

                battle.add("-end", &[
                    pokemon_arg,
                    "Octolock".into(),
                    "[partiallytrapped]".into(),
                    "[silent]".into(),
                ]);

                // return;
                return EventResult::Continue;
            }

            // this.boost({ def: -1, spd: -1 }, pokemon, source, this.dex.getActiveMove('octolock'));
            battle.boost(&[("def", -1), ("spd", -1)], pokemon, Some(source), Some("octolock"));
        }

        EventResult::Continue
    }

    /// onTrapPokemon(pokemon) {
    ///     if (this.effectState.source?.isActive) pokemon.tryTrap();
    /// }
    pub fn on_trap_pokemon(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
        let pokemon = pokemon_pos;

        // if (this.effectState.source?.isActive) pokemon.tryTrap();
        let source_pos = match &battle.current_effect_state {
            Some(es) => es.source,
            None => None,
        };

        if let Some(source) = source_pos {
            let is_active = {
                let source_pokemon = match battle.pokemon_at(source.0, source.1) {
                    Some(p) => p,
                    None => return EventResult::Continue,
                };
                source_pokemon.is_active
            };

            if is_active {
                // pokemon.tryTrap();
                let pokemon_pokemon = match battle.pokemon_at_mut(pokemon.0, pokemon.1) {
                    Some(p) => p,
                    None => return EventResult::Continue,
                };
                pokemon_pokemon.try_trap(false);
            }
        }

        EventResult::Continue
    }
}
