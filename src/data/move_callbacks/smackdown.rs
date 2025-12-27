//! Smack Down Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;


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
    pub fn on_start(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
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
        let has_flying = battle.has_type(pokemon, "Flying");
        let has_levitate = battle.has_ability(pokemon, "levitate");
        if has_flying || has_levitate {
            applies = true;
        }

        // if (pokemon.hasItem('ironball') || pokemon.volatiles['ingrain'] ||
        //     this.field.getPseudoWeather('gravity')) applies = false;
        let has_ironball = battle.has_item(pokemon, "ironball");
        let has_ingrain = {
            let pokemon_data = match battle.pokemon_at(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon_data.volatiles.contains_key(&ID::from("ingrain"))
        };
        let has_gravity = battle.has_pseudo_weather("gravity");

        if has_ironball || has_ingrain || has_gravity {
            applies = false;
        }

        // if (pokemon.removeVolatile('fly') || pokemon.removeVolatile('bounce')) {
        //     applies = true;
        //     this.queue.cancelMove(pokemon);
        //     pokemon.removeVolatile('twoturnmove');
        // }
        let removed_fly = battle.remove_volatile(&ID::from("fly"), pokemon);
        let removed_bounce = battle.remove_volatile(&ID::from("bounce"), pokemon);

        if removed_fly || removed_bounce {
            applies = true;
            battle.cancel_move(pokemon);
            battle.remove_volatile(&ID::from("twoturnmove"), pokemon);
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
            battle.remove_volatile(&ID::from("magnetrise"), pokemon);
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
            pokemon_data.volatiles.contains_key(&ID::from("telekinesis"))
        };

        if has_telekinesis {
            applies = true;
            battle.remove_volatile(&ID::from("telekinesis"), pokemon);
        }

        // if (!applies) return false;
        if !applies {
            return EventResult::Bool(false);
        }

        // this.add('-start', pokemon, 'Smack Down');
        let pokemon_arg = {
            let pokemon_data = match battle.pokemon_at(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            crate::battle::Arg::from(pokemon_data)
        };

        battle.add("-start", &[
            pokemon_arg,
            "Smack Down".into(),
        ]);

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
        let removed_fly = battle.remove_volatile(&ID::from("fly"), pokemon);
        let removed_bounce = battle.remove_volatile(&ID::from("bounce"), pokemon);

        if removed_fly || removed_bounce {
            // this.queue.cancelMove(pokemon);
            battle.cancel_move(pokemon);

            // pokemon.removeVolatile('twoturnmove');
            battle.remove_volatile(&ID::from("twoturnmove"), pokemon);

            // this.add('-start', pokemon, 'Smack Down');
            let pokemon_arg = {
                let pokemon_data = match battle.pokemon_at(pokemon.0, pokemon.1) {
                    Some(p) => p,
                    None => return EventResult::Continue,
                };
                crate::battle::Arg::from(pokemon_data)
            };

            battle.add("-start", &[
                pokemon_arg,
                "Smack Down".into(),
            ]);
        }

        EventResult::Continue
    }
}
