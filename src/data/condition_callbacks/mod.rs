//! Condition Callback Handlers
//!
//! This module provides dispatch functions for condition callbacks.
//! Individual condition implementations are in separate files.

use crate::battle::Battle;
use crate::battle_actions::ActiveMove;
use crate::data::ability_callbacks;
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
pub mod luckychant;
pub mod micleberry;
pub mod mustrecharge;
pub mod par;
pub mod partiallytrapped;
pub mod phantomforce;
pub mod primordialsea;
pub mod psn;
pub mod raindance;
pub mod rolloutstorage;
pub mod sandstorm;
pub mod shadowforce;
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
            // Pass effect_id through so durationCallbacks can check the source effect
            move_callbacks::dispatch_condition_duration_callback(battle, condition_id, pokemon_pos, effect_id)
        }
    }
}
/// Dispatch onAfterMove callbacks
pub fn dispatch_on_after_move(
    battle: &mut Battle,
    condition_id: &str,
    pokemon_pos: (usize, usize),
    target_pos: Option<(usize, usize)>,
    active_move: Option<&ActiveMove>,
) -> EventResult {
    match condition_id {
        "lockedmove" => lockedmove::on_after_move(battle, pokemon_pos, target_pos, active_move),
        // Charge volatile: remove after using Electric move (not the Charge move itself)
        "charge" => move_callbacks::charge::condition::on_after_move(battle, pokemon_pos, target_pos),
        _ => {
            // Fallback to move-embedded condition callbacks
            // dispatch_condition_on_after_move takes (battle, move_id, source_pos, target_pos)
            move_callbacks::dispatch_condition_on_after_move(battle, active_move, pokemon_pos, target_pos)
        }
    }
}
/// Dispatch onAfterMoveSecondary callbacks
pub fn dispatch_on_after_move_secondary(
    battle: &mut Battle,
    condition_id: &str,
    pokemon_pos: (usize, usize),
    source_pos: Option<(usize, usize)>,
    active_move: Option<&ActiveMove>,
) -> EventResult {
    match condition_id {
        "frz" => frz::on_after_move_secondary(battle, pokemon_pos, source_pos, active_move),
        _ => EventResult::Continue,
    }
}
/// Dispatch onAccuracy callbacks
///
/// Called during accuracy check to determine if a move hits.
/// Volatiles like glaiverush return true to make moves always hit.
pub fn dispatch_on_accuracy(
    battle: &mut Battle,
    condition_id: &str,
    accuracy: i32,
    target_pos: Option<(usize, usize)>,
    source_pos: Option<(usize, usize)>,
    active_move: Option<&ActiveMove>,
) -> EventResult {
    match condition_id {
        "glaiverush" => {
            move_callbacks::glaiverush::condition::on_accuracy(battle, accuracy, target_pos, source_pos, active_move)
        }
        "minimize" => {
            move_callbacks::minimize::condition::on_accuracy(battle, accuracy, target_pos, source_pos, active_move)
        }
        "telekinesis" => {
            move_callbacks::telekinesis::condition::on_accuracy(battle, accuracy, target_pos, source_pos, active_move)
        }
        _ => EventResult::Continue,
    }
}

/// Dispatch onSourceAccuracy callbacks
/// Called on volatiles of the SOURCE Pokemon during accuracy checks
/// Lock-On/Mind Reader return true to bypass accuracy checks
pub fn dispatch_on_source_accuracy(
    battle: &mut Battle,
    condition_id: &str,
    accuracy: i32,
    target_pos: Option<(usize, usize)>,
    source_pos: Option<(usize, usize)>,
    active_move: Option<&ActiveMove>,
) -> EventResult {
    match condition_id {
        "lockon" => {
            move_callbacks::lockon::condition::on_source_accuracy(battle, accuracy, target_pos, source_pos, active_move)
        }
        "micleberry" => {
            micleberry::on_source_accuracy(battle, accuracy, target_pos, source_pos, active_move)
        }
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
    active_move: Option<&ActiveMove>,
) -> EventResult {
    match condition_id {
        "charge" => move_callbacks::charge::condition::on_base_power(battle, base_power, pokemon_pos, target_pos),
        "electricterrain" => move_callbacks::electricterrain::condition::on_base_power(battle, base_power, pokemon_pos, target_pos),
        "gem" => gem::on_base_power(battle, base_power, pokemon_pos, target_pos, active_move),
        "grassyterrain" => move_callbacks::grassyterrain::condition::on_base_power(battle, base_power, pokemon_pos, target_pos),
        "mefirst" => move_callbacks::mefirst::condition::on_base_power(battle, base_power, pokemon_pos, target_pos),
        "mistyterrain" => move_callbacks::mistyterrain::condition::on_base_power(battle, base_power, pokemon_pos, target_pos),
        "mudsport" => move_callbacks::mudsport::condition::on_base_power(battle, base_power, pokemon_pos, target_pos),
        "psychicterrain" => move_callbacks::psychicterrain::condition::on_base_power(battle, base_power, pokemon_pos, target_pos),
        "rolloutstorage" => rolloutstorage::on_base_power(battle, base_power, pokemon_pos, target_pos, active_move),
        "watersport" => move_callbacks::watersport::condition::on_base_power(battle, base_power, pokemon_pos, target_pos),
        _ => {
            // Fallback to move-embedded condition callbacks
            // dispatch_condition_on_base_power takes (battle, move_id, source_pos, target_pos)
            move_callbacks::dispatch_condition_on_base_power(battle, active_move, pokemon_pos, target_pos)
        }
    }
}
/// Dispatch onBeforeMove callbacks
pub fn dispatch_on_before_move(
    battle: &mut Battle,
    condition_id: &str,
    pokemon_pos: (usize, usize),
    target_pos: Option<(usize, usize)>,
    active_move: Option<&ActiveMove>,
) -> EventResult {
    match condition_id {
        "choicelock" => choicelock::on_before_move(battle, pokemon_pos, target_pos, active_move),
        "confusion" => confusion::on_before_move(battle, pokemon_pos, target_pos, active_move),
        "flinch" => flinch::on_before_move(battle, pokemon_pos, target_pos, active_move),
        "frz" => frz::on_before_move(battle, pokemon_pos, target_pos, active_move),
        "mustrecharge" => mustrecharge::on_before_move(battle, pokemon_pos, target_pos, active_move),
        "par" => par::on_before_move(battle, pokemon_pos, target_pos, active_move),
        "slp" => slp::on_before_move(battle, pokemon_pos, target_pos, active_move),
        "taunt" => move_callbacks::taunt::condition::on_before_move(battle, pokemon_pos, active_move),
        _ => {
            // Fallback to move-embedded condition callbacks
            move_callbacks::dispatch_condition_on_before_move(battle, condition_id, active_move, pokemon_pos)
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
    active_move: Option<&ActiveMove>,
) -> EventResult {
    match condition_id {
        "imprison" => move_callbacks::imprison::condition::on_foe_before_move(battle, active_move),
        "skydrop" => skydrop::on_foe_before_move(battle, pokemon_pos, target_pos, source_pos, active_move),
        _ => {
            // Fallback to move-embedded condition callbacks
            move_callbacks::dispatch_condition_on_foe_before_move(battle, active_move, pokemon_pos)
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
/// Dispatch side condition onCriticalHit callbacks
pub fn dispatch_side_condition_on_critical_hit(
    battle: &mut Battle,
    condition_id: &str,
    target_pos: Option<(usize, usize)>,
    source_pos: Option<(usize, usize)>,
    active_move: Option<&ActiveMove>,
) -> EventResult {
    match condition_id {
        "luckychant" => luckychant::on_critical_hit(battle, target_pos, source_pos, active_move),
        _ => EventResult::Continue,
    }
}
/// Dispatch side condition onTryBoost callbacks
/// Called when a stat boost is being applied to check if it should be modified/prevented
/// Mist prevents stat drops from opponent's moves
pub fn dispatch_side_condition_on_try_boost(
    battle: &mut Battle,
    condition_id: &str,
    target_pos: Option<(usize, usize)>,
    source_pos: Option<(usize, usize)>,
    _effect_id: Option<&str>,
) -> EventResult {
    match condition_id {
        "mist" => move_callbacks::mist::condition::on_try_boost(battle, target_pos, source_pos, _effect_id),
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
    active_move: Option<&ActiveMove>,
) -> EventResult {
    match condition_id {
        "frz" => frz::on_damaging_hit(battle, damage, pokemon_pos, source_pos, active_move),
        "mirrorcoat" => move_callbacks::mirrorcoat::condition::on_damaging_hit(battle, damage, Some(pokemon_pos), source_pos, active_move),
        "counter" => move_callbacks::counter::condition::on_damaging_hit(battle, damage, Some(pokemon_pos), source_pos, active_move),
        _ => {
            // Fallback to move-embedded condition callbacks
            move_callbacks::dispatch_condition_on_damaging_hit(battle, active_move, pokemon_pos)
        }
    }
}
/// Dispatch onDamage callbacks for conditions like Endure, Focus Band, Focus Sash
/// Called from spread_damage when dealing damage to a Pokemon
/// JavaScript: condition.onDamage(damage, target, source, effect)
pub fn dispatch_on_damage(
    battle: &mut Battle,
    condition_id: &str,
    damage: i32,
    target_pos: (usize, usize),
    source_pos: Option<(usize, usize)>,
    effect_id: Option<&str>,
    is_move_effect: bool,
) -> EventResult {
    match condition_id {
        "bide" => move_callbacks::bide::condition::on_damage(battle, damage, target_pos, source_pos, effect_id, is_move_effect),
        "endure" => move_callbacks::endure::condition::on_damage(battle, damage, target_pos, source_pos, effect_id, is_move_effect),
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
        _ => {
            // Fallback to move-embedded condition callbacks
            move_callbacks::dispatch_condition_on_disable_move(battle, condition_id, pokemon_pos)
        }
    }
}
/// Dispatch onFoeDisableMove callbacks
pub fn dispatch_on_foe_disable_move(
    battle: &mut Battle,
    condition_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    match condition_id {
        "imprison" => move_callbacks::imprison::condition::on_foe_disable_move(battle, pokemon_pos),
        _ => EventResult::Continue,
    }
}
/// Dispatch onDragOut callbacks
pub fn dispatch_on_drag_out(
    battle: &mut Battle,
    condition_id: &str,
    pokemon_pos: (usize, usize),
    source_pos: Option<(usize, usize)>,
    active_move: Option<&ActiveMove>,
) -> EventResult {
    match condition_id {
        "commanded" => commanded::on_drag_out(battle, pokemon_pos, source_pos, active_move),
        "commanding" => commanding::on_drag_out(battle, pokemon_pos, source_pos, active_move),
        "dynamax" => dynamax::on_drag_out(battle, pokemon_pos, source_pos, active_move),
        "ingrain" => move_callbacks::ingrain::condition::on_drag_out(battle, pokemon_pos),
        _ => {
            // Fallback to move-embedded condition callbacks
            move_callbacks::dispatch_condition_on_drag_out(battle, active_move, pokemon_pos)
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
    active_move: Option<&ActiveMove>,
) -> EventResult {
    eprintln!("[DISPATCH_ON_EFFECTIVENESS] condition_id={}, type_mod={}, target_type={}, pokemon_pos={:?}, active_move={:?}",
        condition_id, type_mod, target_type, pokemon_pos, active_move);

    match condition_id {
        "deltastream" => deltastream::on_effectiveness(battle, type_mod, target_type, pokemon_pos, active_move),
        "tarshot" => {
            eprintln!("[DISPATCH_ON_EFFECTIVENESS] Calling tarshot callback with type_mod={}, target_type={}", type_mod, target_type);
            // Call tarshot's on_effectiveness directly (not through move_callbacks dispatcher
            // which would match on active_move.id instead of condition_id)
            move_callbacks::tarshot::condition::on_effectiveness(
                battle,
                type_mod,
                target_type,
                Some(pokemon_pos),
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
    attacking_active_move: Option<&ActiveMove>,
) -> EventResult {
    println!("[DISPATCH_INVULN] Called with condition_id='{}', target_pos={:?}, source_pos={:?}, attacking_active_move='{:?}'",
        condition_id, target_pos, source_pos, attacking_active_move);

    let result = match condition_id {
        "skydrop" => {
            println!("[DISPATCH_INVULN] Routing to skydrop callback");
            skydrop::on_any_invulnerability(battle, target_pos, source_pos, attacking_active_move)
        },
        _ => {
            println!("[DISPATCH_INVULN] No match for '{}', trying fallback", condition_id);
            // Fallback to move-embedded condition callbacks
            move_callbacks::dispatch_condition_on_any_invulnerability(battle, condition_id, target_pos, source_pos, attacking_active_move)
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
        // Bide locks the user into using Bide
        // JavaScript: onLockMove: 'bide'
        "bide" => EventResult::String("bide".to_string()),
        // Uproar locks the user into using Uproar
        // JavaScript: onLockMove: 'uproar'
        "uproar" => EventResult::String("uproar".to_string()),
        // Ice Ball locks the user into using Ice Ball
        // JavaScript: onLockMove: 'iceball'
        "iceball" => EventResult::String("iceball".to_string()),
        // Rollout locks the user into using Rollout
        // JavaScript: onLockMove: 'rollout'
        "rollout" => EventResult::String("rollout".to_string()),
        // Mustrecharge locks the user into using Recharge
        // JavaScript: onLockMove: 'recharge'
        "mustrecharge" => EventResult::String("recharge".to_string()),
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
    _active_move: Option<&ActiveMove>,
) -> EventResult {
    use crate::data::ability_callbacks::protosynthesis;
    use crate::data::ability_callbacks::quarkdrive;

    match condition_id {
        "snowscape" => snowscape::on_modify_def(battle, _def, pokemon_pos, _target_pos, _source_pos, _active_move),
        // Quark Drive/Protosynthesis expect non-Option positions; attacker pos is unused
        "quarkdrive" => quarkdrive::condition::on_modify_def(battle, _def, pokemon_pos, _source_pos.unwrap_or((0, 0)), _active_move),
        "protosynthesis" => protosynthesis::condition::on_modify_def(battle, _def, pokemon_pos, _source_pos.unwrap_or((0, 0)), _active_move),
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
    _active_move: Option<&ActiveMove>,
) -> EventResult {
    use crate::data::ability_callbacks::protosynthesis;
    use crate::data::ability_callbacks::quarkdrive;

    match condition_id {
        "sandstorm" => sandstorm::on_modify_sp_d(battle, _spd, pokemon_pos, _target_pos, _source_pos, _active_move),
        // Quark Drive/Protosynthesis expect non-Option positions; attacker pos is unused
        "quarkdrive" => quarkdrive::condition::on_modify_sp_d(battle, _spd, pokemon_pos, _source_pos.unwrap_or((0, 0)), _active_move),
        "protosynthesis" => protosynthesis::condition::on_modify_sp_d(battle, _spd, pokemon_pos, _source_pos.unwrap_or((0, 0)), _active_move),
        _ => EventResult::Continue,
    }
}
/// Dispatch onModifyType callbacks
/// Called when a Pokemon's move type may be modified by a condition (e.g., Electrify)
pub fn dispatch_on_modify_type(
    battle: &mut Battle,
    condition_id: &str,
    _pokemon_pos: (usize, usize),
    active_move: Option<&ActiveMove>,
) -> EventResult {
    match condition_id {
        "electrify" => move_callbacks::electrify::condition::on_modify_type(battle, active_move),
        "iondeluge" => move_callbacks::iondeluge::condition::on_modify_type(battle, active_move),
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
        "quarkdrive" => ability_callbacks::quarkdrive::condition::on_modify_spe(battle, spe, pokemon_pos),
        "protosynthesis" => ability_callbacks::protosynthesis::condition::on_modify_spe(battle, spe, pokemon_pos),
        "unburden" => ability_callbacks::unburden::condition::on_modify_spe(battle, spe, pokemon_pos),
        _ => {
            // Fallback to move-embedded condition callbacks
            // dispatch_condition_on_modify_spe takes (battle, move_id, spe, source_pos)
            move_callbacks::dispatch_condition_on_modify_spe(battle, condition_id, spe, pokemon_pos)
        }
    }
}

/// Dispatch onModifyAccuracy callbacks for conditions (like Gravity)
/// JavaScript: condition.onModifyAccuracy(accuracy, target, source, move)
pub fn dispatch_on_modify_accuracy(
    battle: &mut Battle,
    condition_id: &str,
    accuracy: i32,
    _target_pos: (usize, usize),
    _source_pos: Option<(usize, usize)>,
) -> EventResult {
    use crate::data::move_callbacks::gravity;

    match condition_id {
        "gravity" => gravity::condition::on_modify_accuracy(battle, accuracy),
        _ => {
            // Fallback to move-embedded condition callbacks
            move_callbacks::dispatch_condition_on_modify_accuracy(battle, None, accuracy)
        }
    }
}

/// Dispatch onModifySpA callbacks for conditions
pub fn dispatch_on_modify_sp_a(
    battle: &mut Battle,
    condition_id: &str,
    spa: i32,
    pokemon_pos: (usize, usize),
    target_pos: (usize, usize),
    active_move: Option<&ActiveMove>,
) -> EventResult {
    use crate::data::ability_callbacks::flashfire;
    use crate::data::ability_callbacks::protosynthesis;
    use crate::data::ability_callbacks::quarkdrive;

    match condition_id {
        "flashfire" => flashfire::condition::on_modify_sp_a(battle, spa, pokemon_pos, target_pos, active_move),
        "protosynthesis" => protosynthesis::condition::on_modify_sp_a(battle, spa, pokemon_pos, target_pos, active_move),
        "quarkdrive" => quarkdrive::condition::on_modify_sp_a(battle, spa, pokemon_pos, target_pos, active_move),
        _ => EventResult::Continue
    }
}

/// Dispatch onModifyAtk callbacks for conditions
pub fn dispatch_on_modify_atk(
    battle: &mut Battle,
    condition_id: &str,
    atk: i32,
    pokemon_pos: (usize, usize),
    target_pos: (usize, usize),
    active_move: Option<&ActiveMove>,
) -> EventResult {
    use crate::data::ability_callbacks::flashfire;
    use crate::data::ability_callbacks::protosynthesis;
    use crate::data::ability_callbacks::quarkdrive;

    match condition_id {
        "flashfire" => flashfire::condition::on_modify_atk(battle, atk, pokemon_pos, target_pos, active_move),
        "protosynthesis" => protosynthesis::condition::on_modify_atk(battle, atk, pokemon_pos, target_pos, active_move),
        "quarkdrive" => quarkdrive::condition::on_modify_atk(battle, atk, pokemon_pos, target_pos, active_move),
        _ => EventResult::Continue
    }
}

/// Dispatch onMoveAborted callbacks
pub fn dispatch_on_move_aborted(
    battle: &mut Battle,
    condition_id: &str,
    pokemon_pos: (usize, usize),
    target_pos: Option<(usize, usize)>,
    active_move: Option<&ActiveMove>,
) -> EventResult {
    match condition_id {
        "twoturnmove" => twoturnmove::on_move_aborted(battle, pokemon_pos, target_pos, active_move),
        _ => {
            // Fallback to move-embedded condition callbacks
            move_callbacks::dispatch_condition_on_move_aborted(battle, active_move, pokemon_pos)
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
/// Check if a condition has an onRestart callback
/// JavaScript: if (!status.onRestart) return false;
/// This is needed because JavaScript checks for callback existence before calling singleEvent
pub fn has_on_restart_callback(condition_id: &str) -> bool {
    match condition_id {
        "lockedmove" | "stall" => true,
        // Move-embedded conditions with onRestart
        "rollout" | "iceball" | "uproar" | "focusenergy" | "laserfocus" | "stockpile" |
        "gmaxchistrike" | "minimize" | "charge" | "healblock" | "smackdown" |
        "powertrick" | "powershift" | "helpinghand" | "furycutter" | "defensecurl" | "allyswitch" => true,
        _ => false,
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
    active_move: Option<&ActiveMove>,
) -> EventResult {
    match condition_id {
        "dynamax" => dynamax::on_source_modify_damage(battle, _damage, source_pos, target_pos, active_move),
        _ => {
            // Fallback to move-embedded condition callbacks
            move_callbacks::dispatch_condition_on_source_modify_damage(battle, condition_id, active_move, source_pos, target_pos)
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
    effect: Option<&crate::battle::Effect>,
) -> EventResult {
    // Extract effect_id for callbacks that still use the string form
    let effect_id = effect.map(|e| e.id.as_str());
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
            // Try move-embedded condition callbacks (nightmare, etc.)
            use crate::data::move_callbacks;
            move_callbacks::dispatch_condition_on_start(battle, condition_id, pokemon_pos, source_pos, effect)
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
/// Dispatch onTryHeal callbacks
pub fn dispatch_on_try_heal(
    battle: &mut Battle,
    condition_id: &str,
    damage: i32,
    target_pos: Option<(usize, usize)>,
    source_pos: Option<(usize, usize)>,
    effect_id: Option<&str>,
) -> EventResult {
    match condition_id {
        // Move-embedded conditions with onTryHeal callbacks
        "healblock" => move_callbacks::healblock::condition::on_try_heal(battle, damage, target_pos, source_pos, effect_id),
        _ => EventResult::Continue,
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
        // Move-embedded conditions with onTryAddVolatile callbacks
        "electricterrain" => move_callbacks::electricterrain::condition::on_try_add_volatile(battle, status, target_pos),
        "focuspunch" => move_callbacks::focuspunch::condition::on_try_add_volatile(battle, status, target_pos.unwrap_or((0,0))),
        "mistyterrain" => move_callbacks::mistyterrain::condition::on_try_add_volatile(battle, status, target_pos, source_pos, effect_id),
        "safeguard" => move_callbacks::safeguard::condition::on_try_add_volatile(battle, status, target_pos, source_pos, effect_id),
        _ => EventResult::Continue,
    }
}
/// Dispatch onTryMove callbacks
pub fn dispatch_on_try_move(
    battle: &mut Battle,
    condition_id: &str,
    pokemon_pos: (usize, usize),
    target_pos: Option<(usize, usize)>,
    active_move: Option<&ActiveMove>,
) -> EventResult {
    match condition_id {
        "desolateland" => desolateland::on_try_move(battle, pokemon_pos, target_pos, active_move),
        "primordialsea" => primordialsea::on_try_move(battle, pokemon_pos, target_pos, active_move),
        _ => {
            // Fallback to move-embedded condition callbacks
            // Pass the condition_id so the move-embedded callback knows which condition to dispatch
            move_callbacks::dispatch_condition_on_try_move(battle, active_move, pokemon_pos, target_pos, Some(condition_id))
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
            move_callbacks::dispatch_condition_on_type(battle, None, pokemon_pos, types)
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
    active_move: Option<&ActiveMove>,
) -> EventResult {
    match condition_id {
        "desolateland" => desolateland::on_weather_modify_damage(battle, damage, attacker_pos, defender_pos, active_move),
        "primordialsea" => primordialsea::on_weather_modify_damage(battle, damage, attacker_pos, defender_pos, active_move),
        "raindance" => raindance::on_weather_modify_damage(battle, damage, attacker_pos, defender_pos, active_move),
        "sunnyday" => sunnyday::on_weather_modify_damage(battle, damage, attacker_pos, defender_pos, active_move),
        _ => EventResult::Continue,
    }
}
/// Dispatch onTryHit callbacks (with source and target)
pub fn dispatch_on_try_hit(
    battle: &mut Battle,
    condition_id: &str,
    source_pos: (usize, usize),
    target_pos: (usize, usize),
    active_move: Option<&ActiveMove>,
) -> EventResult {
    // Route to actual implementation in move_callbacks
    use crate::data::move_callbacks;
    move_callbacks::dispatch_condition_on_try_hit(battle, active_move, source_pos, target_pos, Some(condition_id))
}
/// Dispatch onTryPrimaryHit callbacks
pub fn dispatch_on_try_primary_hit(
    battle: &mut Battle,
    condition_id: &str,
    target_pos: (usize, usize),
    source_pos: Option<(usize, usize)>,
    active_move: Option<&ActiveMove>,
) -> EventResult {
    // Route to actual implementation in move_callbacks
    use crate::data::move_callbacks;
    move_callbacks::dispatch_condition_on_try_primary_hit(battle, condition_id, target_pos, source_pos, active_move)
}

/// Dispatch onUpdate callbacks
/// This handles volatiles added by moves like fling that need to execute onUpdate
pub fn dispatch_on_update(
    battle: &mut Battle,
    condition_id: &str,
    pokemon_pos: (usize, usize),
    _active_move: Option<&ActiveMove>,
) -> EventResult {
    // Route to move-embedded condition callbacks
    // Note: dispatch_condition_on_update expects (battle, active_move, source_pos)
    // but we have the condition_id. We need to match by condition_id, not move_id.
    match condition_id {
        "fling" => crate::data::move_callbacks::fling::condition::on_update(battle, pokemon_pos),
        "attract" => crate::data::move_callbacks::attract::condition::on_update(battle, pokemon_pos),
        "syrupbomb" => crate::data::move_callbacks::syrupbomb::condition::on_update(battle, pokemon_pos),
        "telekinesis" => crate::data::move_callbacks::telekinesis::condition::on_update(battle, pokemon_pos),
        _ => EventResult::Continue,
    }
}

/// Dispatch onSetStatus callbacks for terrain/weather conditions
/// Called when a status is being set on a Pokemon and terrain/weather might prevent it
pub fn dispatch_on_set_status(
    battle: &mut Battle,
    condition_id: &str,
    status: Option<&str>,
    target_pos: Option<(usize, usize)>,
    source_pos: Option<(usize, usize)>,
    effect_id: Option<&str>,
) -> EventResult {
    match condition_id {
        "electricterrain" => {
            crate::data::move_callbacks::electricterrain::condition::on_set_status(
                battle, status, target_pos, source_pos, effect_id,
            )
        }
        "mistyterrain" => {
            crate::data::move_callbacks::mistyterrain::condition::on_set_status(
                battle, status, target_pos, source_pos, effect_id,
            )
        }
        "safeguard" => {
            crate::data::move_callbacks::safeguard::condition::on_set_status(
                battle, status, target_pos, source_pos, effect_id,
            )
        }
        _ => EventResult::Continue,
    }
}

/// Dispatch onAnyPrepareHit callbacks for volatiles like Snatch
/// Called when any Pokemon is about to use a move that can be snatched
pub fn dispatch_on_any_prepare_hit(
    battle: &mut Battle,
    condition_id: &str,
    source_pos: Option<(usize, usize)>,
    target_pos: Option<(usize, usize)>,
    active_move: Option<&ActiveMove>,
) -> EventResult {
    match condition_id {
        "snatch" => {
            move_callbacks::snatch::condition::on_any_prepare_hit(battle, source_pos, target_pos, active_move)
        }
        _ => EventResult::Continue,
    }
}

