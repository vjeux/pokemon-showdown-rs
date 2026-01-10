//! Smack Down Move
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
    ///     let applies = false;
    ///     if (pokemon.hasType('Flying') || pokemon.hasAbility('levitate')) applies = true;
    ///     if (pokemon.hasItem('ironball') || pokemon.volatiles['ingrain'] ||
    ///         this.field.getPseudoWeather('gravity')) applies = false;
    ///     if (pokemon.removeVolatile('fly') || pokemon.removeVolatile('bounce')) {
    ///         applies = true;
    ///         this.queue.cancelMove(pokemon);
    ///         pokemon.removeVolatile('twoturnmove');
    ///     }
    ///     if (pokemon.volatiles['magnetrise']) {
    ///         applies = true;
    ///         delete pokemon.volatiles['magnetrise'];
    ///     }
    ///     if (pokemon.volatiles['telekinesis']) {
    ///         applies = true;
    ///         delete pokemon.volatiles['telekinesis'];
    ///     }
    ///     if (!applies) return false;
    ///     this.add('-start', pokemon, 'Smack Down');
    /// }
    pub fn on_start(
        battle: &mut Battle,
        pokemon_pos: (usize, usize),
        _source_pos: Option<(usize, usize)>,
        _effect: Option<&crate::battle::Effect>,
    ) -> EventResult {
        use crate::dex_data::ID;

        // onStart(pokemon) {
        //     let applies = false;
        //     if (pokemon.hasType('Flying') || pokemon.hasAbility('levitate')) applies = true;
        //     if (pokemon.hasItem('ironball') || pokemon.volatiles['ingrain'] ||
        //         this.field.getPseudoWeather('gravity')) applies = false;
        //     if (pokemon.removeVolatile('fly') || pokemon.removeVolatile('bounce')) {
        //         applies = true;
        //         this.queue.cancelMove(pokemon);
        //         pokemon.removeVolatile('twoturnmove');
        //     }
        //     if (pokemon.volatiles['magnetrise']) {
        //         applies = true;
        //         delete pokemon.volatiles['magnetrise'];
        //     }
        //     if (pokemon.volatiles['telekinesis']) {
        //         applies = true;
        //         delete pokemon.volatiles['telekinesis'];
        //     }
        //     if (!applies) return false;
        //     this.add('-start', pokemon, 'Smack Down');
        // }
        let pokemon = pokemon_pos;

        // let applies = false;
        let mut applies = false;

        // if (pokemon.hasType('Flying') || pokemon.hasAbility('levitate')) applies = true;
        let has_flying = {
            let pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon.has_type(battle, "Flying")
        };
        let has_levitate = {
            let pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon.has_ability(battle, &["levitate"])
        };
        if has_flying || has_levitate {
            applies = true;
        }

        // if (pokemon.hasItem('ironball') || pokemon.volatiles['ingrain'] ||
        //     this.field.getPseudoWeather('gravity')) applies = false;
        let has_ironball = {
            let pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon.has_item(battle, &["ironball"])
        };
        let has_ingrain = {
            let pokemon_data = match battle.pokemon_at(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon_data.volatiles.contains_key(&ID::from("ingrain"))
        };
        let has_gravity = battle
            .field
            .pseudo_weather
            .contains_key(&ID::from("gravity"));

        if has_ironball || has_ingrain || has_gravity {
            applies = false;
        }

        // if (pokemon.removeVolatile('fly') || pokemon.removeVolatile('bounce')) {
        //     applies = true;
        //     this.queue.cancelMove(pokemon);
        //     pokemon.removeVolatile('twoturnmove');
        // }
        let removed_fly = Pokemon::remove_volatile(battle, pokemon, &ID::from("fly"));
        let removed_bounce = Pokemon::remove_volatile(battle, pokemon, &ID::from("bounce"));

        if removed_fly || removed_bounce {
            applies = true;
            // this.queue.cancelMove(pokemon);
            battle.queue.cancel_move(pokemon.0, pokemon.1);
            {
                Pokemon::remove_volatile(battle, pokemon, &ID::from("twoturnmove"));
            }
        }

        // if (pokemon.volatiles['magnetrise']) {
        //     applies = true;
        //     delete pokemon.volatiles['magnetrise'];
        // }
        let has_magnetrise = {
            let pokemon_data = match battle.pokemon_at(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon_data.volatiles.contains_key(&ID::from("magnetrise"))
        };

        if has_magnetrise {
            applies = true;
            {
                Pokemon::remove_volatile(battle, pokemon, &ID::from("magnetrise"));
            }
        }

        // if (pokemon.volatiles['telekinesis']) {
        //     applies = true;
        //     delete pokemon.volatiles['telekinesis'];
        // }
        let has_telekinesis = {
            let pokemon_data = match battle.pokemon_at(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon_data
                .volatiles
                .contains_key(&ID::from("telekinesis"))
        };

        if has_telekinesis {
            applies = true;
            {
                Pokemon::remove_volatile(battle, pokemon, &ID::from("telekinesis"));
            }
        }

        // if (!applies) return false;
        if !applies {
            return EventResult::Boolean(false);
        }

        // this.add('-start', pokemon, 'Smack Down');
        let pokemon_arg = {
            let pokemon_data = match battle.pokemon_at(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon_data.get_slot()
        };

        battle.add("-start", &[pokemon_arg.into(), "Smack Down".into()]);

        EventResult::Continue
    }

    /// onRestart(pokemon) {
    ///     if (pokemon.removeVolatile('fly') || pokemon.removeVolatile('bounce')) {
    ///         this.queue.cancelMove(pokemon);
    ///         pokemon.removeVolatile('twoturnmove');
    ///         this.add('-start', pokemon, 'Smack Down');
    ///     }
    /// }
    pub fn on_restart(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
        use crate::dex_data::ID;

        // onRestart(pokemon) {
        //     if (pokemon.removeVolatile('fly') || pokemon.removeVolatile('bounce')) {
        //         this.queue.cancelMove(pokemon);
        //         pokemon.removeVolatile('twoturnmove');
        //         this.add('-start', pokemon, 'Smack Down');
        //     }
        // }
        let pokemon = pokemon_pos;

        // if (pokemon.removeVolatile('fly') || pokemon.removeVolatile('bounce')) {
        //     this.queue.cancelMove(pokemon);
        //     pokemon.removeVolatile('twoturnmove');
        //     this.add('-start', pokemon, 'Smack Down');
        // }
        let removed_fly = Pokemon::remove_volatile(battle, pokemon, &ID::from("fly"));
        let removed_bounce = Pokemon::remove_volatile(battle, pokemon, &ID::from("bounce"));

        if removed_fly || removed_bounce {
            // this.queue.cancelMove(pokemon);
            battle.queue.cancel_move(pokemon.0, pokemon.1);

            // pokemon.removeVolatile('twoturnmove');
            {
                Pokemon::remove_volatile(battle, pokemon, &ID::from("twoturnmove"));
            }

            // this.add('-start', pokemon, 'Smack Down');
            let pokemon_arg = {
                let pokemon_data = match battle.pokemon_at(pokemon.0, pokemon.1) {
                    Some(p) => p,
                    None => return EventResult::Continue,
                };
                pokemon_data.get_slot()
            };

            battle.add("-start", &[pokemon_arg.into(), "Smack Down".into()]);
        }

        EventResult::Continue
    }
}
