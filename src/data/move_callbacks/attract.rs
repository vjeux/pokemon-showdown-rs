//! Attract Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::dex_data::{Gender, ID};
use crate::event::EventResult;
use crate::Pokemon;

/// ```ignore
/// onTryImmunity(target, source) {
///     return (target.gender === 'M' && source.gender === 'F') || (target.gender === 'F' && source.gender === 'M');
/// }
/// ```
pub fn on_try_immunity(
    battle: &mut Battle,
    target_pos: Option<(usize, usize)>,
    source_pos: Option<(usize, usize)>,
) -> EventResult {
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
    let is_compatible = (target_pokemon.gender == Gender::Male
        && source_pokemon.gender == Gender::Female)
        || (target_pokemon.gender == Gender::Female && source_pokemon.gender == Gender::Male);

    EventResult::Boolean(is_compatible)
}

pub mod condition {
    use super::*;

    /// ```ignore
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
    /// ```
    pub fn on_start(
        battle: &mut Battle,
        pokemon_pos: (usize, usize),
        source_pos: Option<(usize, usize)>,
        effect: Option<&crate::battle::Effect>,
    ) -> EventResult {
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
        let is_compatible = (pokemon.gender == Gender::Male
            && source_pokemon.gender == Gender::Female)
            || (pokemon.gender == Gender::Female && source_pokemon.gender == Gender::Male);

        if !is_compatible {
            battle.debug("incompatible gender");
            return EventResult::Boolean(false);
        }

        // if (!this.runEvent('Attract', pokemon, source)) {
        //     this.debug('Attract event failed');
        //     return false;
        // }
        let event_result = battle.run_event("Attract", Some(crate::event::EventTarget::Pokemon(pokemon_pos)), Some(source), None, EventResult::Continue, false, false);
        // JavaScript: if (!result) return false;
        // This checks if result is FALSY, not if it equals 0
        if !event_result.is_truthy() {
            battle.debug("Attract event failed");
            return EventResult::Boolean(false);
        }

        // Get pokemon identifier and source name for battle.add
        let (pokemon_ident, source_name) = {
            let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };

            let source_pokemon = match battle.pokemon_at(source.0, source.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };

            (pokemon.get_slot(), source_pokemon.name.clone())
        };

        // if (effect.name === 'Cute Charm') {
        //     this.add('-start', pokemon, 'Attract', '[from] ability: Cute Charm', `[of] ${source}`);
        // } else if (effect.name === 'Destiny Knot') {
        //     this.add('-start', pokemon, 'Attract', '[from] item: Destiny Knot', `[of] ${source}`);
        // } else {
        //     this.add('-start', pokemon, 'Attract');
        // }
        if let Some(eff) = effect {
            if eff.name == "Cute Charm" {
                battle.add(
                    "-start",
                    &[
                        pokemon_ident.as_str().into(),
                        "Attract".into(),
                        "[from] ability: Cute Charm".into(),
                        format!("[of] {}", source_name).into(),
                    ],
                );
            } else if eff.name == "Destiny Knot" {
                battle.add(
                    "-start",
                    &[
                        pokemon_ident.as_str().into(),
                        "Attract".into(),
                        "[from] item: Destiny Knot".into(),
                        format!("[of] {}", source_name).into(),
                    ],
                );
            } else {
                battle.add("-start", &[pokemon_ident.as_str().into(), "Attract".into()]);
            }
        } else {
            battle.add("-start", &[pokemon_ident.as_str().into(), "Attract".into()]);
        }

        // Store the source in the effect state
        // JavaScript: this.effectState.source = ...
        battle.with_effect_state(|state| {
            state.source = Some(source);
        });

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
            if let Some(source_pos) = volatile.borrow().source {
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
            Pokemon::remove_volatile(battle, pokemon_pos, &attract_id);
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
    pub fn on_before_move(
        battle: &mut Battle,
        pokemon_pos: (usize, usize),
        _target_pos: Option<(usize, usize)>,
        _active_move: Option<&crate::battle_actions::ActiveMove>,
    ) -> EventResult {
        // Get the pokemon to access its volatiles
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        // Get the source from effect state
        let attract_id = ID::from("attract");
        let source_pos = if let Some(volatile) = pokemon.volatiles.get(&attract_id) {
            volatile.borrow().source
        } else {
            None
        };

        // this.add('-activate', pokemon, 'move: Attract', '[of] ' + this.effectState.source);
        if let Some(source) = source_pos {
            let (pokemon_ident, source_name) = {
                let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                    Some(p) => p,
                    None => return EventResult::Continue,
                };

                let source_pokemon = match battle.pokemon_at(source.0, source.1) {
                    Some(p) => p,
                    None => return EventResult::Continue,
                };

                (pokemon.get_slot(), source_pokemon.name.clone())
            };

            battle.add(
                "-activate",
                &[
                    pokemon_ident.as_str().into(),
                    "move: Attract".into(),
                    format!("[of] {}", source_name).into(),
                ],
            );
        }

        // if (this.randomChance(1, 2)) {
        if battle.random_chance(1.0, 2) {
            // this.add('cant', pokemon, 'Attract');
            let pokemon_ident = {
                let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                    Some(p) => p,
                    None => return EventResult::Continue,
                };
                pokemon.get_slot()
            };
            battle.add("cant", &[pokemon_ident.as_str().into(), "Attract".into()]);

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
        let pokemon_ident = {
            let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon.get_slot()
        };

        battle.add(
            "-end",
            &[
                pokemon_ident.as_str().into(),
                "Attract".into(),
                "[silent]".into(),
            ],
        );

        EventResult::Continue
    }
}
