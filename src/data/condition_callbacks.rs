//! Condition Callback Handlers
//!
//! This module provides dispatch functions for condition callbacks.
//! Individual condition implementations will be added as needed.

use crate::battle::Battle;
use crate::event::EventResult;
use crate::ID;

// =========================================================================
// DISPATCH FUNCTIONS
//
// These functions route condition events to specific condition implementations.
// They return EventResult directly, with EventResult::Continue for no match.
// =========================================================================

/// Dispatch onAfterMove callbacks
pub fn dispatch_on_after_move(
    _battle: &mut Battle,
    _condition_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onAfterMoveSecondary callbacks
pub fn dispatch_on_after_move_secondary(
    _battle: &mut Battle,
    _condition_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onBasePower callbacks
pub fn dispatch_on_base_power(
    _battle: &mut Battle,
    _condition_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onBasePowerPriority callbacks
pub fn dispatch_on_base_power_priority(
    _battle: &mut Battle,
    _condition_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onBeforeMove callbacks
/// JavaScript source (conditions.ts):
/// par: {
///     onBeforeMovePriority: 1,
///     onBeforeMove(pokemon) {
///         if (this.randomChance(1, 4)) {
///             this.add('cant', pokemon, 'par');
///             return false;
///         }
///     },
/// }
pub fn dispatch_on_before_move(
    battle: &mut Battle,
    condition_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    use crate::battle::Arg;

    eprintln!("[CONDITION_CALLBACKS T{}] dispatch_on_before_move: condition_id={}", battle.turn, condition_id);

    match condition_id {
        "par" => {
            eprintln!("[CONDITION_CALLBACKS T{}] Handling paralysis BeforeMove", battle.turn);
            // Paralysis has 25% chance to prevent move
            if battle.random_chance(1, 4) {
                eprintln!("[CONDITION_CALLBACKS T{}] Paralysis check succeeded, preventing move", battle.turn);
                // Get pokemon identifier string (e.g. "p1a: Metang")
                let pokemon_id = {
                    let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                        Some(p) => p,
                        None => return EventResult::Continue,
                    };
                    let side_id = match pokemon_pos.0 {
                        0 => "p1",
                        1 => "p2",
                        2 => "p3",
                        _ => "p4",
                    };
                    let position_letter = match pokemon.position {
                        0 => "a",
                        1 => "b",
                        2 => "c",
                        3 => "d",
                        4 => "e",
                        _ => "f",
                    };
                    format!("{}{}: {}", side_id, position_letter, pokemon.name)
                };
                battle.add("cant", &[Arg::String(pokemon_id), Arg::Str("par")]);
                return EventResult::Boolean(false);
            }
            EventResult::Continue
        }
        "flinch" => {
            // JavaScript source (conditions.ts):
            // flinch: {
            //     name: 'flinch',
            //     duration: 1,
            //     onBeforeMovePriority: 8,
            //     onBeforeMove(pokemon) {
            //         this.add('cant', pokemon, 'flinch');
            //         this.runEvent('Flinch', pokemon);
            //         return false;
            //     },
            // },
            eprintln!("[CONDITION_CALLBACKS T{}] Handling flinch BeforeMove", battle.turn);
            // Get pokemon identifier string (e.g. "p1a: Metang")
            let pokemon_id = {
                let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                    Some(p) => p,
                    None => return EventResult::Continue,
                };
                let side_id = match pokemon_pos.0 {
                    0 => "p1",
                    1 => "p2",
                    2 => "p3",
                    _ => "p4",
                };
                let position_letter = match pokemon.position {
                    0 => "a",
                    1 => "b",
                    2 => "c",
                    3 => "d",
                    4 => "e",
                    _ => "f",
                };
                format!("{}{}: {}", side_id, position_letter, pokemon.name)
            };
            battle.add("cant", &[Arg::String(pokemon_id), Arg::Str("flinch")]);
            // TODO: Implement runEvent('Flinch', pokemon) when event system is ready
            return EventResult::Boolean(false);
        }
        _ => EventResult::Continue,
    }
}

/// Dispatch onBeforeMovePriority callbacks
pub fn dispatch_on_before_move_priority(
    _battle: &mut Battle,
    _condition_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onBeforeSwitchOut callbacks
pub fn dispatch_on_before_switch_out(
    _battle: &mut Battle,
    _condition_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onBeforeSwitchOutPriority callbacks
pub fn dispatch_on_before_switch_out_priority(
    _battle: &mut Battle,
    _condition_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onBeforeTurn callbacks
pub fn dispatch_on_before_turn(
    _battle: &mut Battle,
    _condition_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onDamagingHit callbacks
pub fn dispatch_on_damaging_hit(
    _battle: &mut Battle,
    _condition_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onDisableMove callbacks
pub fn dispatch_on_disable_move(
    _battle: &mut Battle,
    _condition_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onDragOut callbacks
pub fn dispatch_on_drag_out(
    _battle: &mut Battle,
    _condition_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onDragOutPriority callbacks
pub fn dispatch_on_drag_out_priority(
    _battle: &mut Battle,
    _condition_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onEffectiveness callbacks
pub fn dispatch_on_effectiveness(
    _battle: &mut Battle,
    _condition_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onEffectivenessPriority callbacks
pub fn dispatch_on_effectiveness_priority(
    _battle: &mut Battle,
    _condition_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onEnd callbacks
pub fn dispatch_on_end(
    _battle: &mut Battle,
    _condition_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onFieldEnd callbacks
pub fn dispatch_on_field_end(
    _battle: &mut Battle,
    _condition_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onFieldResidual callbacks
pub fn dispatch_on_field_residual(
    _battle: &mut Battle,
    _condition_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onFieldResidualOrder callbacks
pub fn dispatch_on_field_residual_order(
    _battle: &mut Battle,
    _condition_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onFieldStart callbacks
pub fn dispatch_on_field_start(
    _battle: &mut Battle,
    _condition_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onImmunity callbacks
pub fn dispatch_on_immunity(
    _battle: &mut Battle,
    _condition_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onInvulnerability callbacks
pub fn dispatch_on_invulnerability(
    _battle: &mut Battle,
    _condition_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onLockMove callbacks
pub fn dispatch_on_lock_move(
    _battle: &mut Battle,
    _condition_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onModifyDef callbacks
pub fn dispatch_on_modify_def(
    _battle: &mut Battle,
    _condition_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onModifyDefPriority callbacks
pub fn dispatch_on_modify_def_priority(
    _battle: &mut Battle,
    _condition_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onModifyMove callbacks
pub fn dispatch_on_modify_move(
    _battle: &mut Battle,
    _condition_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onModifySpD callbacks
pub fn dispatch_on_modify_sp_d(
    _battle: &mut Battle,
    _condition_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onModifySpDPriority callbacks
pub fn dispatch_on_modify_sp_d_priority(
    _battle: &mut Battle,
    _condition_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onModifySpe callbacks
pub fn dispatch_on_modify_spe(
    _battle: &mut Battle,
    _condition_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onModifySpePriority callbacks
pub fn dispatch_on_modify_spe_priority(
    _battle: &mut Battle,
    _condition_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onMoveAborted callbacks
pub fn dispatch_on_move_aborted(
    _battle: &mut Battle,
    _condition_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onResidual callbacks
pub fn dispatch_on_residual(
    battle: &mut Battle,
    condition_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    match condition_id {
        // JavaScript brn condition (data/conditions.ts):
        //   brn: {
        //     name: 'brn',
        //     effectType: 'Status',
        //     onResidualOrder: 10,
        //     onResidual(pokemon) {
        //       this.damage(pokemon.baseMaxhp / 16);
        //     },
        //   },
        "brn" => {
            // JS: this.damage(pokemon.baseMaxhp / 16);
            // Extract maxhp first to avoid borrow conflicts
            let damage_amount = if let Some(side) = battle.sides.get(pokemon_pos.0) {
                if let Some(pokemon) = side.pokemon.get(pokemon_pos.1) {
                    pokemon.maxhp / 16
                } else {
                    return EventResult::Continue;
                }
            } else {
                return EventResult::Continue;
            };

            // Now call damage with the amount
            // JS: this.damage(pokemon.baseMaxhp / 16);
            // JavaScript damage() signature: damage(damage, target=null, source=null, effect=null, instafaint=false)
            let effect_id = ID::new("brn");
            battle.damage(
                damage_amount,
                Some(pokemon_pos),
                None,  // No source
                Some(&effect_id),  // Effect is the burn status
                false,  // instafaint = false (default in JS)
            );

            EventResult::Continue
        }
        // Volatile conditions from moves (like leechseed) are handled by move_callbacks
        _ => {
            use crate::data::move_callbacks;
            move_callbacks::dispatch_condition_on_residual(battle, condition_id, pokemon_pos)
        }
    }
}

/// Dispatch onResidualOrder callbacks
pub fn dispatch_on_residual_order(
    _battle: &mut Battle,
    _condition_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onResidualPriority callbacks
pub fn dispatch_on_residual_priority(
    _battle: &mut Battle,
    _condition_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onRestart callbacks
pub fn dispatch_on_restart(
    battle: &mut Battle,
    condition_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    // JavaScript source from conditions.js:
    // onRestart() {
    //   if (this.effectState.counter < (this.effect as Condition).counterMax!) {
    //     this.effectState.counter *= 3;
    //   }
    //   this.effectState.duration = 2;
    // }

    if condition_id != "stall" {
        return EventResult::Continue;
    }

    eprintln!("[STALL_RESTART] Called for {:?}", pokemon_pos);

    // Get current counter value
    let current_counter = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        let stall_volatile = match pokemon.volatiles.get(&ID::from("stall")) {
            Some(v) => v,
            None => return EventResult::Continue,
        };

        stall_volatile.data.get("counter")
            .and_then(|v| v.as_i64())
            .map(|v| v as i32)
            .unwrap_or(1)
    };

    eprintln!("[STALL_RESTART] Current counter: {}", current_counter);

    // Update counter and duration
    let pokemon_mut = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };

    if let Some(stall_volatile) = pokemon_mut.volatiles.get_mut(&ID::from("stall")) {
        // if (this.effectState.counter < (this.effect as Condition).counterMax!) {
        //     this.effectState.counter *= 3;
        // }
        let counter_max = 729; // From JavaScript: counterMax: 729
        let new_counter = if current_counter < counter_max {
            current_counter * 3
        } else {
            current_counter
        };

        stall_volatile.data.insert("counter".to_string(), serde_json::Value::from(new_counter));
        stall_volatile.duration = Some(2);

        eprintln!("[STALL_RESTART] Updated counter to {}, duration to 2", new_counter);
    }

    EventResult::Continue
}

/// Dispatch onSourceModifyDamage callbacks
pub fn dispatch_on_source_modify_damage(
    _battle: &mut Battle,
    _condition_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onStallMove callbacks
pub fn dispatch_on_stall_move(
    battle: &mut Battle,
    condition_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    // JavaScript source from conditions.js:
    // onStallMove(pokemon) {
    //   const counter = this.effectState.counter || 1;
    //   this.debug(`Success chance: ${Math.round(100 / counter)}%`);
    //   const success = this.randomChance(1, counter);
    //   if (!success) delete pokemon.volatiles["stall"];
    //   return success;
    // }

    if condition_id != "stall" {
        return EventResult::Continue;
    }

    // Phase 1: Get counter from the volatile's effectState
    let counter = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        // Get the stall volatile
        let stall_volatile = match pokemon.volatiles.get(&ID::from("stall")) {
            Some(v) => v,
            None => return EventResult::Continue,
        };

        // Get counter from data, default to 1
        stall_volatile.data.get("counter")
            .and_then(|v| v.as_i64())
            .map(|v| v as i32)
            .unwrap_or(1)
    };

    eprintln!("[STALL_MOVE] turn={}, counter={}, Success chance: {}%", battle.turn, counter, 100 / counter);

    // Call randomChance(1, counter)
    let success = battle.random_chance(1, counter);

    eprintln!("[STALL_MOVE] turn={}, randomChance(1, {}) = {}", battle.turn, counter, success);

    // If unsuccessful, remove the stall volatile
    if !success {
        let pokemon_mut = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon_mut.volatiles.remove(&ID::from("stall"));
        eprintln!("[STALL] Removed stall volatile (failed)");
    }

    EventResult::Boolean(success)
}

/// Dispatch onStart callbacks
pub fn dispatch_on_start(
    battle: &mut Battle,
    condition_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    // JavaScript source from conditions.js:
    // onStart() {
    //   this.effectState.counter = 3;
    // }

    if condition_id != "stall" {
        return EventResult::Continue;
    }

    eprintln!("[STALL_START] Called for {:?}", pokemon_pos);

    // Set counter to 3 when stall is first added
    let pokemon_mut = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };

    if let Some(stall_volatile) = pokemon_mut.volatiles.get_mut(&ID::from("stall")) {
        stall_volatile.data.insert("counter".to_string(), serde_json::Value::from(3));
        eprintln!("[STALL_START] Set counter to 3");
    }

    EventResult::Continue
}

/// Dispatch onSwitchIn callbacks
pub fn dispatch_on_switch_in(
    _battle: &mut Battle,
    _condition_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onTrapPokemon callbacks
pub fn dispatch_on_trap_pokemon(
    _battle: &mut Battle,
    _condition_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onTrapPokemonPriority callbacks
pub fn dispatch_on_trap_pokemon_priority(
    _battle: &mut Battle,
    _condition_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onTryAddVolatile callbacks
pub fn dispatch_on_try_add_volatile(
    _battle: &mut Battle,
    _condition_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onTryMove callbacks
pub fn dispatch_on_try_move(
    _battle: &mut Battle,
    _condition_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onTryMovePriority callbacks
pub fn dispatch_on_try_move_priority(
    _battle: &mut Battle,
    _condition_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onTryPrimaryHit callbacks
pub fn dispatch_on_try_primary_hit(
    battle: &mut Battle,
    condition_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    // Route to actual implementation in move_callbacks
    use crate::data::move_callbacks;
    move_callbacks::dispatch_condition_on_try_primary_hit(battle, condition_id, pokemon_pos)
}

/// Dispatch onTryHit callbacks
pub fn dispatch_on_try_hit(
    battle: &mut Battle,
    condition_id: &str,
    source_pos: (usize, usize),
    target_pos: (usize, usize),
) -> EventResult {
    // Route to actual implementation in move_callbacks
    use crate::data::move_callbacks;
    move_callbacks::dispatch_condition_on_try_hit(battle, condition_id, source_pos, target_pos)
}

/// Dispatch onType callbacks
pub fn dispatch_on_type(
    _battle: &mut Battle,
    _condition_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onTypePriority callbacks
pub fn dispatch_on_type_priority(
    _battle: &mut Battle,
    _condition_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onWeather callbacks
pub fn dispatch_on_weather(
    _battle: &mut Battle,
    _condition_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onWeatherModifyDamage callbacks
pub fn dispatch_on_weather_modify_damage(
    battle: &mut Battle,
    condition_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    eprintln!("[WEATHER_MODIFY_DAMAGE] Called for condition: {}", condition_id);

    // Get the current move type from active_move or current_event
    let move_type = if let Some(ref active_move) = battle.active_move {
        active_move.move_type.clone()
    } else if let Some(ref event) = battle.current_event {
        // Try to get move type from event effect
        if let Some(ref effect_id) = event.effect {
            if let Some(move_data) = battle.dex.moves().get(effect_id.as_str()) {
                move_data.move_type.clone()
            } else {
                return EventResult::Continue;
            }
        } else {
            return EventResult::Continue;
        }
    } else {
        return EventResult::Continue;
    };

    eprintln!("[WEATHER_MODIFY_DAMAGE] move_type={}", move_type);

    // Get the condition definition to check type_boost and type_weaken
    if let Some(condition) = crate::data::conditions::get_condition(&ID::new(condition_id)) {
        // Check for type boost (e.g., Fire in Sunny Day)
        if let Some((boost_type, boost_mult)) = &condition.type_boost {
            if &move_type == boost_type {
                eprintln!("[WEATHER_MODIFY_DAMAGE] Applying type boost: {}x for type {}", boost_mult, boost_type);
                let result = battle.chain_modify(*boost_mult as f32);
                return EventResult::Number(result);
            }
        }

        // Check for type weaken (e.g., Water in Sunny Day)
        if let Some((weaken_type, weaken_mult)) = &condition.type_weaken {
            if &move_type == weaken_type {
                eprintln!("[WEATHER_MODIFY_DAMAGE] Applying type weaken: {}x for type {}", weaken_mult, weaken_type);
                let result = battle.chain_modify(*weaken_mult as f32);
                return EventResult::Number(result);
            }
        }
    }

    EventResult::Continue
}
