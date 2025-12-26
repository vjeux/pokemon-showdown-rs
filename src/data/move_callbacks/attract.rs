//! Attract Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts
//!
//! ```text
//! JS Source (data/moves.ts):
//! 	attract: {
//! 		num: 213,
//! 		accuracy: 100,
//! 		basePower: 0,
//! 		category: "Status",
//! 		name: "Attract",
//! 		pp: 15,
//! 		priority: 0,
//! 		flags: { protect: 1, reflectable: 1, mirror: 1, bypasssub: 1, metronome: 1 },
//! 		volatileStatus: 'attract',
//! 		condition: {
//! 			noCopy: true,
//! 			onStart(pokemon, source, effect) {
//! 				if (!(pokemon.gender === 'M' && source.gender === 'F') && !(pokemon.gender === 'F' && source.gender === 'M')) {
//! 					this.debug('incompatible gender');
//! 					return false;
//! 				}
//! 				if (!this.runEvent('Attract', pokemon, source)) {
//! 					this.debug('Attract event failed');
//! 					return false;
//! 				}
//!
//! 				if (effect.name === 'Cute Charm') {
//! 					this.add('-start', pokemon, 'Attract', '[from] ability: Cute Charm', `[of] ${source}`);
//! 				} else if (effect.name === 'Destiny Knot') {
//! 					this.add('-start', pokemon, 'Attract', '[from] item: Destiny Knot', `[of] ${source}`);
//! 				} else {
//! 					this.add('-start', pokemon, 'Attract');
//! 				}
//! 			},
//! 			onUpdate(pokemon) {
//! 				if (this.effectState.source && !this.effectState.source.isActive && pokemon.volatiles['attract']) {
//! 					this.debug(`Removing Attract volatile on ${pokemon}`);
//! 					pokemon.removeVolatile('attract');
//! 				}
//! 			},
//! 			onBeforeMovePriority: 2,
//! 			onBeforeMove(pokemon, target, move) {
//! 				this.add('-activate', pokemon, 'move: Attract', '[of] ' + this.effectState.source);
//! 				if (this.randomChance(1, 2)) {
//! 					this.add('cant', pokemon, 'Attract');
//! 					return false;
//! 				}
//! 			},
//! 			onEnd(pokemon) {
//! 				this.add('-end', pokemon, 'Attract', '[silent]');
//! 			},
//! 		},
//! 		onTryImmunity(target, source) {
//! 			return (target.gender === 'M' && source.gender === 'F') || (target.gender === 'F' && source.gender === 'M');
//! 		},
//! 		secondary: null,
//! 		target: "normal",
//! 		type: "Normal",
//! 		zMove: { effect: 'clearnegativeboost' },
//! 		contestType: "Cute",
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{MoveHandlerResult, Status, Effect};

/// onStart - Condition callback
/// Called when attract volatile is added
pub fn on_start(battle: &mut Battle, target: (usize, usize)) -> MoveHandlerResult {
    // JavaScript: if (!(pokemon.gender === 'M' && source.gender === 'F') && !(pokemon.gender === 'F' && source.gender === 'M')) { ... }
    let (target_side, target_idx) = target;

    // Get source from volatile's effectState
    let source = if let Some(side) = battle.sides.get(target_side) {
        if let Some(pokemon) = side.pokemon.get(target_idx) {
            if let Some(volatile) = pokemon.volatiles.get(&ID::new("attract")) {
                if let Some(source_slot) = &volatile.source_slot {
                    // Parse source_slot "side:idx"
                    let parts: Vec<&str> = source_slot.split(':').collect();
                    if parts.len() == 2 {
                        if let (Ok(s), Ok(i)) = (parts[0].parse::<usize>(), parts[1].parse::<usize>()) {
                            Some((s, i))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    None
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

    let source = if let Some(s) = source {
        s
    } else {
        return MoveHandlerResult::False;
    };

    // Check gender compatibility
    let (target_gender, source_gender) = if let (Some(target_side_ref), Some(source_side_ref)) =
        (battle.sides.get(target_side), battle.sides.get(source.0)) {
        if let (Some(target_poke), Some(source_poke)) =
            (target_side_ref.pokemon.get(target_idx), source_side_ref.pokemon.get(source.1)) {
            (target_poke.gender.to_str(), source_poke.gender.to_str())
        } else {
            return MoveHandlerResult::False;
        }
    } else {
        return MoveHandlerResult::False;
    };

    // JavaScript: if (!(pokemon.gender === 'M' && source.gender === 'F') && !(pokemon.gender === 'F' && source.gender === 'M'))
    let compatible = (target_gender == "M" && source_gender == "F") || (target_gender == "F" && source_gender == "M");

    if !compatible {
        battle.debug("incompatible gender");
        return MoveHandlerResult::False;
    }

    // JavaScript: if (!this.runEvent('Attract', pokemon, source)) { ... }
    // TODO: Run Attract event - for now skip

    // JavaScript: this.add('-start', pokemon, 'Attract');
    // Get effect name from volatile data to check for Cute Charm / Destiny Knot
    let effect_name = if let Some(side) = battle.sides.get(target_side) {
        if let Some(pokemon) = side.pokemon.get(target_idx) {
            if let Some(volatile) = pokemon.volatiles.get(&ID::new("attract")) {
                if let Some(eff) = volatile.data.get("sourceEffect") {
                    eff.as_str().map(|s| s.to_string())
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    };

    let pokemon_id = if let Some(side) = battle.sides.get(target_side) {
        if let Some(pokemon) = side.pokemon.get(target_idx) {
            format!("{}: {}", side.id_str(), pokemon.name)
        } else {
            String::from("unknown")
        }
    } else {
        String::from("unknown")
    };

    // JavaScript: if (effect.name === 'Cute Charm') { ... } else if (effect.name === 'Destiny Knot') { ... } else { ... }
    if let Some(eff) = effect_name {
        if eff == "cutecharm" {
            let source_id = if let Some(side) = battle.sides.get(source.0) {
                if let Some(pokemon) = side.pokemon.get(source.1) {
                    format!("{}: {}", side.id_str(), pokemon.name)
                } else {
                    String::from("unknown")
                }
            } else {
                String::from("unknown")
            };
            battle.add_log("-start", &[&pokemon_id, "Attract", "[from] ability: Cute Charm", &format!("[of] {}", source_id)]);
        } else if eff == "destinyknot" {
            let source_id = if let Some(side) = battle.sides.get(source.0) {
                if let Some(pokemon) = side.pokemon.get(source.1) {
                    format!("{}: {}", side.id_str(), pokemon.name)
                } else {
                    String::from("unknown")
                }
            } else {
                String::from("unknown")
            };
            battle.add_log("-start", &[&pokemon_id, "Attract", "[from] item: Destiny Knot", &format!("[of] {}", source_id)]);
        } else {
            battle.add_log("-start", &[&pokemon_id, "Attract"]);
        }
    } else {
        battle.add_log("-start", &[&pokemon_id, "Attract"]);
    }

    MoveHandlerResult::True
}

/// onUpdate - Condition callback
/// Called each turn to check if source is still active
pub fn on_update(battle: &mut Battle, target: (usize, usize)) -> MoveHandlerResult {
    // JavaScript: if (this.effectState.source && !this.effectState.source.isActive && pokemon.volatiles['attract']) { ... }
    let (side_idx, poke_idx) = target;

    // Get source from volatile
    let source_active = if let Some(side) = battle.sides.get(side_idx) {
        if let Some(pokemon) = side.pokemon.get(poke_idx) {
            if let Some(volatile) = pokemon.volatiles.get(&ID::new("attract")) {
                if let Some(source_slot) = &volatile.source_slot {
                    let parts: Vec<&str> = source_slot.split(':').collect();
                    if parts.len() == 2 {
                        if let (Ok(s), Ok(i)) = (parts[0].parse::<usize>(), parts[1].parse::<usize>()) {
                            // Check if source is still active
                            if let Some(source_side) = battle.sides.get(s) {
                                if let Some(source_poke) = source_side.pokemon.get(i) {
                                    source_poke.is_active
                                } else {
                                    false
                                }
                            } else {
                                false
                            }
                        } else {
                            true // Can't parse, assume active
                        }
                    } else {
                        true
                    }
                } else {
                    true
                }
            } else {
                return MoveHandlerResult::Undefined;
            }
        } else {
            return MoveHandlerResult::Undefined;
        }
    } else {
        return MoveHandlerResult::Undefined;
    };

    if !source_active {
        battle.debug(&format!("Removing Attract volatile on {}:{}", side_idx, poke_idx));
        if let Some(side) = battle.sides.get_mut(side_idx) {
            if let Some(pokemon) = side.pokemon.get_mut(poke_idx) {
                pokemon.volatiles.remove(&ID::new("attract"));
            }
        }
    }

    MoveHandlerResult::Undefined
}

/// onBeforeMovePriority - Returns priority for onBeforeMove
pub fn on_before_move_priority(_battle: &mut Battle, _target: (usize, usize)) -> MoveHandlerResult {
    // JavaScript: onBeforeMovePriority: 2
    MoveHandlerResult::Number(2)
}

/// onBeforeMove - Condition callback
/// Called before the Pokemon uses a move, may prevent it
pub fn on_before_move(battle: &mut Battle, target: (usize, usize)) -> MoveHandlerResult {
    // JavaScript: this.add('-activate', pokemon, 'move: Attract', '[of] ' + this.effectState.source);
    let (side_idx, poke_idx) = target;

    let (pokemon_id, source_id) = if let Some(side) = battle.sides.get(side_idx) {
        if let Some(pokemon) = side.pokemon.get(poke_idx) {
            let pid = format!("{}: {}", side.id_str(), pokemon.name);

            // Get source
            if let Some(volatile) = pokemon.volatiles.get(&ID::new("attract")) {
                if let Some(source_slot) = &volatile.source_slot {
                    let parts: Vec<&str> = source_slot.split(':').collect();
                    if parts.len() == 2 {
                        if let (Ok(s), Ok(i)) = (parts[0].parse::<usize>(), parts[1].parse::<usize>()) {
                            if let Some(source_side) = battle.sides.get(s) {
                                if let Some(source_poke) = source_side.pokemon.get(i) {
                                    let sid = format!("{}: {}", source_side.id_str(), source_poke.name);
                                    (pid, sid)
                                } else {
                                    (pid, String::from("unknown"))
                                }
                            } else {
                                (pid, String::from("unknown"))
                            }
                        } else {
                            (pid, String::from("unknown"))
                        }
                    } else {
                        (pid, String::from("unknown"))
                    }
                } else {
                    (pid, String::from("unknown"))
                }
            } else {
                (pid, String::from("unknown"))
            }
        } else {
            (String::from("unknown"), String::from("unknown"))
        }
    } else {
        (String::from("unknown"), String::from("unknown"))
    };

    battle.add_log("-activate", &[&pokemon_id, "move: Attract", &format!("[of] {}", source_id)]);

    // JavaScript: if (this.randomChance(1, 2)) { this.add('cant', pokemon, 'Attract'); return false; }
    if battle.random_chance(1, 2) {
        battle.add_log("cant", &[&pokemon_id, "Attract"]);
        return MoveHandlerResult::False;
    }

    MoveHandlerResult::Undefined
}

/// onEnd - Condition callback
/// Called when attract volatile ends
pub fn on_end(battle: &mut Battle, target: (usize, usize)) -> MoveHandlerResult {
    // JavaScript: this.add('-end', pokemon, 'Attract', '[silent]');
    let pokemon_id = if let Some(side) = battle.sides.get(target.0) {
        if let Some(pokemon) = side.pokemon.get(target.1) {
            format!("{}: {}", side.id_str(), pokemon.name)
        } else {
            String::from("unknown")
        }
    } else {
        String::from("unknown")
    };

    battle.add_log("-end", &[&pokemon_id, "Attract", "[silent]"]);

    MoveHandlerResult::Undefined
}

/// onTryImmunity - Move callback
/// Checks if the target can be affected by attract
pub fn on_try_immunity(
    battle: &mut Battle,
    target: (usize, usize),
    source: (usize, usize),
    _move_id: &ID,
) -> MoveHandlerResult {
    // JavaScript: return (target.gender === 'M' && source.gender === 'F') || (target.gender === 'F' && source.gender === 'M');
    let (target_gender, source_gender) = if let (Some(target_side), Some(source_side)) =
        (battle.sides.get(target.0), battle.sides.get(source.0)) {
        if let (Some(target_poke), Some(source_poke)) =
            (target_side.pokemon.get(target.1), source_side.pokemon.get(source.1)) {
            (target_poke.gender.to_str(), source_poke.gender.to_str())
        } else {
            return MoveHandlerResult::False;
        }
    } else {
        return MoveHandlerResult::False;
    };

    let compatible = (target_gender == "M" && source_gender == "F") || (target_gender == "F" && source_gender == "M");

    if compatible {
        MoveHandlerResult::True
    } else {
        MoveHandlerResult::False
    }
}
