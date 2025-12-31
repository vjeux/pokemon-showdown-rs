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
pub fn dispatch_on_before_move(
    _battle: &mut Battle,
    _condition_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
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
        _ => EventResult::Continue,
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

/// Dispatch onStallMove callbacks
pub fn dispatch_on_stall_move(
    _battle: &mut Battle,
    _condition_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onStart callbacks
pub fn dispatch_on_start(
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
    _battle: &mut Battle,
    _condition_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}
