// NOTE: This method is NOT in JavaScript - Rust-specific implementation

use crate::*;

impl Battle {

    /// Handle condition events (status, volatile, weather, terrain)
    /// Rust helper method - JavaScript's singleEvent() directly invokes condition[`on${eventId}`] callbacks
    /// This method dispatches to condition_callbacks module based on event name
    /// Routes to condition-specific handlers for all event types (Residual, BeforeMove, Weather, etc.)
    ///
    /// Event names are normalized by removing "on" prefix if present.
    /// JavaScript fires events as "TryPrimaryHit" but callbacks are named "onTryPrimaryHit".
    /// We consistently use the non-prefixed version for matching.
    pub fn handle_condition_event(
        &mut self,
        event_id: &str,
        condition_id: &str,
        target: Option<&crate::event::EventTarget>,
    ) -> crate::event::EventResult {
        use crate::data::condition_callbacks;
        use crate::event::EventResult;

        // Extract pokemon position from EventTarget
        let pokemon_pos = target.and_then(|t| t.as_pokemon()).unwrap_or((0, 0));

        if event_id.contains("Invulnerability") {
            eprintln!("[HANDLE_CONDITION_EVENT] event_id={}, condition_id={}, target={:?}",
                event_id, condition_id, Some(pokemon_pos));
        }

        // Normalize event name by removing "on" prefix if present
        let normalized_event = if event_id.starts_with("on") {
            &event_id[2..]
        } else {
            event_id
        };

        if event_id.contains("Invulnerability") {
            eprintln!("[HANDLE_CONDITION_EVENT] normalized_event={}", normalized_event);
        }

        // Try condition_callbacks first
        let result = match normalized_event {
            "AfterMove" => {
                condition_callbacks::dispatch_on_after_move(self, condition_id, pokemon_pos)
            }
            "AfterMoveSecondary" => condition_callbacks::dispatch_on_after_move_secondary(
                self,
                condition_id,
                pokemon_pos,
            ),
            "BasePower" => {
                condition_callbacks::dispatch_on_base_power(self, condition_id, pokemon_pos)
            }
            "BeforeMove" => {
                condition_callbacks::dispatch_on_before_move(self, condition_id, pokemon_pos)
            }
            "FoeBeforeMove" => {
                condition_callbacks::dispatch_on_foe_before_move(self, condition_id, pokemon_pos)
            }
            "AllyBeforeMove" => {
                condition_callbacks::dispatch_on_ally_before_move(self, condition_id, pokemon_pos)
            }
            "SourceBeforeMove" => {
                condition_callbacks::dispatch_on_source_before_move(self, condition_id, pokemon_pos)
            }
            "BeforeSwitchOut" => {
                condition_callbacks::dispatch_on_before_switch_out(self, condition_id, pokemon_pos)
            }
            "BeforeTurn" => {
                condition_callbacks::dispatch_on_before_turn(self, condition_id, pokemon_pos)
            }
            "DamagingHit" => {
                condition_callbacks::dispatch_on_damaging_hit(self, condition_id, pokemon_pos)
            }
            "DisableMove" => {
                condition_callbacks::dispatch_on_disable_move(self, condition_id, pokemon_pos)
            }
            "DragOut" => condition_callbacks::dispatch_on_drag_out(self, condition_id, pokemon_pos),
            "Effectiveness" => {
                condition_callbacks::dispatch_on_effectiveness(self, condition_id, pokemon_pos)
            }
            "End" => condition_callbacks::dispatch_on_end(self, condition_id, pokemon_pos),
            "Faint" => {
                // Faint needs target, source, and effect from current_event
                // target_pos is the pokemon that fainted (pokemon_pos)
                // source_pos is the pokemon that caused the faint (from current_event)
                // effect_id is the move/ability/item that caused the faint (from current_event)
                // Extract values first to avoid borrow checker issues
                let source_pos = self.current_event.as_ref().and_then(|e| e.source);
                let effect_id_owned = self.current_event.as_ref()
                    .and_then(|e| e.effect.as_ref())
                    .map(|id| id.to_string());

                condition_callbacks::dispatch_on_faint(
                    self,
                    condition_id,
                    Some(pokemon_pos),
                    source_pos,
                    effect_id_owned.as_deref(),
                )
            }
            "FieldEnd" => {
                condition_callbacks::dispatch_on_field_end(self, condition_id, pokemon_pos)
            }
            "FieldResidual" => {
                condition_callbacks::dispatch_on_field_residual(self, condition_id, pokemon_pos)
            }
            "FieldStart" => {
                condition_callbacks::dispatch_on_field_start(self, condition_id, pokemon_pos)
            }
            "FieldRestart" => {
                condition_callbacks::dispatch_on_field_restart(self, condition_id, pokemon_pos)
            }
            "Immunity" => {
                condition_callbacks::dispatch_on_immunity(self, condition_id, pokemon_pos)
            }
            "LockMove" => {
                condition_callbacks::dispatch_on_lock_move(self, condition_id, pokemon_pos)
            }
            "ModifyDef" => {
                condition_callbacks::dispatch_on_modify_def(self, condition_id, pokemon_pos)
            }
            "ModifyMove" => {
                condition_callbacks::dispatch_on_modify_move(self, condition_id, pokemon_pos)
            }
            "ModifySpD" => {
                condition_callbacks::dispatch_on_modify_sp_d(self, condition_id, pokemon_pos)
            }
            "ModifySpe" => {
                let spe = self.event.as_ref().and_then(|e| match &e.relay_var {
                    Some(EventResult::Number(n)) => Some(*n),
                    _ => None
                }).unwrap_or(0);
                condition_callbacks::dispatch_on_modify_spe(self, condition_id, spe, pokemon_pos)
            }
            "MoveAborted" => {
                condition_callbacks::dispatch_on_move_aborted(self, condition_id, pokemon_pos)
            }
            "Residual" => {
                condition_callbacks::dispatch_on_residual(self, condition_id, pokemon_pos)
            }
            "Restart" => condition_callbacks::dispatch_on_restart(self, condition_id, pokemon_pos),
            "SourceModifyDamage" => condition_callbacks::dispatch_on_source_modify_damage(
                self,
                condition_id,
                pokemon_pos,
            ),
            "StallMove" => {
                condition_callbacks::dispatch_on_stall_move(self, condition_id, pokemon_pos)
            }
            "Start" => condition_callbacks::dispatch_on_start(self, condition_id, pokemon_pos),
            "SwitchIn" => {
                condition_callbacks::dispatch_on_switch_in(self, condition_id, pokemon_pos)
            }
            "TrapPokemon" => {
                condition_callbacks::dispatch_on_trap_pokemon(self, condition_id, pokemon_pos)
            }
            "TryAddVolatile" => {
                condition_callbacks::dispatch_on_try_add_volatile(self, condition_id, pokemon_pos)
            }
            "TryPrimaryHit" => {
                condition_callbacks::dispatch_on_try_primary_hit(self, condition_id, pokemon_pos)
            }
            "TryHit" => {
                // TryHit needs both source and target positions
                // Get source from current_event
                let source_pos = self.current_event.as_ref().and_then(|e| e.source).unwrap_or((0, 0));
                condition_callbacks::dispatch_on_try_hit(self, condition_id, source_pos, pokemon_pos)
            }
            "TryMove" => condition_callbacks::dispatch_on_try_move(self, condition_id, pokemon_pos),
            "Type" => condition_callbacks::dispatch_on_type(self, condition_id, pokemon_pos),
            "Weather" => {
                condition_callbacks::dispatch_on_weather(self, condition_id, pokemon_pos)
            }
            "WeatherModifyDamage" => condition_callbacks::dispatch_on_weather_modify_damage(
                self,
                condition_id,
                pokemon_pos,
            ),
            "AnyInvulnerability" | "Invulnerability" => {
                condition_callbacks::dispatch_on_any_invulnerability(self, condition_id, pokemon_pos)
            }
            _ => EventResult::Continue,
        };

        result
    }
}
