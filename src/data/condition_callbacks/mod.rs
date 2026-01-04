//! Condition Callback Handlers
//!
//! This module provides dispatch functions for condition callbacks.
//! Individual condition implementations are in separate files.

use crate::battle::Battle;
use crate::event::EventResult;
use crate::ID;

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

/// Dispatch onBasePowerPriority callbacks
pub fn dispatch_on_base_power_priority(
    battle: &mut Battle,
    condition_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    match condition_id {
        "gem" => gem::on_base_power_priority(battle, pokemon_pos),
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
        "flinch" => flinch::on_before_move(battle, pokemon_pos),
        "mustrecharge" => mustrecharge::on_before_move(battle, pokemon_pos),
        "par" => par::on_before_move(battle, pokemon_pos),
        _ => EventResult::Continue,
    }
}

/// Dispatch onBeforeMovePriority callbacks
pub fn dispatch_on_before_move_priority(
    battle: &mut Battle,
    condition_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    match condition_id {
        "flinch" => flinch::on_before_move_priority(battle, pokemon_pos),
        "mustrecharge" => mustrecharge::on_before_move_priority(battle, pokemon_pos),
        "par" => par::on_before_move_priority(battle, pokemon_pos),
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
        "commanding" => commanding::on_drag_out(battle, pokemon_pos),
        _ => EventResult::Continue,
    }
}

/// Dispatch onDragOutPriority callbacks
pub fn dispatch_on_drag_out_priority(
    battle: &mut Battle,
    condition_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    match condition_id {
        "commanding" => commanding::on_drag_out_priority(battle, pokemon_pos),
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

/// Dispatch onEffectivenessPriority callbacks
pub fn dispatch_on_effectiveness_priority(
    battle: &mut Battle,
    condition_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    match condition_id {
        "deltastream" => deltastream::on_effectiveness_priority(battle, pokemon_pos),
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
        "lockedmove" => lockedmove::on_residual(battle, pokemon_pos),
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
        "stall" => stall::on_restart(battle, pokemon_pos),
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
        "par" => par::on_start(battle, pokemon_pos),
        "psn" => psn::on_start(battle, pokemon_pos),
        "slp" => slp::on_start(battle, pokemon_pos),
        "stall" => stall::on_start(battle, pokemon_pos),
        "tox" => tox::on_start(battle, pokemon_pos),
        "twoturnmove" => twoturnmove::on_start(battle, pokemon_pos),
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
        "trapped" => trapped::on_trap_pokemon(battle, pokemon_pos),
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

/// Dispatch onTryMovePriority callbacks
pub fn dispatch_on_try_move_priority(
    battle: &mut Battle,
    condition_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    match condition_id {
        "desolateland" => desolateland::on_try_move_priority(battle, pokemon_pos),
        "primordialsea" => primordialsea::on_try_move_priority(battle, pokemon_pos),
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

/// Dispatch onTypePriority callbacks
pub fn dispatch_on_type_priority(
    battle: &mut Battle,
    condition_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    match condition_id {
        "arceus" => arceus::on_type_priority(battle, pokemon_pos),
        "silvally" => silvally::on_type_priority(battle, pokemon_pos),
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

// =========================================================================
// ADDITIONAL DISPATCH FUNCTIONS (Stub Implementations)
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

/// Dispatch onSourceModifyDamage callbacks
pub fn dispatch_on_source_modify_damage(
    _battle: &mut Battle,
    _condition_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
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

/// Dispatch onTryMove callbacks (deprecated - use dispatch_on_try_move instead)
pub fn dispatch_on_try_move_old(
    _battle: &mut Battle,
    _condition_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onTryMovePriority callbacks (deprecated - use dispatch_on_try_move_priority instead)
pub fn dispatch_on_try_move_priority_old(
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
    _battle: &mut Battle,
    _condition_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}
