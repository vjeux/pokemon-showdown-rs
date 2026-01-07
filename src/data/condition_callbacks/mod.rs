//! Condition Callback Handlers
//!
//! This module provides dispatch functions for condition callbacks.
//! Individual condition implementations are in separate files.

use crate::battle::Battle;
use crate::data::move_callbacks;
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
pub mod phantomforce;
pub mod primordialsea;
pub mod psn;
pub mod raindance;
pub mod rolloutstorage;
pub mod sandstorm;
pub mod silvally;
pub mod skydrop;
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
    source_pos: Option<(usize, usize)>,
    effect_id: Option<&str>,
) -> EventResult {
    match condition_id {
        "hail" => hail::duration_callback(battle, pokemon_pos, source_pos, effect_id),
        "partiallytrapped" => partiallytrapped::duration_callback(battle, pokemon_pos, source_pos, effect_id),
        "raindance" => raindance::duration_callback(battle, pokemon_pos, source_pos, effect_id),
        "sandstorm" => sandstorm::duration_callback(battle, pokemon_pos, source_pos, effect_id),
        "snowscape" => snowscape::duration_callback(battle, pokemon_pos, source_pos, effect_id),
        "sunnyday" => sunnyday::duration_callback(battle, pokemon_pos, source_pos, effect_id),
        _ => {
            // Fallback to move-embedded condition callbacks
            move_callbacks::dispatch_condition_duration_callback(battle, condition_id, pokemon_pos)
        }
    }
}
/// Dispatch onAfterMove callbacks
pub fn dispatch_on_after_move(
    battle: &mut Battle,
    condition_id: &str,
    pokemon_pos: (usize, usize),
    target_pos: Option<(usize, usize)>,
    move_id: &str,
) -> EventResult {
    match condition_id {
        "lockedmove" => lockedmove::on_after_move(battle, pokemon_pos, target_pos, move_id),
        _ => {
            // Fallback to move-embedded condition callbacks
            // dispatch_condition_on_after_move takes (battle, move_id, source_pos, target_pos)
            move_callbacks::dispatch_condition_on_after_move(battle, condition_id, pokemon_pos, target_pos)
        }
    }
}
/// Dispatch onAfterMoveSecondary callbacks
pub fn dispatch_on_after_move_secondary(
    battle: &mut Battle,
    condition_id: &str,
    pokemon_pos: (usize, usize),
    source_pos: Option<(usize, usize)>,
    move_id: &str,
) -> EventResult {
    match condition_id {
        "frz" => frz::on_after_move_secondary(battle, pokemon_pos, source_pos, move_id),
        _ => EventResult::Continue,
    }
}
/// Dispatch onBasePower callbacks
pub fn dispatch_on_base_power(
    battle: &mut Battle,
    condition_id: &str,
    base_power: i32,
    pokemon_pos: (usize, usize),
    target_pos: Option<(usize, usize)>,
    move_id: &str,
) -> EventResult {
    match condition_id {
        "gem" => gem::on_base_power(battle, base_power, pokemon_pos, target_pos, move_id),
        "rolloutstorage" => rolloutstorage::on_base_power(battle, base_power, pokemon_pos, target_pos, move_id),
        _ => {
            // Fallback to move-embedded condition callbacks
            // dispatch_condition_on_base_power takes (battle, move_id, source_pos, target_pos)
            move_callbacks::dispatch_condition_on_base_power(battle, condition_id, pokemon_pos, target_pos)
        }
    }
}
/// Dispatch onBeforeMove callbacks
pub fn dispatch_on_before_move(
    battle: &mut Battle,
    condition_id: &str,
    pokemon_pos: (usize, usize),
    target_pos: Option<(usize, usize)>,
    move_id: &str,
) -> EventResult {
    match condition_id {
        "choicelock" => choicelock::on_before_move(battle, pokemon_pos, target_pos, move_id),
        "confusion" => confusion::on_before_move(battle, pokemon_pos, target_pos, move_id),
        "flinch" => flinch::on_before_move(battle, pokemon_pos, target_pos, move_id),
        "frz" => frz::on_before_move(battle, pokemon_pos, target_pos, move_id),
        "mustrecharge" => mustrecharge::on_before_move(battle, pokemon_pos, target_pos, move_id),
        "par" => par::on_before_move(battle, pokemon_pos, target_pos, move_id),
        "slp" => slp::on_before_move(battle, pokemon_pos, target_pos, move_id),
        _ => {
            // Fallback to move-embedded condition callbacks
            move_callbacks::dispatch_condition_on_before_move(battle, condition_id, pokemon_pos)
        }
    }
}
/// Dispatch onFoeBeforeMove callbacks
pub fn dispatch_on_foe_before_move(
    battle: &mut Battle,
    condition_id: &str,
    pokemon_pos: (usize, usize),
    target_pos: Option<(usize, usize)>,
    source_pos: Option<(usize, usize)>,
    move_id: &str,
) -> EventResult {
    match condition_id {
        "skydrop" => skydrop::on_foe_before_move(battle, pokemon_pos, target_pos, source_pos, move_id),
        _ => {
            // Fallback to move-embedded condition callbacks
            move_callbacks::dispatch_condition_on_foe_before_move(battle, condition_id, pokemon_pos)
        }
    }
}
/// Dispatch onAllyBeforeMove callbacks
pub fn dispatch_on_ally_before_move(
    _battle: &mut Battle,
    condition_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    match condition_id {
        _ => EventResult::Continue,
    }
}
/// Dispatch onSourceBeforeMove callbacks
pub fn dispatch_on_source_before_move(
    _battle: &mut Battle,
    condition_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    match condition_id {
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
        _ => {
            // Fallback to move-embedded condition callbacks
            move_callbacks::dispatch_condition_on_before_switch_out(battle, condition_id, pokemon_pos)
        }
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
    damage: i32,
    pokemon_pos: (usize, usize),
    source_pos: Option<(usize, usize)>,
    move_id: &str,
) -> EventResult {
    match condition_id {
        "frz" => frz::on_damaging_hit(battle, damage, pokemon_pos, source_pos, move_id),
        _ => {
            // Fallback to move-embedded condition callbacks
            move_callbacks::dispatch_condition_on_damaging_hit(battle, condition_id, pokemon_pos)
        }
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
        _ => {
            // Fallback to move-embedded condition callbacks
            move_callbacks::dispatch_condition_on_disable_move(battle, condition_id, pokemon_pos)
        }
    }
}
/// Dispatch onDragOut callbacks
pub fn dispatch_on_drag_out(
    battle: &mut Battle,
    condition_id: &str,
    pokemon_pos: (usize, usize),
    source_pos: Option<(usize, usize)>,
    move_id: &str,
) -> EventResult {
    match condition_id {
        "commanded" => commanded::on_drag_out(battle, pokemon_pos, source_pos, move_id),
        "commanding" => commanding::on_drag_out(battle, pokemon_pos, source_pos, move_id),
        "dynamax" => dynamax::on_drag_out(battle, pokemon_pos, source_pos, move_id),
        _ => {
            // Fallback to move-embedded condition callbacks
            move_callbacks::dispatch_condition_on_drag_out(battle, condition_id, pokemon_pos)
        }
    }
}
/// Dispatch onEffectiveness callbacks
pub fn dispatch_on_effectiveness(
    battle: &mut Battle,
    condition_id: &str,
    type_mod: i32,
    target_type: &str,
    pokemon_pos: (usize, usize),
    move_id: &str,
) -> EventResult {
    eprintln!("[DISPATCH_ON_EFFECTIVENESS] condition_id={}, type_mod={}, target_type={}, pokemon_pos={:?}, move_id={}",
        condition_id, type_mod, target_type, pokemon_pos, move_id);

    match condition_id {
        "deltastream" => deltastream::on_effectiveness(battle, type_mod, target_type, pokemon_pos, move_id),
        "tarshot" => {
            eprintln!("[DISPATCH_ON_EFFECTIVENESS] Calling tarshot callback with type_mod={}, target_type={}", type_mod, target_type);
            crate::data::move_callbacks::dispatch_condition_on_effectiveness(
                battle,
                "tarshot",
                type_mod,
                target_type,
                pokemon_pos,
            )
        }
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
        _ => {
            // Fallback to move-embedded condition callbacks
            move_callbacks::dispatch_condition_on_end(battle, condition_id, pokemon_pos)
        }
    }
}
/// Dispatch onFaint callbacks (with target, source, effect)
pub fn dispatch_on_faint(
    battle: &mut Battle,
    condition_id: &str,
    target_pos: Option<(usize, usize)>,
    source_pos: Option<(usize, usize)>,
    effect_id: Option<&str>,
) -> EventResult {
    // Route to actual implementation in move_callbacks
    use crate::data::move_callbacks;
    move_callbacks::dispatch_condition_on_faint(battle, condition_id, target_pos, source_pos, effect_id)
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
        _ => {
            // Fallback to move-embedded condition callbacks
            move_callbacks::dispatch_condition_on_field_end(battle, condition_id, pokemon_pos)
        }
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
        _ => {
            // Fallback to move-embedded condition callbacks
            move_callbacks::dispatch_condition_on_field_residual(battle, condition_id, pokemon_pos)
        }
    }
}
/// Dispatch onFieldStart callbacks
pub fn dispatch_on_field_start(
    battle: &mut Battle,
    condition_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    use crate::data::move_callbacks;

    match condition_id {
        "deltastream" => deltastream::on_field_start(battle, _pokemon_pos),
        "desolateland" => desolateland::on_field_start(battle, _pokemon_pos),
        "hail" => hail::on_field_start(battle, _pokemon_pos),
        "primordialsea" => primordialsea::on_field_start(battle, _pokemon_pos),
        "raindance" => raindance::on_field_start(battle, _pokemon_pos),
        "sandstorm" => sandstorm::on_field_start(battle, _pokemon_pos),
        "snowscape" => snowscape::on_field_start(battle, _pokemon_pos),
        "sunnyday" => sunnyday::on_field_start(battle, _pokemon_pos),
        _ => {
            // Fallback to move-embedded condition callbacks
            move_callbacks::dispatch_condition_on_field_start(battle, condition_id, _pokemon_pos)
        }
    }
}
/// Dispatch onFieldRestart callbacks
pub fn dispatch_on_field_restart(
    battle: &mut Battle,
    condition_id: &str,
    _pokemon_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
    _source_pos: Option<(usize, usize)>,
) -> EventResult {
    use crate::data::move_callbacks;

    match condition_id {
        _ => {
            // Fallback to move-embedded condition callbacks
            move_callbacks::dispatch_condition_on_field_restart(battle, condition_id, _pokemon_pos)
        }
    }
}
/// Dispatch onImmunity callbacks
pub fn dispatch_on_immunity(
    battle: &mut Battle,
    condition_id: &str,
    immunity_type: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    match condition_id {
        "desolateland" => desolateland::on_immunity(battle, immunity_type, pokemon_pos),
        "sunnyday" => sunnyday::on_immunity(battle, immunity_type, pokemon_pos),
        _ => {
            // Fallback to move-embedded condition callbacks
            move_callbacks::dispatch_condition_on_immunity(battle, condition_id, immunity_type, pokemon_pos)
        }
    }
}
/// Dispatch onAnyInvulnerability callbacks
pub fn dispatch_on_any_invulnerability(
    battle: &mut Battle,
    condition_id: &str,
    target_pos: (usize, usize),
    source_pos: (usize, usize),
    attacking_move_id: &str,
) -> EventResult {
    println!("[DISPATCH_INVULN] Called with condition_id='{}', target_pos={:?}, source_pos={:?}, attacking_move_id='{}'",
        condition_id, target_pos, source_pos, attacking_move_id);

    let result = match condition_id {
        "skydrop" => {
            println!("[DISPATCH_INVULN] Routing to skydrop callback");
            skydrop::on_any_invulnerability(battle, target_pos, source_pos, attacking_move_id)
        },
        _ => {
            println!("[DISPATCH_INVULN] No match for '{}', trying fallback", condition_id);
            // Fallback to move-embedded condition callbacks
            move_callbacks::dispatch_condition_on_any_invulnerability(battle, condition_id, target_pos, source_pos, attacking_move_id)
        },
    };

    println!("[DISPATCH_INVULN] Returning {:?}", result);
    result
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
    _def: i32,
    pokemon_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
    _source_pos: Option<(usize, usize)>,
    _move_id: &str,
) -> EventResult {
    match condition_id {
        "snowscape" => snowscape::on_modify_def(battle, _def, pokemon_pos, _target_pos, _source_pos, _move_id),
        _ => EventResult::Continue,
    }
}
/// Dispatch onModifyMove callbacks
pub fn dispatch_on_modify_move(
    battle: &mut Battle,
    condition_id: &str,
    pokemon_pos: (usize, usize),
    target_pos: Option<(usize, usize)>,
) -> EventResult {
    match condition_id {
        "frz" => frz::on_modify_move(battle, pokemon_pos, target_pos),
        _ => {
            // Fallback to move-embedded condition callbacks
            move_callbacks::dispatch_condition_on_modify_move(battle, condition_id, pokemon_pos)
        }
    }
}
/// Dispatch onModifySpD callbacks
pub fn dispatch_on_modify_sp_d(
    battle: &mut Battle,
    condition_id: &str,
    _spd: i32,
    pokemon_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
    _source_pos: Option<(usize, usize)>,
    _move_id: &str,
) -> EventResult {
    match condition_id {
        "sandstorm" => sandstorm::on_modify_sp_d(battle, _spd, pokemon_pos, _target_pos, _source_pos, _move_id),
        _ => EventResult::Continue,
    }
}
/// Dispatch onModifySpe callbacks
pub fn dispatch_on_modify_spe(
    battle: &mut Battle,
    condition_id: &str,
    spe: i32,
    pokemon_pos: (usize, usize),
) -> EventResult {
    match condition_id {
        "par" => par::on_modify_spe(battle, spe, pokemon_pos),
        _ => {
            // Fallback to move-embedded condition callbacks
            // dispatch_condition_on_modify_spe takes (battle, move_id, spe, source_pos)
            move_callbacks::dispatch_condition_on_modify_spe(battle, condition_id, spe, pokemon_pos)
        }
    }
}
/// Dispatch onMoveAborted callbacks
pub fn dispatch_on_move_aborted(
    battle: &mut Battle,
    condition_id: &str,
    pokemon_pos: (usize, usize),
    target_pos: Option<(usize, usize)>,
    move_id: &str,
) -> EventResult {
    match condition_id {
        "twoturnmove" => twoturnmove::on_move_aborted(battle, pokemon_pos, target_pos, move_id),
        _ => {
            // Fallback to move-embedded condition callbacks
            move_callbacks::dispatch_condition_on_move_aborted(battle, condition_id, pokemon_pos)
        }
    }
}
/// Dispatch onResidual callbacks
pub fn dispatch_on_residual(
    battle: &mut Battle,
    condition_id: &str,
    pokemon_pos: (usize, usize),
    source_pos: Option<(usize, usize)>,
    effect_id: Option<&str>,
) -> EventResult {
    eprintln!("[DISPATCH_ON_RESIDUAL] condition_id='{}', pokemon_pos={:?}, turn={}",
        condition_id, pokemon_pos, battle.turn);
    match condition_id {
        "brn" => brn::on_residual(battle, pokemon_pos, source_pos, effect_id),
        "dynamax" => dynamax::on_residual(battle, pokemon_pos, source_pos, effect_id),
        "futuremove" => futuremove::on_residual(battle, pokemon_pos, source_pos, effect_id),
        "lockedmove" => lockedmove::on_residual(battle, pokemon_pos, source_pos, effect_id),
        "partiallytrapped" => partiallytrapped::on_residual(battle, pokemon_pos, source_pos, effect_id),
        "psn" => psn::on_residual(battle, pokemon_pos, source_pos, effect_id),
        "tox" => tox::on_residual(battle, pokemon_pos, source_pos, effect_id),
        _ => {
            // Fallback to move-embedded condition callbacks
            move_callbacks::dispatch_condition_on_residual(battle, condition_id, pokemon_pos)
        }
    }
}
/// Dispatch onRestart callbacks
pub fn dispatch_on_restart(
    battle: &mut Battle,
    condition_id: &str,
    pokemon_pos: (usize, usize),
    source_pos: Option<(usize, usize)>,
    effect_id: Option<&str>,
) -> EventResult {
    match condition_id {
        "lockedmove" => lockedmove::on_restart(battle, pokemon_pos, source_pos, effect_id),
        "stall" => stall::on_restart(battle, pokemon_pos, source_pos, effect_id),
        _ => {
            // Fallback to move-embedded condition callbacks
            move_callbacks::dispatch_condition_on_restart(battle, condition_id, pokemon_pos)
        }
    }
}
/// Dispatch onSourceModifyDamage callbacks
pub fn dispatch_on_source_modify_damage(
    battle: &mut Battle,
    condition_id: &str,
    _damage: i32,
    source_pos: (usize, usize),
    target_pos: (usize, usize),
    move_id: &str,
) -> EventResult {
    match condition_id {
        "dynamax" => dynamax::on_source_modify_damage(battle, _damage, source_pos, target_pos, move_id),
        _ => {
            // Fallback to move-embedded condition callbacks
            move_callbacks::dispatch_condition_on_source_modify_damage(battle, condition_id, source_pos, target_pos)
        }
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
    source_pos: Option<(usize, usize)>,
    effect_id: Option<&str>,
) -> EventResult {
    match condition_id {
        "brn" => brn::on_start(battle, pokemon_pos, source_pos, effect_id),
        "choicelock" => choicelock::on_start(battle, pokemon_pos, source_pos, effect_id),
        "commanded" => commanded::on_start(battle, pokemon_pos, source_pos, effect_id),
        "confusion" => confusion::on_start(battle, pokemon_pos, source_pos, effect_id),
        "dynamax" => dynamax::on_start(battle, pokemon_pos, source_pos, effect_id),
        "frz" => frz::on_start(battle, pokemon_pos, source_pos, effect_id),
        "futuremove" => futuremove::on_start(battle, pokemon_pos, source_pos, effect_id),
        "healreplacement" => healreplacement::on_start(battle, pokemon_pos, source_pos, effect_id),
        "lockedmove" => lockedmove::on_start(battle, pokemon_pos, source_pos, effect_id),
        "mustrecharge" => mustrecharge::on_start(battle, pokemon_pos, source_pos, effect_id),
        "par" => par::on_start(battle, pokemon_pos, source_pos, effect_id),
        "partiallytrapped" => partiallytrapped::on_start(battle, pokemon_pos, source_pos, effect_id),
        "psn" => psn::on_start(battle, pokemon_pos, source_pos, effect_id),
        "slp" => slp::on_start(battle, pokemon_pos, source_pos, effect_id),
        "stall" => stall::on_start(battle, pokemon_pos, source_pos, effect_id),
        "tox" => tox::on_start(battle, pokemon_pos, source_pos, effect_id),
        "trapped" => trapped::on_start(battle, pokemon_pos, source_pos, effect_id),
        "twoturnmove" => twoturnmove::on_start(battle, pokemon_pos, source_pos, effect_id),
        _ => {
            // Try move-embedded condition callbacks
            use crate::data::move_callbacks;
            move_callbacks::dispatch_condition_on_start(battle, condition_id, pokemon_pos)
        }
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
        _ => {
            // Fallback to move-embedded condition callbacks
            move_callbacks::dispatch_condition_on_switch_in(battle, condition_id, pokemon_pos)
        }
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
        _ => {
            // Fallback to move-embedded condition callbacks
            move_callbacks::dispatch_condition_on_trap_pokemon(battle, condition_id, pokemon_pos)
        }
    }
}
/// Dispatch onTryAddVolatile callbacks
pub fn dispatch_on_try_add_volatile(
    battle: &mut Battle,
    condition_id: &str,
    status: Option<&str>,
    target_pos: Option<(usize, usize)>,
    source_pos: Option<(usize, usize)>,
    effect_id: Option<&str>,
) -> EventResult {
    match condition_id {
        "dynamax" => dynamax::on_try_add_volatile(battle, status, target_pos.unwrap_or((0,0)), source_pos, effect_id),
        _ => {
            // Fallback to move-embedded condition callbacks
            move_callbacks::dispatch_condition_on_try_add_volatile(
                battle,
                condition_id,
                status,
                target_pos,
                source_pos,
                effect_id
            )
        }
    }
}
/// Dispatch onTryMove callbacks
pub fn dispatch_on_try_move(
    battle: &mut Battle,
    condition_id: &str,
    pokemon_pos: (usize, usize),
    target_pos: Option<(usize, usize)>,
    move_id: Option<&str>,
) -> EventResult {
    match condition_id {
        "desolateland" => desolateland::on_try_move(battle, pokemon_pos, target_pos, move_id),
        "primordialsea" => primordialsea::on_try_move(battle, pokemon_pos, target_pos, move_id),
        _ => {
            // Fallback to move-embedded condition callbacks
            move_callbacks::dispatch_condition_on_try_move(battle, condition_id, pokemon_pos, target_pos, move_id)
        }
    }
}
/// Dispatch onType callbacks
pub fn dispatch_on_type(
    battle: &mut Battle,
    condition_id: &str,
    pokemon_pos: (usize, usize),
    types: Option<&[String]>,
) -> EventResult {
    match condition_id {
        "arceus" => arceus::on_type(battle, pokemon_pos, types),
        "silvally" => silvally::on_type(battle, pokemon_pos, types),
        _ => {
            // Fallback to move-embedded condition callbacks
            move_callbacks::dispatch_condition_on_type(battle, condition_id, pokemon_pos, types)
        }
    }
}
/// Dispatch onWeather callbacks
pub fn dispatch_on_weather(
    battle: &mut Battle,
    condition_id: &str,
    pokemon_pos: (usize, usize),
    source_pos: Option<(usize, usize)>,
    effect_id: Option<&str>,
) -> EventResult {
    match condition_id {
        "hail" => hail::on_weather(battle, pokemon_pos, source_pos, effect_id),
        "sandstorm" => sandstorm::on_weather(battle, pokemon_pos, source_pos, effect_id),
        _ => EventResult::Continue,
    }
}
/// Dispatch onWeatherModifyDamage callbacks
pub fn dispatch_on_weather_modify_damage(
    battle: &mut Battle,
    condition_id: &str,
    damage: i32,
    attacker_pos: Option<(usize, usize)>,
    defender_pos: Option<(usize, usize)>,
    move_id: Option<&str>,
) -> EventResult {
    match condition_id {
        "desolateland" => desolateland::on_weather_modify_damage(battle, damage, attacker_pos, defender_pos, move_id),
        "primordialsea" => primordialsea::on_weather_modify_damage(battle, damage, attacker_pos, defender_pos, move_id),
        "raindance" => raindance::on_weather_modify_damage(battle, damage, attacker_pos, defender_pos, move_id),
        "sunnyday" => sunnyday::on_weather_modify_damage(battle, damage, attacker_pos, defender_pos, move_id),
        _ => EventResult::Continue,
    }
}
/// Dispatch onTryHit callbacks (with source and target)
pub fn dispatch_on_try_hit(
    battle: &mut Battle,
    condition_id: &str,
    source_pos: (usize, usize),
    target_pos: (usize, usize),
    move_id: Option<&str>,
) -> EventResult {
    // Route to actual implementation in move_callbacks
    use crate::data::move_callbacks;
    move_callbacks::dispatch_condition_on_try_hit(battle, condition_id, source_pos, target_pos, move_id)
}
/// Dispatch onTryPrimaryHit callbacks
pub fn dispatch_on_try_primary_hit(
    battle: &mut Battle,
    condition_id: &str,
    target_pos: (usize, usize),
    source_pos: Option<(usize, usize)>,
    move_id: Option<&str>,
) -> EventResult {
    // Route to actual implementation in move_callbacks
    use crate::data::move_callbacks;
    move_callbacks::dispatch_condition_on_try_primary_hit(battle, condition_id, target_pos, source_pos, move_id)
}
