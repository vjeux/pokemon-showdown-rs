//! Ally Switch Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::{Battle, Arg};
use crate::event::EventResult;
use crate::dex_data::{ID, GameType};

/// onPrepareHit(pokemon) {
///     return pokemon.addVolatile('allyswitch');
/// }
pub fn on_prepare_hit(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    // return pokemon.addVolatile('allyswitch');
    let pokemon = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };

    let result = pokemon.add_volatile(ID::from("allyswitch"));
    EventResult::Boolean(result)
}

/// onHit(pokemon) {
///     let success = true;
///     // Fail in formats where you don't control allies
///     if (this.format.gameType !== 'doubles' && this.format.gameType !== 'triples') success = false;
/// 
///     // Fail in triples if the Pokemon is in the middle
///     if (pokemon.side.active.length === 3 && pokemon.position === 1) success = false;
/// 
///     const newPosition = (pokemon.position === 0 ? pokemon.side.active.length - 1 : 0);
///     if (!pokemon.side.active[newPosition]) success = false;
///     if (pokemon.side.active[newPosition].fainted) success = false;
///     if (!success) {
///         this.add('-fail', pokemon, 'move: Ally Switch');
///         this.attrLastMove('[still]');
///         return this.NOT_FAIL;
///     }
///     this.swapPosition(pokemon, newPosition, '[from] move: Ally Switch');
/// }
pub fn on_hit(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    // let success = true;
    let mut success = true;

    // Fail in formats where you don't control allies
    // if (this.format.gameType !== 'doubles' && this.format.gameType !== 'triples') success = false;
    if battle.game_type != GameType::Doubles && battle.game_type != GameType::Triples {
        success = false;
    }

    // Get the pokemon and side
    let pokemon_side_idx = pokemon_pos.0;
    let pokemon_idx = pokemon_pos.1;

    let (pokemon_position, side_active_len) = {
        let pokemon = match battle.pokemon_at(pokemon_side_idx, pokemon_idx) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        (pokemon.position, battle.sides[pokemon_side_idx].active.len())
    };

    // Fail in triples if the Pokemon is in the middle
    // if (pokemon.side.active.length === 3 && pokemon.position === 1) success = false;
    if side_active_len == 3 && pokemon_position == 1 {
        success = false;
    }

    // const newPosition = (pokemon.position === 0 ? pokemon.side.active.length - 1 : 0);
    let new_position = if pokemon_position == 0 {
        side_active_len - 1
    } else {
        0
    };

    // if (!pokemon.side.active[newPosition]) success = false;
    if battle.sides[pokemon_side_idx].active.get(new_position).and_then(|&idx| idx).is_none() {
        success = false;
    }

    // if (pokemon.side.active[newPosition].fainted) success = false;
    if let Some(Some(poke_idx)) = battle.sides[pokemon_side_idx].active.get(new_position) {
        if let Some(other_pokemon) = battle.pokemon_at(pokemon_side_idx, *poke_idx) {
            if other_pokemon.fainted {
                success = false;
            }
        }
    }

    // if (!success) {
    if !success {
        // this.add('-fail', pokemon, 'move: Ally Switch');
        let pokemon_ident = {
            let pokemon = match battle.pokemon_at(pokemon_side_idx, pokemon_idx) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            format!("p{}{}", pokemon_side_idx + 1, pokemon.ident)
        };
        battle.add("-fail", &[pokemon_ident.into(), "move: Ally Switch".into()]);

        // this.attrLastMove('[still]');
        battle.attr_last_move(&["[still]"]);

        // return this.NOT_FAIL;
        return EventResult::NotFail;
    }

    // this.swapPosition(pokemon, newPosition, '[from] move: Ally Switch');
    battle.swap_position(pokemon_pos, new_position, Some("[from] move: Ally Switch"));

    EventResult::Continue
}

pub mod condition {
    use super::*;

    /// onStart() {
    ///     this.effectState.counter = 3;
    /// }
    pub fn on_start(battle: &mut Battle, source_pos: (usize, usize)) -> EventResult {
        // Get the pokemon to access its volatile effect state
        let pokemon = match battle.pokemon_at_mut(source_pos.0, source_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        // Get the allyswitch volatile effect state
        if let Some(volatile) = pokemon.volatiles.get_mut(&ID::from("allyswitch")) {
            // this.effectState.counter = 3;
            volatile.set_i32("counter", 3);
        }

        EventResult::Continue
    }

    /// onRestart(pokemon) {
    ///     // this.effectState.counter should never be undefined here.
    ///     // However, just in case, use 1 if it is undefined.
    ///     const counter = this.effectState.counter || 1;
    ///     this.debug(`Ally Switch success chance: ${Math.round(100 / counter)}%`);
    ///     const success = this.randomChance(1, counter);
    ///     if (!success) {
    ///         delete pokemon.volatiles['allyswitch'];
    ///         return false;
    ///     }
    ///     if (this.effectState.counter < (this.effect as Condition).counterMax!) {
    ///         this.effectState.counter *= 3;
    ///     }
    ///     this.effectState.duration = 2;
    /// }
    pub fn on_restart(battle: &mut Battle, source_pos: (usize, usize)) -> EventResult {
        // Get the counter from the pokemon's volatile
        let allyswitch_id = ID::from("allyswitch");
        let counter = {
            let pokemon = match battle.pokemon_at(source_pos.0, source_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };

            // const counter = this.effectState.counter || 1;
            if let Some(volatile) = pokemon.volatiles.get(&allyswitch_id) {
                volatile.get_i32("counter").unwrap_or(1)
            } else {
                return EventResult::Continue;
            }
        };

        // this.debug(`Ally Switch success chance: ${Math.round(100 / counter)}%`);
        let success_percent = (100.0 / counter as f64).round() as i32;
        battle.debug(&format!("Ally Switch success chance: {}%", success_percent));

        // const success = this.randomChance(1, counter);
        let success = battle.random_chance(1, counter);

        // Determine what to do based on success
        let (new_counter, should_remove) = if !success {
            // if (!success) {
            //     delete pokemon.volatiles['allyswitch'];
            //     return false;
            // }
            (counter, true)
        } else {
            // if (this.effectState.counter < (this.effect as Condition).counterMax!) {
            //     this.effectState.counter *= 3;
            // }
            let new_counter = if counter < 729 {
                counter * 3
            } else {
                counter
            };

            (new_counter, false)
        };

        if should_remove {
            let pokemon = match battle.pokemon_at_mut(source_pos.0, source_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon.volatiles.remove(&allyswitch_id);
            return EventResult::Boolean(false);
        }

        // Update the counter and duration
        let pokemon = match battle.pokemon_at_mut(source_pos.0, source_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        if let Some(volatile) = pokemon.volatiles.get_mut(&allyswitch_id) {
            volatile.set_i32("counter", new_counter);
            // this.effectState.duration = 2;
            volatile.duration = Some(2);
        }

        EventResult::Continue
    }
}
