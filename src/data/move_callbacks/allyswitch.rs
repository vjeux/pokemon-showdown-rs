//! Ally Switch Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts
//!
//! ```text
//! JS Source (data/moves.ts):
//! 	allyswitch: {
//! 		num: 502,
//! 		accuracy: true,
//! 		basePower: 0,
//! 		category: "Status",
//! 		name: "Ally Switch",
//! 		pp: 15,
//! 		priority: 2,
//! 		flags: { metronome: 1 },
//! 		onPrepareHit(pokemon) {
//! 			return pokemon.addVolatile('allyswitch');
//! 		},
//! 		onHit(pokemon) {
//! 			let success = true;
//! 			// Fail in formats where you don't control allies
//! 			if (this.format.gameType !== 'doubles' && this.format.gameType !== 'triples') success = false;
//! 
//! 			// Fail in triples if the Pokemon is in the middle
//! 			if (pokemon.side.active.length === 3 && pokemon.position === 1) success = false;
//! 
//! 			const newPosition = (pokemon.position === 0 ? pokemon.side.active.length - 1 : 0);
//! 			if (!pokemon.side.active[newPosition]) success = false;
//! 			if (pokemon.side.active[newPosition].fainted) success = false;
//! 			if (!success) {
//! 				this.add('-fail', pokemon, 'move: Ally Switch');
//! 				this.attrLastMove('[still]');
//! 				return this.NOT_FAIL;
//! 			}
//! 			this.swapPosition(pokemon, newPosition, '[from] move: Ally Switch');
//! 		},
//! 		condition: {
//! 			duration: 2,
//! 			counterMax: 729,
//! 			onStart() {
//! 				this.effectState.counter = 3;
//! 			},
//! 			onRestart(pokemon) {
//! 				// this.effectState.counter should never be undefined here.
//! 				// However, just in case, use 1 if it is undefined.
//! 				const counter = this.effectState.counter || 1;
//! 				this.debug(`Ally Switch success chance: ${Math.round(100 / counter)}%`);
//! 				const success = this.randomChance(1, counter);
//! 				if (!success) {
//! 					delete pokemon.volatiles['allyswitch'];
//! 					return false;
//! 				}
//! 				if (this.effectState.counter < (this.effect as Condition).counterMax!) {
//! 					this.effectState.counter *= 3;
//! 				}
//! 				this.effectState.duration = 2;
//! 			},
//! 		},
//! 		secondary: null,
//! 		target: "self",
//! 		type: "Psychic",
//! 		zMove: { boost: { spe: 2 } },
//! 		contestType: "Clever",
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{MoveHandlerResult, Status, Effect};

/// onPrepareHit callback for Ally Switch
/// Adds allyswitch volatile to the user
pub fn on_prepare_hit(
    battle: &mut Battle,
    pokemon: (usize, usize),
    _source: (usize, usize),
    _move_id: &ID,
) -> MoveHandlerResult {
    // JavaScript: return pokemon.addVolatile('allyswitch');
    let result = battle.add_volatile_to_pokemon(
        pokemon,
        &ID::new("allyswitch"),
        Some(pokemon),
        None,
    );

    if result {
        MoveHandlerResult::True
    } else {
        MoveHandlerResult::False
    }
}

/// onHit callback for Ally Switch
/// Swaps position with ally pokemon
pub fn on_hit(
    battle: &mut Battle,
    pokemon: (usize, usize),
    _source: (usize, usize),
    _move_id: &ID,
) -> MoveHandlerResult {
    // JavaScript implementation from lines 22-38
    let mut success = true;

    // Fail in formats where you don't control allies
    if battle.game_type != crate::dex_data::GameType::Doubles &&
       battle.game_type != crate::dex_data::GameType::Triples {
        success = false;
    }

    let (side_idx, poke_idx) = pokemon;

    // Get active length and position
    let (active_length, position) = if let Some(side) = battle.sides.get(side_idx) {
        if let Some(poke) = side.pokemon.get(poke_idx) {
            (side.active.len(), poke.position)
        } else {
            (0, 0)
        }
    } else {
        return MoveHandlerResult::False;
    };

    // Fail in triples if the Pokemon is in the middle
    if active_length == 3 && position == 1 {
        success = false;
    }

    // Calculate new position
    let new_position = if position == 0 {
        active_length - 1
    } else {
        0
    };

    // Check if new position is valid
    if let Some(side) = battle.sides.get(side_idx) {
        if new_position >= side.active.len() {
            success = false;
        } else if let Some(target) = side.active.get(new_position) {
            if let Some(target_poke) = target {
                if let Some(actual_target) = side.pokemon.get(*target_poke) {
                    if actual_target.fainted {
                        success = false;
                    }
                } else {
                    success = false;
                }
            } else {
                success = false;
            }
        } else {
            success = false;
        }
    } else {
        success = false;
    }

    if !success {
        // Get pokemon identifier for log
        let pokemon_id = if let Some(side) = battle.sides.get(side_idx) {
            if let Some(poke) = side.pokemon.get(poke_idx) {
                format!("{}: {}", side.id_str(), poke.name)
            } else {
                String::from("unknown")
            }
        } else {
            String::from("unknown")
        };

        battle.add_log("-fail", &[&pokemon_id, "move: Ally Switch"]);
        battle.attr_last_move(&["[still]"]);
        return MoveHandlerResult::False; // NOT_FAIL in JS, but False is close enough
    }

    // Swap position
    battle.swap_position(pokemon, new_position, Some("[from] move: Ally Switch"));

    MoveHandlerResult::Undefined
}

/// onStart - Condition callback (not used for move itself)
/// This would be called when the allyswitch volatile is added
/// For now, this is a placeholder as condition callbacks need separate infrastructure
pub fn on_start(battle: &mut Battle, target: (usize, usize)) -> MoveHandlerResult {
    // JavaScript: this.effectState.counter = 3;
    // This needs to be handled in the condition system
    // TODO: Set effectState.counter = 3 when volatile is added
    let (side_idx, poke_idx) = target;
    if let Some(side) = battle.sides.get_mut(side_idx) {
        if let Some(pokemon) = side.pokemon.get_mut(poke_idx) {
            if let Some(volatile) = pokemon.volatiles.get_mut(&ID::new("allyswitch")) {
                volatile.data.insert("counter".to_string(), serde_json::json!(3));
                volatile.duration = Some(2);
            }
        }
    }
    MoveHandlerResult::Undefined
}

/// onRestart - Condition callback
/// Called when trying to add allyswitch volatile when it already exists
pub fn on_restart(battle: &mut Battle, target: (usize, usize)) -> MoveHandlerResult {
    // JavaScript implementation from lines 45-59
    let (side_idx, poke_idx) = target;

    // Get counter from effectState
    let counter = if let Some(side) = battle.sides.get(side_idx) {
        if let Some(pokemon) = side.pokemon.get(poke_idx) {
            if let Some(volatile) = pokemon.volatiles.get(&ID::new("allyswitch")) {
                if let Some(counter_val) = volatile.data.get("counter") {
                    counter_val.as_u64().unwrap_or(1) as i32
                } else {
                    1
                }
            } else {
                return MoveHandlerResult::False;
            }
        } else {
            return MoveHandlerResult::False;
        }
    } else {
        return MoveHandlerResult::False;
    };

    // Debug message
    let chance_pct = (100.0 / counter as f64).round() as i32;
    battle.debug(&format!("Ally Switch success chance: {}%", chance_pct));

    // Random chance check
    let success = battle.random_chance(1, counter);

    if !success {
        // Remove volatile
        if let Some(side) = battle.sides.get_mut(side_idx) {
            if let Some(pokemon) = side.pokemon.get_mut(poke_idx) {
                pokemon.volatiles.remove(&ID::new("allyswitch"));
            }
        }
        return MoveHandlerResult::False;
    }

    // Update counter (multiply by 3, max 729)
    let new_counter = if counter < 729 {
        counter * 3
    } else {
        counter
    };

    // Update effectState
    if let Some(side) = battle.sides.get_mut(side_idx) {
        if let Some(pokemon) = side.pokemon.get_mut(poke_idx) {
            if let Some(volatile) = pokemon.volatiles.get_mut(&ID::new("allyswitch")) {
                volatile.data.insert("counter".to_string(), serde_json::json!(new_counter));
                volatile.duration = Some(2);
            }
        }
    }

    MoveHandlerResult::True
}


// Condition handlers
pub mod condition {
    use super::*;

    // TODO: Implement condition handlers
}
