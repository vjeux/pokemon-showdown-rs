//! Slow Start Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::battle::Effect;
use crate::event::EventResult;

/// JavaScript source (data/abilities.ts:4231-4263):
///
/// slowstart: {
///     onStart(pokemon) {
///         this.add('-start', pokemon, 'ability: Slow Start');
///         this.effectState.counter = 5;
///     },
///     onResidualOrder: 28,
///     onResidualSubOrder: 2,
///     onResidual(pokemon) {
///         if (pokemon.activeTurns && this.effectState.counter) {
///             this.effectState.counter--;
///             if (!this.effectState.counter) {
///                 this.add('-end', pokemon, 'Slow Start');
///                 delete this.effectState.counter;
///             }
///         }
///     },
///     onModifyAtkPriority: 5,
///     onModifyAtk(atk, pokemon) {
///         if (this.effectState.counter) {
///             return this.chainModify(0.5);
///         }
///     },
///     onModifySpe(spe, pokemon) {
///         if (this.effectState.counter) {
///             return this.chainModify(0.5);
///         }
///     },
/// }
pub fn on_start(battle: &mut Battle, pokemon_pos: (usize, usize), _source_pos: Option<(usize, usize)>, _effect: Option<&Effect>) -> EventResult {
    // JS: this.add('-start', pokemon, 'ability: Slow Start');
    // JS: this.effectState.counter = 5;

    // Get ability state and set counter to 5
    let pokemon = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };

    pokemon.ability_state.counter = Some(5);

    EventResult::Continue
}

pub fn on_residual(battle: &mut Battle, pokemon_pos: (usize, usize), _source_pos: Option<(usize, usize)>, _effect: Option<&Effect>) -> EventResult {
    // JS: if (pokemon.activeTurns && this.effectState.counter) {
    //         this.effectState.counter--;
    //         if (!this.effectState.counter) {
    //             this.add('-end', pokemon, 'Slow Start');
    //             delete this.effectState.counter;
    //         }
    //     }

    let (active_turns, counter) = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        (pokemon.active_turns, pokemon.ability_state.counter)
    };

    if active_turns > 0 {
        if let Some(current_counter) = counter {
            if current_counter > 0 {
                let new_counter = current_counter - 1;

                let pokemon_mut = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
                    Some(p) => p,
                    None => return EventResult::Continue,
                };

                if new_counter == 0 {
                    // Counter reached 0, remove it (equivalent to delete)
                    pokemon_mut.ability_state.counter = None;
                } else {
                    pokemon_mut.ability_state.counter = Some(new_counter);
                }
            }
        }
    }

    EventResult::Continue
}

pub fn on_modify_atk(battle: &mut Battle, _atk: i32, attacker_pos: (usize, usize), _defender_pos: (usize, usize), _active_move: Option<&crate::battle_actions::ActiveMove>) -> EventResult {
    // JS: if (this.effectState.counter) {
    //         return this.chainModify(0.5);
    //     }

    let pokemon = match battle.pokemon_at(attacker_pos.0, attacker_pos.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };

    if let Some(counter) = pokemon.ability_state.counter {
        if counter > 0 {
            battle.chain_modify(0.5);
            return EventResult::Continue;
        }
    }

    EventResult::Continue
}

pub fn on_modify_spe(battle: &mut Battle, _spe: i32, pokemon_pos: (usize, usize)) -> EventResult {
    // JS: if (this.effectState.counter) {
    //         return this.chainModify(0.5);
    //     }

    let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };

    if let Some(counter) = pokemon.ability_state.counter {
        if counter > 0 {
            debug_elog!("[SLOW START onModifySpe] counter={}, applying 0.5x modifier", counter);
            battle.chain_modify(0.5);
            return EventResult::Continue;
        }
    }

    EventResult::Continue
}
