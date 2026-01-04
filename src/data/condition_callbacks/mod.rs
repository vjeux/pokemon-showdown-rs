//! Condition Callback Handlers
//!
//! This module provides dispatch functions for condition callbacks.
//! Individual condition implementations are in separate files.

use crate::battle::Battle;
use crate::event::EventResult;

// Individual condition modules
pub mod arceus;
pub mod brn;
pub mod choicelock;
pub mod commanded;
pub mod commanding;
pub mod confusion;
pub mod deltastream;
pub mod desolateland;
pub mod dynamax;
pub mod flinch;
pub mod frz;
pub mod futuremove;
pub mod gem;
pub mod hail;
pub mod healreplacement;
pub mod lockedmove;
pub mod mustrecharge;
pub mod par;
pub mod partiallytrapped;
pub mod primordialsea;
pub mod psn;
pub mod raindance;
pub mod rolloutstorage;
pub mod sandstorm;
pub mod silvally;
pub mod slp;
pub mod snowscape;
pub mod stall;
pub mod sunnyday;
pub mod tox;
pub mod trapped;
pub mod twoturnmove;

// =========================================================================
// DISPATCH FUNCTIONS
//
// These functions route condition events to specific condition implementations.
// They return EventResult directly, with EventResult::Continue for no match.
// =========================================================================

/// Dispatch durationCallback callbacks
pub fn dispatch_duration_callback(
    battle: &mut Battle,
    condition_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    match condition_id {
        "hail" => hail::duration_callback(battle, pokemon_pos),
        "partiallytrapped" => partiallytrapped::duration_callback(battle, pokemon_pos),
        "raindance" => raindance::duration_callback(battle, pokemon_pos),
        "sandstorm" => sandstorm::duration_callback(battle, pokemon_pos),
        "snowscape" => snowscape::duration_callback(battle, pokemon_pos),
        "sunnyday" => sunnyday::duration_callback(battle, pokemon_pos),
        _ => EventResult::Continue,
    }
}

/// Dispatch onAfterMove callbacks
pub fn dispatch_on_after_move(
    battle: &mut Battle,
    condition_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    match condition_id {
        "lockedmove" => lockedmove::on_after_move(battle, pokemon_pos),
        _ => EventResult::Continue,
    }
}

/// Dispatch onAfterMoveSecondary callbacks
pub fn dispatch_on_after_move_secondary(
    battle: &mut Battle,
    condition_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    match condition_id {
        "frz" => frz::on_after_move_secondary(battle, pokemon_pos),
        _ => EventResult::Continue,
    }
}

/// Dispatch onBasePower callbacks
pub fn dispatch_on_base_power(
    battle: &mut Battle,
    condition_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    match condition_id {
        "gem" => gem::on_base_power(battle, pokemon_pos),
        "rolloutstorage" => rolloutstorage::on_base_power(battle, pokemon_pos),
        _ => EventResult::Continue,
    }
}

/// Dispatch onBeforeMove callbacks
pub fn dispatch_on_before_move(
    battle: &mut Battle,
    condition_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    match condition_id {
        "choicelock" => choicelock::on_before_move(battle, pokemon_pos),
        "confusion" => confusion::on_before_move(battle, pokemon_pos),
        "flinch" => flinch::on_before_move(battle, pokemon_pos),
        "frz" => frz::on_before_move(battle, pokemon_pos),
        "mustrecharge" => mustrecharge::on_before_move(battle, pokemon_pos),
        "par" => par::on_before_move(battle, pokemon_pos),
        "slp" => slp::on_before_move(battle, pokemon_pos),
        _ => EventResult::Continue,
    }
}

/// Dispatch onBeforeSwitchOut callbacks
pub fn dispatch_on_before_switch_out(
    battle: &mut Battle,
    condition_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    match condition_id {
        "dynamax" => dynamax::on_before_switch_out(battle, pokemon_pos),
        _ => EventResult::Continue,
    }
}

/// Dispatch onBeforeTurn callbacks
pub fn dispatch_on_before_turn(
    battle: &mut Battle,
    condition_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    match condition_id {
        "commanding" => commanding::on_before_turn(battle, pokemon_pos),
        _ => EventResult::Continue,
    }
}

/// Dispatch onDamagingHit callbacks
pub fn dispatch_on_damaging_hit(
    battle: &mut Battle,
    condition_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    match condition_id {
        "frz" => frz::on_damaging_hit(battle, pokemon_pos),
        _ => EventResult::Continue,
    }
}

/// Dispatch onDisableMove callbacks
pub fn dispatch_on_disable_move(
    battle: &mut Battle,
    condition_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    match condition_id {
        "choicelock" => choicelock::on_disable_move(battle, pokemon_pos),
        _ => EventResult::Continue,
    }
}

/// Dispatch onDragOut callbacks
pub fn dispatch_on_drag_out(
    battle: &mut Battle,
    condition_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    match condition_id {
        "commanded" => commanded::on_drag_out(battle, pokemon_pos),
        "commanding" => commanding::on_drag_out(battle, pokemon_pos),
        "dynamax" => dynamax::on_drag_out(battle, pokemon_pos),
        _ => EventResult::Continue,
    }
}

/// Dispatch onEffectiveness callbacks
pub fn dispatch_on_effectiveness(
    battle: &mut Battle,
    condition_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    match condition_id {
        "deltastream" => deltastream::on_effectiveness(battle, pokemon_pos),
        _ => EventResult::Continue,
    }
}

/// Dispatch onEnd callbacks
pub fn dispatch_on_end(
    battle: &mut Battle,
    condition_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    match condition_id {
        "confusion" => confusion::on_end(battle, pokemon_pos),
        "dynamax" => dynamax::on_end(battle, pokemon_pos),
        "futuremove" => futuremove::on_end(battle, pokemon_pos),
        "lockedmove" => lockedmove::on_end(battle, pokemon_pos),
        "partiallytrapped" => partiallytrapped::on_end(battle, pokemon_pos),
        "twoturnmove" => twoturnmove::on_end(battle, pokemon_pos),
        _ => EventResult::Continue,
    }
}

/// Dispatch onFieldEnd callbacks
pub fn dispatch_on_field_end(
    battle: &mut Battle,
    condition_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    match condition_id {
        "deltastream" => deltastream::on_field_end(battle, pokemon_pos),
        "desolateland" => desolateland::on_field_end(battle, pokemon_pos),
        "hail" => hail::on_field_end(battle, pokemon_pos),
        "primordialsea" => primordialsea::on_field_end(battle, pokemon_pos),
        "raindance" => raindance::on_field_end(battle, pokemon_pos),
        "sandstorm" => sandstorm::on_field_end(battle, pokemon_pos),
        "snowscape" => snowscape::on_field_end(battle, pokemon_pos),
        "sunnyday" => sunnyday::on_field_end(battle, pokemon_pos),
        _ => EventResult::Continue,
    }
}

/// Dispatch onFieldResidual callbacks
pub fn dispatch_on_field_residual(
    battle: &mut Battle,
    condition_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    match condition_id {
        "deltastream" => deltastream::on_field_residual(battle, pokemon_pos),
        "desolateland" => desolateland::on_field_residual(battle, pokemon_pos),
        "hail" => hail::on_field_residual(battle, pokemon_pos),
        "primordialsea" => primordialsea::on_field_residual(battle, pokemon_pos),
        "raindance" => raindance::on_field_residual(battle, pokemon_pos),
        "sandstorm" => sandstorm::on_field_residual(battle, pokemon_pos),
        "snowscape" => snowscape::on_field_residual(battle, pokemon_pos),
        "sunnyday" => sunnyday::on_field_residual(battle, pokemon_pos),
        _ => EventResult::Continue,
    }
}

/// Dispatch onFieldStart callbacks
pub fn dispatch_on_field_start(
    battle: &mut Battle,
    condition_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    match condition_id {
        "deltastream" => deltastream::on_field_start(battle, pokemon_pos),
        "desolateland" => desolateland::on_field_start(battle, pokemon_pos),
        "hail" => hail::on_field_start(battle, pokemon_pos),
        "primordialsea" => primordialsea::on_field_start(battle, pokemon_pos),
        "raindance" => raindance::on_field_start(battle, pokemon_pos),
        "sandstorm" => sandstorm::on_field_start(battle, pokemon_pos),
        "snowscape" => snowscape::on_field_start(battle, pokemon_pos),
        "sunnyday" => sunnyday::on_field_start(battle, pokemon_pos),
        _ => EventResult::Continue,
    }
}

/// Dispatch onImmunity callbacks
pub fn dispatch_on_immunity(
    battle: &mut Battle,
    condition_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    match condition_id {
        "desolateland" => desolateland::on_immunity(battle, pokemon_pos),
        "sunnyday" => sunnyday::on_immunity(battle, pokemon_pos),
        _ => EventResult::Continue,
    }
}

/// Dispatch onLockMove callbacks
pub fn dispatch_on_lock_move(
    battle: &mut Battle,
    condition_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    match condition_id {
        "lockedmove" => lockedmove::on_lock_move(battle, pokemon_pos),
        "twoturnmove" => twoturnmove::on_lock_move(battle, pokemon_pos),
        _ => EventResult::Continue,
    }
}

/// Dispatch onModifyDef callbacks
pub fn dispatch_on_modify_def(
    battle: &mut Battle,
    condition_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    match condition_id {
        "snowscape" => snowscape::on_modify_def(battle, pokemon_pos),
        _ => EventResult::Continue,
    }
}

/// Dispatch onModifyMove callbacks
pub fn dispatch_on_modify_move(
    battle: &mut Battle,
    condition_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    match condition_id {
        "frz" => frz::on_modify_move(battle, pokemon_pos),
        _ => EventResult::Continue,
    }
}

/// Dispatch onModifySpD callbacks
pub fn dispatch_on_modify_sp_d(
    battle: &mut Battle,
    condition_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    match condition_id {
        "sandstorm" => sandstorm::on_modify_sp_d(battle, pokemon_pos),
        _ => EventResult::Continue,
    }
}

/// Dispatch onModifySpe callbacks
pub fn dispatch_on_modify_spe(
    battle: &mut Battle,
    condition_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    match condition_id {
        "par" => par::on_modify_spe(battle, pokemon_pos),
        _ => EventResult::Continue,
    }
}

/// Dispatch onMoveAborted callbacks
pub fn dispatch_on_move_aborted(
    battle: &mut Battle,
    condition_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    match condition_id {
        "twoturnmove" => twoturnmove::on_move_aborted(battle, pokemon_pos),
        _ => EventResult::Continue,
    }
}

/// Dispatch onResidual callbacks
pub fn dispatch_on_residual(
    battle: &mut Battle,
    condition_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    match condition_id {
        "brn" => brn::on_residual(battle, pokemon_pos),
        "dynamax" => dynamax::on_residual(battle, pokemon_pos),
        "futuremove" => futuremove::on_residual(battle, pokemon_pos),
        "lockedmove" => lockedmove::on_residual(battle, pokemon_pos),
        "partiallytrapped" => partiallytrapped::on_residual(battle, pokemon_pos),
        "psn" => psn::on_residual(battle, pokemon_pos),
        "tox" => tox::on_residual(battle, pokemon_pos),
        _ => EventResult::Continue,
    }
}

/// Dispatch onRestart callbacks
pub fn dispatch_on_restart(
    battle: &mut Battle,
    condition_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    match condition_id {
        "lockedmove" => lockedmove::on_restart(battle, pokemon_pos),
        "stall" => stall::on_restart(battle, pokemon_pos),
        _ => EventResult::Continue,
    }
}

/// Dispatch onSourceModifyDamage callbacks
pub fn dispatch_on_source_modify_damage(
    battle: &mut Battle,
    condition_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    match condition_id {
        "dynamax" => dynamax::on_source_modify_damage(battle, pokemon_pos),
        _ => EventResult::Continue,
    }
}

/// Dispatch onStallMove callbacks
pub fn dispatch_on_stall_move(
    battle: &mut Battle,
    condition_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    match condition_id {
        "stall" => stall::on_stall_move(battle, pokemon_pos),
        _ => EventResult::Continue,
    }
}

/// Dispatch onStart callbacks
pub fn dispatch_on_start(
    battle: &mut Battle,
    condition_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    match condition_id {
        "brn" => brn::on_start(battle, pokemon_pos),
        "choicelock" => choicelock::on_start(battle, pokemon_pos),
        "commanded" => commanded::on_start(battle, pokemon_pos),
        "confusion" => confusion::on_start(battle, pokemon_pos),
        "dynamax" => dynamax::on_start(battle, pokemon_pos),
        "frz" => frz::on_start(battle, pokemon_pos),
        "futuremove" => futuremove::on_start(battle, pokemon_pos),
        "healreplacement" => healreplacement::on_start(battle, pokemon_pos),
        "lockedmove" => lockedmove::on_start(battle, pokemon_pos),
        "mustrecharge" => mustrecharge::on_start(battle, pokemon_pos),
        "par" => par::on_start(battle, pokemon_pos),
        "partiallytrapped" => partiallytrapped::on_start(battle, pokemon_pos),
        "psn" => psn::on_start(battle, pokemon_pos),
        "slp" => slp::on_start(battle, pokemon_pos),
        "stall" => stall::on_start(battle, pokemon_pos),
        "tox" => tox::on_start(battle, pokemon_pos),
        "trapped" => trapped::on_start(battle, pokemon_pos),
        "twoturnmove" => twoturnmove::on_start(battle, pokemon_pos),
        _ => EventResult::Continue,
    }
}

/// Dispatch onSwitchIn callbacks
pub fn dispatch_on_switch_in(
    battle: &mut Battle,
    condition_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    match condition_id {
        "healreplacement" => healreplacement::on_switch_in(battle, pokemon_pos),
        "tox" => tox::on_switch_in(battle, pokemon_pos),
        _ => EventResult::Continue,
    }
}

/// Dispatch onTrapPokemon callbacks
pub fn dispatch_on_trap_pokemon(
    battle: &mut Battle,
    condition_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    match condition_id {
        "commanded" => commanded::on_trap_pokemon(battle, pokemon_pos),
        "commanding" => commanding::on_trap_pokemon(battle, pokemon_pos),
        "partiallytrapped" => partiallytrapped::on_trap_pokemon(battle, pokemon_pos),
        "trapped" => trapped::on_trap_pokemon(battle, pokemon_pos),
        _ => EventResult::Continue,
    }
}

/// Dispatch onTryAddVolatile callbacks
pub fn dispatch_on_try_add_volatile(
    battle: &mut Battle,
    condition_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    match condition_id {
        "dynamax" => dynamax::on_try_add_volatile(battle, pokemon_pos),
        _ => EventResult::Continue,
    }
}

/// Dispatch onTryMove callbacks
pub fn dispatch_on_try_move(
    battle: &mut Battle,
    condition_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    match condition_id {
        "desolateland" => desolateland::on_try_move(battle, pokemon_pos),
        "primordialsea" => primordialsea::on_try_move(battle, pokemon_pos),
        _ => EventResult::Continue,
    }
}

/// Dispatch onType callbacks
pub fn dispatch_on_type(
    battle: &mut Battle,
    condition_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    match condition_id {
        "arceus" => arceus::on_type(battle, pokemon_pos),
        "silvally" => silvally::on_type(battle, pokemon_pos),
        _ => EventResult::Continue,
    }
}

/// Dispatch onWeather callbacks
pub fn dispatch_on_weather(
    battle: &mut Battle,
    condition_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    match condition_id {
        "hail" => hail::on_weather(battle, pokemon_pos),
        "sandstorm" => sandstorm::on_weather(battle, pokemon_pos),
        _ => EventResult::Continue,
    }
}

/// Dispatch onWeatherModifyDamage callbacks
pub fn dispatch_on_weather_modify_damage(
    battle: &mut Battle,
    condition_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    match condition_id {
        "desolateland" => desolateland::on_weather_modify_damage(battle, pokemon_pos),
        "primordialsea" => primordialsea::on_weather_modify_damage(battle, pokemon_pos),
        "raindance" => raindance::on_weather_modify_damage(battle, pokemon_pos),
        "sunnyday" => sunnyday::on_weather_modify_damage(battle, pokemon_pos),
        _ => EventResult::Continue,
    }
}

/// Dispatch onTryHit callbacks (with source and target)
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
