//! Gorilla Tactics Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onStart(pokemon) {
///     pokemon.abilityState.choiceLock = "";
/// }
pub fn on_start(battle: &mut Battle, pokemon_pos: (usize, usize), _source_pos: Option<(usize, usize)>, _effect_id: Option<&str>) -> EventResult {
    // pokemon.abilityState.choiceLock = "";
    let pokemon_mut = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };
    pokemon_mut.ability_state.data.insert("choiceLock".to_string(), serde_json::json!(""));

    EventResult::Continue
}

/// onBeforeMove(pokemon, target, move) {
///     if (move.isZOrMaxPowered || move.id === 'struggle') return;
///     if (pokemon.abilityState.choiceLock && pokemon.abilityState.choiceLock !== move.id) {
///         // Fails unless ability is being ignored (these events will not run), no PP lost.
///         this.addMove('move', pokemon, move.name);
///         this.attrLastMove('[still]');
///         this.debug("Disabled by Gorilla Tactics");
///         this.add('-fail', pokemon);
///         return false;
///     }
/// }
pub fn on_before_move(battle: &mut Battle, pokemon_pos: (usize, usize), _target_pos: Option<(usize, usize)>, move_id: &str) -> EventResult {
    use crate::battle::Arg;

    // if (move.isZOrMaxPowered || move.id === 'struggle') return;
    let is_z_or_max_or_struggle = {
        if move_id == "struggle" {
            true
        } else if let Some(ref active_move) = battle.active_move {
            active_move.is_z || active_move.is_max
        } else {
            false
        }
    };

    if is_z_or_max_or_struggle {
        return EventResult::Continue;
    }

    // if (pokemon.abilityState.choiceLock && pokemon.abilityState.choiceLock !== move.id)
    let choice_lock = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.ability_state.data.get("choiceLock")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
    };

    if let Some(ref lock) = choice_lock {
        if !lock.is_empty() && lock != move_id {
            // this.addMove('move', pokemon, move.name);
            let move_name = {
                if let Some(move_data) = battle.dex.moves().get(move_id) {
                    move_data.name.clone()
                } else {
                    move_id.to_string()
                }
            };

            let pokemon_id = {
                let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                    Some(p) => p,
                    None => return EventResult::Continue,
                };
                pokemon.get_slot()
            };

            battle.add_move(&["move", &pokemon_id, &move_name]);

            // this.attrLastMove('[still]');
            battle.attr_last_move(&["[still]"]);

            // this.debug("Disabled by Gorilla Tactics");
            // (debug is for internal logging, skip in Rust for now)

            // this.add('-fail', pokemon);
            battle.add("-fail", &[Arg::String(pokemon_id)]);

            // return false;
            return EventResult::Boolean(false);
        }
    }

    EventResult::Continue
}

/// onModifyMove(move, pokemon) {
///     if (pokemon.abilityState.choiceLock || move.isZOrMaxPowered || move.id === 'struggle') return;
///     pokemon.abilityState.choiceLock = move.id;
/// }
pub fn on_modify_move(battle: &mut Battle, move_id: &str, pokemon_pos: (usize, usize), _target_pos: Option<(usize, usize)>) -> EventResult {
    // if (pokemon.abilityState.choiceLock || move.isZOrMaxPowered || move.id === 'struggle') return;
    let choice_lock = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.ability_state.data.get("choiceLock")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
    };

    // If already locked, return
    if let Some(ref lock) = choice_lock {
        if !lock.is_empty() {
            return EventResult::Continue;
        }
    }

    // If Z/Max move or Struggle, return
    let is_z_or_max_or_struggle = {
        if move_id == "struggle" {
            true
        } else if let Some(ref active_move) = battle.active_move {
            active_move.is_z || active_move.is_max
        } else {
            false
        }
    };

    if is_z_or_max_or_struggle {
        return EventResult::Continue;
    }

    // pokemon.abilityState.choiceLock = move.id;
    let pokemon_mut = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };
    pokemon_mut.ability_state.data.insert("choiceLock".to_string(), serde_json::json!(move_id));

    EventResult::Continue
}

/// onModifyAtk(atk, pokemon) {
///     if (pokemon.volatiles['dynamax']) return;
///     // PLACEHOLDER
///     this.debug('Gorilla Tactics Atk Boost');
///     return this.chainModify(1.5);
/// }
pub fn on_modify_atk(battle: &mut Battle, _atk: i32, attacker_pos: (usize, usize), _defender_pos: (usize, usize), _move_id: &str) -> EventResult {
    use crate::dex_data::ID;

    // if (pokemon.volatiles['dynamax']) return;
    let has_dynamax = {
        let pokemon = match battle.pokemon_at(attacker_pos.0, attacker_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.volatiles.contains_key(&ID::from("dynamax"))
    };

    if has_dynamax {
        return EventResult::Continue;
    }

    // this.debug('Gorilla Tactics Atk Boost');
    // return this.chainModify(1.5);
    { battle.chain_modify(1.5); EventResult::Continue }
}

/// onDisableMove(pokemon) {
///     if (!pokemon.abilityState.choiceLock) return;
///     if (pokemon.volatiles['dynamax']) return;
///     for (const moveSlot of pokemon.moveSlots) {
///         if (moveSlot.id !== pokemon.abilityState.choiceLock) {
///             pokemon.disableMove(moveSlot.id, false, this.effectState.sourceEffect);
///         }
///     }
/// }
pub fn on_disable_move(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    use crate::dex_data::ID;

    // if (!pokemon.abilityState.choiceLock) return;
    let choice_lock = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.ability_state.data.get("choiceLock")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
    };

    let lock = match choice_lock {
        Some(ref l) if !l.is_empty() => l.clone(),
        _ => return EventResult::Continue,
    };

    // if (pokemon.volatiles['dynamax']) return;
    let has_dynamax = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.volatiles.contains_key(&ID::from("dynamax"))
    };

    if has_dynamax {
        return EventResult::Continue;
    }

    // for (const moveSlot of pokemon.moveSlots) {
    //     if (moveSlot.id !== pokemon.abilityState.choiceLock) {
    //         pokemon.disableMove(moveSlot.id, false, this.effectState.sourceEffect);
    //     }
    // }
    let move_ids: Vec<String> = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.move_slots.iter()
            .filter(|slot| slot.id.as_str() != lock)
            .map(|slot| slot.id.as_str().to_string())
            .collect()
    };

    for move_id in move_ids {
        let pokemon_mut = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => continue,
        };
        pokemon_mut.disable_move(&move_id, false, None);
    }

    EventResult::Continue
}

/// onEnd(pokemon) {
///     pokemon.abilityState.choiceLock = "";
/// }
pub fn on_end(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // pokemon.abilityState.choiceLock = "";
    let pokemon_mut = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };
    pokemon_mut.ability_state.data.insert("choiceLock".to_string(), serde_json::json!(""));

    EventResult::Continue
}

