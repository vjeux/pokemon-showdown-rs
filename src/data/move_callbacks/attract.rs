//! Attract Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::{Battle, Arg};
use crate::event::EventResult;
use crate::dex_data::{ID, Gender};

/// onTryImmunity(target, source) {
///     return (target.gender === 'M' && source.gender === 'F') || (target.gender === 'F' && source.gender === 'M');
/// }
pub fn on_try_immunity(battle: &mut Battle, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>) -> EventResult {
    // Get the target
    let target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    let target_pokemon = match battle.pokemon_at(target.0, target.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };

    // Get the source
    let source = match source_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    let source_pokemon = match battle.pokemon_at(source.0, source.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };

    // return (target.gender === 'M' && source.gender === 'F') || (target.gender === 'F' && source.gender === 'M');
    let is_compatible =
        (target_pokemon.gender == Gender::Male && source_pokemon.gender == Gender::Female) ||
        (target_pokemon.gender == Gender::Female && source_pokemon.gender == Gender::Male);

    EventResult::Boolean(is_compatible)
}

pub mod condition {
    use super::*;

    /// onStart(pokemon, source, effect) {
    ///     if (!(pokemon.gender === 'M' && source.gender === 'F') && !(pokemon.gender === 'F' && source.gender === 'M')) {
    ///         this.debug('incompatible gender');
    ///         return false;
    ///     }
    ///     if (!this.runEvent('Attract', pokemon, source)) {
    ///         this.debug('Attract event failed');
    ///         return false;
    ///     }
    ///
    ///     if (effect.name === 'Cute Charm') {
    ///         this.add('-start', pokemon, 'Attract', '[from] ability: Cute Charm', `[of] ${source}`);
    ///     } else if (effect.name === 'Destiny Knot') {
    ///         this.add('-start', pokemon, 'Attract', '[from] item: Destiny Knot', `[of] ${source}`);
    ///     } else {
    ///         this.add('-start', pokemon, 'Attract');
    ///     }
    /// }
    pub fn on_start(battle: &mut Battle, pokemon_pos: (usize, usize), source_pos: Option<(usize, usize)>, effect_id: Option<&str>) -> EventResult {
        // Get source position
        let source = match source_pos {
            Some(pos) => pos,
            None => return EventResult::Continue,
        };

        // Get the pokemon
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        // Get the source pokemon
        let source_pokemon = match battle.pokemon_at(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        // if (!(pokemon.gender === 'M' && source.gender === 'F') && !(pokemon.gender === 'F' && source.gender === 'M')) {
        //     this.debug('incompatible gender');
        //     return false;
        // }
        let is_compatible =
            (pokemon.gender == Gender::Male && source_pokemon.gender == Gender::Female) ||
            (pokemon.gender == Gender::Female && source_pokemon.gender == Gender::Male);

        if !is_compatible {
            battle.debug("incompatible gender");
            return EventResult::Boolean(false);
        }

        // if (!this.runEvent('Attract', pokemon, source)) {
        //     this.debug('Attract event failed');
        //     return false;
        // }
        let event_result = battle.run_event("Attract", Some(pokemon_pos), Some(source), None, None);
        if !event_result {
            battle.debug("Attract event failed");
            return EventResult::Boolean(false);
        }

        // Get pokemon and source again for battle.add
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        let source_pokemon = match battle.pokemon_at(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        // if (effect.name === 'Cute Charm') {
        //     this.add('-start', pokemon, 'Attract', '[from] ability: Cute Charm', `[of] ${source}`);
        // } else if (effect.name === 'Destiny Knot') {
        //     this.add('-start', pokemon, 'Attract', '[from] item: Destiny Knot', `[of] ${source}`);
        // } else {
        //     this.add('-start', pokemon, 'Attract');
        // }
        if let Some(effect_name) = effect_id {
            if effect_name == "Cute Charm" {
                battle.add("-start", &[
                    pokemon.into(),
                    "Attract".into(),
                    "[from] ability: Cute Charm".into(),
                    format!("[of] {}", source_pokemon.name).into(),
                ]);
            } else if effect_name == "Destiny Knot" {
                battle.add("-start", &[
                    pokemon.into(),
                    "Attract".into(),
                    "[from] item: Destiny Knot".into(),
                    format!("[of] {}", source_pokemon.name).into(),
                ]);
            } else {
                battle.add("-start", &[pokemon.into(), "Attract".into()]);
            }
        } else {
            battle.add("-start", &[pokemon.into(), "Attract".into()]);
        }

        // Store the source in the effect state
        let pokemon_mut = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        if let Some(volatile) = pokemon_mut.volatiles.get_mut(&ID::from("attract")) {
            volatile.source = Some(source);
        }

        EventResult::Continue
    }

    /// onUpdate(pokemon) {
    ///     if (this.effectState.source && !this.effectState.source.isActive && pokemon.volatiles['attract']) {
    ///         this.debug(`Removing Attract volatile on ${pokemon}`);
    ///         pokemon.removeVolatile('attract');
    ///     }
    /// }
    pub fn on_update(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
        // Get the pokemon to access its volatiles
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        // if (this.effectState.source && !this.effectState.source.isActive && pokemon.volatiles['attract']) {
        let attract_id = ID::from("attract");
        let should_remove = if let Some(volatile) = pokemon.volatiles.get(&attract_id) {
            if let Some(source_pos) = volatile.source {
                // Check if source is not active
                if let Some(source_pokemon) = battle.pokemon_at(source_pos.0, source_pos.1) {
                    !source_pokemon.is_active
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            false
        };

        if should_remove {
            // this.debug(`Removing Attract volatile on ${pokemon}`);
            let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            battle.debug(&format!("Removing Attract volatile on {}", pokemon.name));

            // pokemon.removeVolatile('attract');
            let pokemon_mut = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon_mut.remove_volatile(&attract_id);
        }

        EventResult::Continue
    }

    /// onBeforeMove(pokemon, target, move) {
    ///     this.add('-activate', pokemon, 'move: Attract', '[of] ' + this.effectState.source);
    ///     if (this.randomChance(1, 2)) {
    ///         this.add('cant', pokemon, 'Attract');
    ///         return false;
    ///     }
    /// }
    pub fn on_before_move(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>, move_id: &str) -> EventResult {
        // Get the pokemon to access its volatiles
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        // Get the source from effect state
        let attract_id = ID::from("attract");
        let source_pos = if let Some(volatile) = pokemon.volatiles.get(&attract_id) {
            volatile.source
        } else {
            None
        };

        // this.add('-activate', pokemon, 'move: Attract', '[of] ' + this.effectState.source);
        if let Some(source) = source_pos {
            let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };

            let source_pokemon = match battle.pokemon_at(source.0, source.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };

            battle.add("-activate", &[
                pokemon.into(),
                "move: Attract".into(),
                format!("[of] {}", source_pokemon.name).into(),
            ]);
        }

        // if (this.randomChance(1, 2)) {
        if battle.random_chance(1, 2) {
            // this.add('cant', pokemon, 'Attract');
            let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            battle.add("cant", &[pokemon.into(), "Attract".into()]);

            // return false;
            return EventResult::Boolean(false);
        }

        EventResult::Continue
    }

    /// onEnd(pokemon) {
    ///     this.add('-end', pokemon, 'Attract', '[silent]');
    /// }
    pub fn on_end(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
        // this.add('-end', pokemon, 'Attract', '[silent]');
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        battle.add("-end", &[pokemon.into(), "Attract".into(), "[silent]".into()]);

        EventResult::Continue
    }
}
