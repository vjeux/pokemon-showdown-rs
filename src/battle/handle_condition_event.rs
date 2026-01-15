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

        // Clone active_move to avoid borrow issues
        let active_move_clone = self.active_move.clone();

        // Extract pokemon position from EventTarget
        // If target is None (for field handlers), fall back to event.target
        // This is important for Weather events where the handler is a field handler
        // but the target is the Pokemon passed to runEvent
        let pokemon_pos = target
            .and_then(|t| t.as_pokemon())
            .or_else(|| self.event.as_ref().and_then(|e| e.target))
            .unwrap_or((0, 0));

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
                // Extract target from event and move_id from active_move
                let target_pos = self.event.as_ref().and_then(|e| e.target);
                let _move_id = self.active_move.as_ref()
                    .map(|m| m.id.to_string())
                    .unwrap_or_default();
                condition_callbacks::dispatch_on_after_move(self, condition_id, pokemon_pos, target_pos, active_move_clone.as_ref())
            }
            "AfterMoveSecondary" => {
                // Extract source from event and move_id from active_move
                let source_pos = self.event.as_ref().and_then(|e| e.source);
                condition_callbacks::dispatch_on_after_move_secondary(
                    self,
                    condition_id,
                    pokemon_pos,
                    source_pos,
                    active_move_clone.as_ref()
                )
            }
            "Accuracy" => {
                // Extract accuracy from relay_var, source from event
                // onAccuracy is called on volatiles of the target to check if move hits
                // Glaiverush returns true to make moves always hit
                //
                // CRITICAL: If relay_var is Boolean(true), a previous handler (like No Guard)
                // has already determined the move will hit. In JavaScript, the handler would
                // receive `true` as the accuracy parameter and return it unchanged.
                // We should preserve this behavior by returning Boolean(true) immediately.
                let relay_var = self.event.as_ref().and_then(|e| e.relay_var.clone());
                if matches!(relay_var, Some(EventResult::Boolean(true))) {
                    return EventResult::Boolean(true);
                }
                let accuracy = match &relay_var {
                    Some(EventResult::Number(n)) => *n,
                    _ => 0
                };
                let source_pos = self.event.as_ref().and_then(|e| e.source);
                condition_callbacks::dispatch_on_accuracy(
                    self,
                    condition_id,
                    accuracy,
                    Some(pokemon_pos),
                    source_pos,
                    active_move_clone.as_ref()
                )
            }
            "SourceAccuracy" => {
                // Extract accuracy from relay_var, target from event
                // onSourceAccuracy is called on volatiles of the SOURCE Pokemon
                // Lock-On/Mind Reader return true to bypass accuracy checks
                //
                // CRITICAL: If relay_var is Boolean(true), a previous handler (like No Guard)
                // has already determined the move will hit. Return Boolean(true) immediately.
                let relay_var = self.event.as_ref().and_then(|e| e.relay_var.clone());
                if matches!(relay_var, Some(EventResult::Boolean(true))) {
                    return EventResult::Boolean(true);
                }
                let accuracy = match &relay_var {
                    Some(EventResult::Number(n)) => *n,
                    _ => 0
                };
                let target_pos = self.event.as_ref().and_then(|e| e.target);
                condition_callbacks::dispatch_on_source_accuracy(
                    self,
                    condition_id,
                    accuracy,
                    target_pos,
                    Some(pokemon_pos), // pokemon_pos is the source (effect_holder)
                    active_move_clone.as_ref()
                )
            }
            "BasePower" => {
                // Extract base_power from relay_var and defender from event.source
                // JavaScript: onBasePower(basePower, attacker, defender, move)
                // In run_event:
                //   event.target = attacker (from runEvent's 2nd param = source_pos in get_damage)
                //   event.source = defender (from runEvent's 3rd param = target_pos in get_damage)
                // Note: The parameter names in run_event are swapped from JavaScript convention
                let base_power = self.event.as_ref().and_then(|e| match &e.relay_var {
                    Some(EventResult::Number(n)) => Some(*n),
                    _ => None
                }).unwrap_or(0);
                // Use event.source for defender (not event.target which is the attacker)
                let defender_pos = self.event.as_ref().and_then(|e| e.source);
                let _move_id = self.active_move.as_ref()
                    .map(|m| m.id.to_string())
                    .unwrap_or_default();
                condition_callbacks::dispatch_on_base_power(self, condition_id, base_power, pokemon_pos, defender_pos, active_move_clone.as_ref())
            }
            "BeforeMove" => {
                // Extract target from event and move_id from active_move
                let target_pos = self.event.as_ref().and_then(|e| e.target);
                let _move_id = self.active_move.as_ref()
                    .map(|m| m.id.to_string())
                    .unwrap_or_default();
                condition_callbacks::dispatch_on_before_move(self, condition_id, pokemon_pos, target_pos, active_move_clone.as_ref())
            }
            "FoeBeforeMove" => {
                // Extract target from event, source, and move_id from active_move
                let target_pos = self.event.as_ref().and_then(|e| e.target);
                let source_pos = self.event.as_ref().and_then(|e| e.source);
                let _move_id = self.active_move.as_ref()
                    .map(|m| m.id.to_string())
                    .unwrap_or_default();
                condition_callbacks::dispatch_on_foe_before_move(self, condition_id, pokemon_pos, target_pos, source_pos, active_move_clone.as_ref())
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
            "CriticalHit" => {
                // Called to check if a critical hit should occur
                // Lucky Chant returns false to prevent critical hits
                let source_pos = self.event.as_ref().and_then(|e| e.source);
                condition_callbacks::dispatch_side_condition_on_critical_hit(
                    self,
                    condition_id,
                    Some(pokemon_pos),
                    source_pos,
                    active_move_clone.as_ref()
                )
            }
            "TryBoost" => {
                // Called when stat boosts are being applied
                // Mist prevents negative boosts from opponent's moves
                let source_pos = self.event.as_ref().and_then(|e| e.source);
                let effect_id = self.event.as_ref()
                    .and_then(|e| e.effect.as_ref())
                    .map(|eff| eff.id.as_str().to_string());
                condition_callbacks::dispatch_side_condition_on_try_boost(
                    self,
                    condition_id,
                    Some(pokemon_pos),
                    source_pos,
                    effect_id.as_deref()
                )
            }
            "DamagingHit" => {
                // Extract damage from relay_var, source from event, and move_id from active_move
                let damage = self.event.as_ref().and_then(|e| match &e.relay_var {
                    Some(EventResult::Number(n)) => Some(*n),
                    _ => None
                }).unwrap_or(0);
                let source_pos = self.event.as_ref().and_then(|e| e.source);
                let _move_id = self.active_move.as_ref()
                    .map(|m| m.id.to_string())
                    .unwrap_or_default();
                eprintln!("[HANDLE_CONDITION_EVENT] DamagingHit: condition_id={}, damage={}, pokemon_pos={:?}, source_pos={:?}", condition_id, damage, pokemon_pos, source_pos);
                condition_callbacks::dispatch_on_damaging_hit(self, condition_id, damage, pokemon_pos, source_pos, active_move_clone.as_ref())
            }
            "Damage" => {
                // Called during spread_damage to modify damage before applying
                // JavaScript: onDamage(damage, target, source, effect)
                // Conditions like Endure use this to prevent fainting
                let damage = self.event.as_ref().and_then(|e| match &e.relay_var {
                    Some(EventResult::Number(n)) => Some(*n),
                    _ => None
                }).unwrap_or(0);
                let source_pos = self.event.as_ref().and_then(|e| e.source);
                let effect_id = self.event.as_ref()
                    .and_then(|e| e.effect.as_ref())
                    .map(|eff| eff.id.as_str().to_string());
                // Check if the effect is a Move type
                // JavaScript: if (!move || move.effectType !== 'Move' || !source) return;
                let is_move_effect = self.event.as_ref()
                    .and_then(|e| e.effect.as_ref())
                    .map(|eff| eff.effect_type == crate::battle::EffectType::Move)
                    .unwrap_or(false);
                condition_callbacks::dispatch_on_damage(self, condition_id, damage, pokemon_pos, source_pos, effect_id.as_deref(), is_move_effect)
            }
            "DisableMove" => {
                condition_callbacks::dispatch_on_disable_move(self, condition_id, pokemon_pos)
            }
            "FoeDisableMove" => {
                // For FoeDisableMove, the callback receives the target of the event (the Pokemon whose moves should be disabled)
                // NOT the Pokemon that has the handler (effect holder)
                // JavaScript: onFoeDisableMove(pokemon) - pokemon is the target of the DisableMove event
                let target_pos = self.event.as_ref().and_then(|e| e.target).unwrap_or(pokemon_pos);
                condition_callbacks::dispatch_on_foe_disable_move(self, condition_id, target_pos)
            }
            "DragOut" => {
                // Extract source from event and move_id from active_move
                let source_pos = self.event.as_ref().and_then(|e| e.source);
                let _move_id = self.active_move.as_ref()
                    .map(|m| m.id.to_string())
                    .unwrap_or_default();
                condition_callbacks::dispatch_on_drag_out(self, condition_id, pokemon_pos, source_pos, active_move_clone.as_ref())
            }
            "Effectiveness" => {
                // Effectiveness needs type_mod, target_type, and move_id
                // Extract type_mod from relay_var (type effectiveness modifier as number)
                let type_mod = self.event.as_ref().and_then(|e| match &e.relay_var {
                    Some(EventResult::Number(n)) => Some(*n),
                    _ => None
                }).unwrap_or(0);

                // Extract target_type from type_param (the type being checked)
                let target_type = self.event.as_ref()
                    .and_then(|e| e.type_param.clone())
                    .unwrap_or_default();

                // Extract move_id from active_move
                let _move_id = self.active_move.as_ref()
                    .map(|m| m.id.to_string())
                    .unwrap_or_default();

                condition_callbacks::dispatch_on_effectiveness(self, condition_id, type_mod, &target_type, pokemon_pos, active_move_clone.as_ref())
            }
            "End" => {
                // Try ability-embedded condition callbacks first (for abilities like Zen Mode)
                // These have side effects even when returning Continue, so we always call them
                crate::data::ability_callbacks::dispatch_condition_on_end(self, condition_id, pokemon_pos);
                // Then try standalone/move-embedded condition callbacks
                condition_callbacks::dispatch_on_end(self, condition_id, pokemon_pos)
            }
            "Faint" => {
                // Faint needs target, source, and effect from event
                // target_pos is the pokemon that fainted (pokemon_pos)
                // source_pos is the pokemon that caused the faint (from event)
                // effect_id is the move/ability/item that caused the faint (from event)
                // Extract values first to avoid borrow checker issues
                let source_pos = self.event.as_ref().and_then(|e| e.source);
                let effect_id_owned = self.event.as_ref()
                    .and_then(|e| e.effect.as_ref())
                    .map(|eff| eff.id.to_string());

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
                // Extract target and source from event
                let target_pos = self.event.as_ref().and_then(|e| e.target);
                let source_pos = self.event.as_ref().and_then(|e| e.source);
                condition_callbacks::dispatch_on_field_restart(self, condition_id, pokemon_pos, target_pos, source_pos)
            }
            "Immunity" => {
                // Extract immunity type from event relay_var (passed by run_status_immunity)
                // JavaScript: runEvent('Immunity', this, null, null, type)
                // The type is passed as the relay_var (5th parameter)
                let immunity_type = self.event.as_ref()
                    .and_then(|e| e.relay_var.as_ref())
                    .and_then(|rv| match rv {
                        EventResult::String(s) => Some(s.clone()),
                        _ => None
                    })
                    .unwrap_or_default();
                condition_callbacks::dispatch_on_immunity(self, condition_id, &immunity_type, pokemon_pos)
            }
            "LockMove" => {
                condition_callbacks::dispatch_on_lock_move(self, condition_id, pokemon_pos)
            }
            "ModifyDef" => {
                // Extract def from relay_var, target, source, and move_id
                let def = self.event.as_ref().and_then(|e| match &e.relay_var {
                    Some(EventResult::Number(n)) => Some(*n),
                    _ => None
                }).unwrap_or(0);
                let target_pos = self.event.as_ref().and_then(|e| e.target);
                let source_pos = self.event.as_ref().and_then(|e| e.source);
                let _move_id = self.active_move.as_ref()
                    .map(|m| m.id.to_string())
                    .unwrap_or_default();
                condition_callbacks::dispatch_on_modify_def(self, condition_id, def, pokemon_pos, target_pos, source_pos, active_move_clone.as_ref())
            }
            "ModifyMove" => {
                // Extract target from event
                let target_pos = self.event.as_ref().and_then(|e| e.target);
                condition_callbacks::dispatch_on_modify_move(self, condition_id, pokemon_pos, target_pos)
            }
            "ModifyType" => {
                // Electrify and Ion Deluge modify move types
                condition_callbacks::dispatch_on_modify_type(self, condition_id, pokemon_pos, active_move_clone.as_ref())
            }
            "ModifySpD" => {
                // Extract spd from relay_var, target, source, and move_id
                let spd = self.event.as_ref().and_then(|e| match &e.relay_var {
                    Some(EventResult::Number(n)) => Some(*n),
                    _ => None
                }).unwrap_or(0);
                let target_pos = self.event.as_ref().and_then(|e| e.target);
                let source_pos = self.event.as_ref().and_then(|e| e.source);
                let _move_id = self.active_move.as_ref()
                    .map(|m| m.id.to_string())
                    .unwrap_or_default();
                condition_callbacks::dispatch_on_modify_sp_d(self, condition_id, spd, pokemon_pos, target_pos, source_pos, active_move_clone.as_ref())
            }
            "ModifySpA" => {
                // Extract spa from relay_var, target, source
                let spa = self.event.as_ref().and_then(|e| match &e.relay_var {
                    Some(EventResult::Number(n)) => Some(*n),
                    _ => None
                }).unwrap_or(0);
                let target_pos = self.event.as_ref().and_then(|e| e.target);
                let _source_pos = self.event.as_ref().and_then(|e| e.source);
                condition_callbacks::dispatch_on_modify_sp_a(self, condition_id, spa, pokemon_pos, target_pos.unwrap_or(pokemon_pos), active_move_clone.as_ref())
            }
            "ModifyAtk" => {
                // Extract atk from relay_var, target, source
                let atk = self.event.as_ref().and_then(|e| match &e.relay_var {
                    Some(EventResult::Number(n)) => Some(*n),
                    _ => None
                }).unwrap_or(0);
                let target_pos = self.event.as_ref().and_then(|e| e.target);
                let _source_pos = self.event.as_ref().and_then(|e| e.source);
                condition_callbacks::dispatch_on_modify_atk(self, condition_id, atk, pokemon_pos, target_pos.unwrap_or(pokemon_pos), active_move_clone.as_ref())
            }
            "ModifySpe" => {
                let spe = self.event.as_ref().and_then(|e| match &e.relay_var {
                    Some(EventResult::Number(n)) => Some(*n),
                    _ => None
                }).unwrap_or(0);
                condition_callbacks::dispatch_on_modify_spe(self, condition_id, spe, pokemon_pos)
            }
            "ModifyAccuracy" => {
                // Gravity's onModifyAccuracy boosts accuracy of all moves by 5/3
                let accuracy = self.event.as_ref().and_then(|e| match &e.relay_var {
                    Some(EventResult::Number(n)) => Some(*n),
                    _ => None
                }).unwrap_or(0);
                let source_pos = self.event.as_ref().and_then(|e| e.source);
                condition_callbacks::dispatch_on_modify_accuracy(self, condition_id, accuracy, pokemon_pos, source_pos)
            }
            "ModifyCritRatio" => {
                // Extract crit_ratio from relay_var and source_pos from event
                // gmaxchistrike, focusenergy, dragoncheer, laserfocus - move-embedded conditions
                let crit_ratio = self.event.as_ref().and_then(|e| match &e.relay_var {
                    Some(EventResult::Number(n)) => Some(*n),
                    _ => None
                }).unwrap_or(0);
                let source_pos = self.event.as_ref().and_then(|e| e.source);
                crate::data::move_callbacks::dispatch_condition_on_modify_crit_ratio(
                    self,
                    condition_id,
                    crit_ratio,
                    source_pos,
                )
            }
            "MoveAborted" => {
                // Extract target from event and move_id from active_move
                let target_pos = self.event.as_ref().and_then(|e| e.target);
                let _move_id = self.active_move.as_ref()
                    .map(|m| m.id.to_string())
                    .unwrap_or_default();
                condition_callbacks::dispatch_on_move_aborted(self, condition_id, pokemon_pos, target_pos, active_move_clone.as_ref())
            }
            "Residual" => {
                // Extract source and effect from event
                let source_pos = self.event.as_ref().and_then(|e| e.source);
                let effect_id_owned = self.event.as_ref()
                    .and_then(|e| e.effect.as_ref())
                    .map(|eff| eff.id.to_string());
                condition_callbacks::dispatch_on_residual(self, condition_id, pokemon_pos, source_pos, effect_id_owned.as_deref())
            }
            "SideResidual" => {
                // Some side conditions use onResidual callback for SideResidual events
                // Example: gmaxvolcalith has condition.onResidual
                // This matches JavaScript behavior where the callback signature is compatible
                // Extract source and effect from event
                let source_pos = self.event.as_ref().and_then(|e| e.source);
                let effect_id_owned = self.event.as_ref()
                    .and_then(|e| e.effect.as_ref())
                    .map(|eff| eff.id.to_string());
                condition_callbacks::dispatch_on_residual(self, condition_id, pokemon_pos, source_pos, effect_id_owned.as_deref())
            }
            "Restart" => {
                // Extract source and effect from event
                let source_pos = self.event.as_ref().and_then(|e| e.source);
                let effect_id_owned = self.event.as_ref()
                    .and_then(|e| e.effect.as_ref())
                    .map(|eff| eff.id.to_string());
                condition_callbacks::dispatch_on_restart(self, condition_id, pokemon_pos, source_pos, effect_id_owned.as_deref())
            }
            "SourceModifyDamage" => {
                // SourceModifyDamage needs damage, source, target, and move
                // pokemon_pos is the source, extract target and damage from event
                let damage = self.event.as_ref().and_then(|e| match &e.relay_var {
                    Some(EventResult::Number(n)) => Some(*n),
                    _ => None
                }).unwrap_or(0);
                let target_pos = self.event.as_ref().and_then(|e| e.target).unwrap_or((0, 0));
                condition_callbacks::dispatch_on_source_modify_damage(
                    self,
                    condition_id,
                    damage,
                    pokemon_pos,  // source_pos
                    target_pos,
                    active_move_clone.as_ref(),
                )
            }
            "StallMove" => {
                condition_callbacks::dispatch_on_stall_move(self, condition_id, pokemon_pos)
            }
            "Start" => {
                // Extract source and effect from event
                let source_pos = self.event.as_ref().and_then(|e| e.source);
                let source_effect = self.event.as_ref()
                    .and_then(|e| e.effect.clone());
                let effect_id_owned = source_effect.as_ref()
                    .map(|eff| eff.id.to_string());
                // Try ability-embedded condition callbacks first (for abilities like Zen Mode)
                crate::data::ability_callbacks::dispatch_condition_on_start(self, condition_id, pokemon_pos, source_pos, effect_id_owned.as_deref());
                // Then try standalone/move-embedded condition callbacks
                condition_callbacks::dispatch_on_start(self, condition_id, pokemon_pos, source_pos, source_effect.as_ref())
            }
            "SetStatus" => {
                // SetStatus is called when a status is being set on the target
                // Terrain conditions like Misty Terrain and Electric Terrain prevent status
                // Extract status, target, source, and effect from event
                // status is what's being set (par, brn, slp, etc.) - passed via relay_var
                // target_pos is from the event (the Pokemon receiving status)
                let target_pos = self.event.as_ref().and_then(|e| e.target);
                let source_pos = self.event.as_ref().and_then(|e| e.source);
                let effect_id_owned = self.event.as_ref()
                    .and_then(|e| e.effect.as_ref())
                    .map(|eff| eff.id.to_string());
                // Extract status from relay_var - passed as EventResult::String from set_status.rs
                let status_owned = self.event.as_ref()
                    .and_then(|e| e.relay_var.as_ref())
                    .and_then(|rv| match rv {
                        EventResult::String(s) => Some(s.clone()),
                        _ => None,
                    });
                condition_callbacks::dispatch_on_set_status(
                    self,
                    condition_id,
                    status_owned.as_deref(),
                    target_pos,
                    source_pos,
                    effect_id_owned.as_deref(),
                )
            }
            "SwitchIn" => {
                condition_callbacks::dispatch_on_switch_in(self, condition_id, pokemon_pos)
            }
            "Swap" => {
                // Swap is called by slot conditions like lunardance/healingwish
                // to trigger healing on the incoming Pokemon
                use crate::data::move_callbacks;
                move_callbacks::dispatch_condition_on_swap_by_id(self, condition_id, pokemon_pos)
            }
            "TrapPokemon" => {
                condition_callbacks::dispatch_on_trap_pokemon(self, condition_id, pokemon_pos)
            }
            "TryHeal" => {
                // TryHeal is called when healing is attempted on the target
                // Heal Block's onTryHeal returns false to prevent healing
                // Extract damage, target, source, and effect from event
                let damage = self.event.as_ref().and_then(|e| match &e.relay_var {
                    Some(EventResult::Number(n)) => Some(*n),
                    _ => None
                }).unwrap_or(0);
                let source_pos = self.event.as_ref().and_then(|e| e.source);
                let effect_id_owned = self.event.as_ref()
                    .and_then(|e| e.effect.as_ref())
                    .map(|eff| eff.id.to_string());
                condition_callbacks::dispatch_on_try_heal(
                    self,
                    condition_id,
                    damage,
                    Some(pokemon_pos),
                    source_pos,
                    effect_id_owned.as_deref()
                )
            }
            "TryAddVolatile" => {
                // For TryAddVolatile, we need to pass the status (volatile) being added,
                // the target, source, and effect. The status is in relay_var as EventResult::String.
                // Extract owned strings to avoid borrow checker issues
                let status_owned = self.event.as_ref()
                    .and_then(|e| e.relay_var.as_ref())
                    .and_then(|rv| match rv {
                        EventResult::String(s) => Some(s.clone()),
                        _ => None,
                    });
                // For TryAddVolatile, pokemon_pos is the Pokemon that has the handler (e.g., a Pokemon on the side with Safeguard),
                // NOT the Pokemon having the volatile added. We need to get the actual target from event.
                let target_pos = self.event.as_ref().and_then(|e| e.target);
                let source_pos = self.event.as_ref().and_then(|e| e.source);
                let effect_id_owned = self.event.as_ref()
                    .and_then(|e| e.effect.as_ref())
                    .map(|eff| eff.id.to_string());

                condition_callbacks::dispatch_on_try_add_volatile(
                    self,
                    condition_id,
                    status_owned.as_deref(),
                    target_pos,
                    source_pos,
                    effect_id_owned.as_deref()
                )
            }
            "TryPrimaryHit" => {
                // Extract source from event
                let source_pos = self.event.as_ref().and_then(|e| e.source);
                condition_callbacks::dispatch_on_try_primary_hit(
                    self,
                    condition_id,
                    pokemon_pos,
                    source_pos,
                    active_move_clone.as_ref(),
                )
            }
            "TryHit" => {
                // TryHit needs both source and target positions
                // Get source from event
                let source_pos = self.event.as_ref().and_then(|e| e.source).unwrap_or((0, 0));
                condition_callbacks::dispatch_on_try_hit(
                    self,
                    condition_id,
                    source_pos,
                    pokemon_pos,
                    active_move_clone.as_ref(),
                )
            }
            "TryMove" => {
                // Extract target from event
                let target_pos = self.event.as_ref().and_then(|e| e.target);
                condition_callbacks::dispatch_on_try_move(
                    self,
                    condition_id,
                    pokemon_pos,
                    target_pos,
                    active_move_clone.as_ref(),
                )
            }
            "Type" => {
                // Extract types from pokemon
                let types_vec = self.pokemon_at(pokemon_pos.0, pokemon_pos.1)
                    .map(|p| p.types.clone());
                let types_slice = types_vec.as_ref().map(|v| &v[..]);
                condition_callbacks::dispatch_on_type(self, condition_id, pokemon_pos, types_slice)
            }
            "Weather" => {
                // Extract source and effect from event
                let source_pos = self.event.as_ref().and_then(|e| e.source);
                let effect_id_owned = self.event.as_ref()
                    .and_then(|e| e.effect.as_ref())
                    .map(|eff| eff.id.to_string());
                condition_callbacks::dispatch_on_weather(
                    self,
                    condition_id,
                    pokemon_pos,
                    source_pos,
                    effect_id_owned.as_deref(),
                )
            }
            "WeatherModifyDamage" => {
                // Extract damage from relay_var, attacker/defender from event, and move_id from active_move
                let damage = self.event.as_ref().and_then(|e| match &e.relay_var {
                    Some(EventResult::Number(n)) => Some(*n as i32),
                    _ => None,
                }).unwrap_or(0);
                let attacker_pos = self.event.as_ref().and_then(|e| e.source);
                let defender_pos = self.event.as_ref().and_then(|e| e.target);
                condition_callbacks::dispatch_on_weather_modify_damage(
                    self,
                    condition_id,
                    damage,
                    attacker_pos,
                    defender_pos,
                    active_move_clone.as_ref(),
                )
            }
            "Hit" => {
                // onHit callback for conditions (volatiles like rage)
                // pokemon_pos is the Pokemon with the volatile (target being hit)
                // source_pos is the Pokemon doing the hitting (attacker)
                // condition_id is the volatile that triggered this (e.g., "rage")
                let source_pos = self.event.as_ref().and_then(|e| e.source).unwrap_or((0, 0));
                crate::data::move_callbacks::dispatch_condition_on_hit(
                    self,
                    condition_id,
                    source_pos,
                    pokemon_pos,
                )
            }
            "AnyInvulnerability" => {
                // For AnyInvulnerability, we need the ORIGINAL target/source from the run_event call,
                // NOT the Pokemon that has the volatile. The volatile may be on a different Pokemon
                // than the target of the attack (e.g., Sky Drop volatile is on the user, not the target).
                let target_pos = self.event.as_ref().and_then(|e| e.target).unwrap_or((0, 0));
                let source_pos = self.event.as_ref().and_then(|e| e.source).unwrap_or((0, 0));
                condition_callbacks::dispatch_on_any_invulnerability(
                    self,
                    condition_id,
                    target_pos,  // Use original target from event, not pokemon_pos
                    source_pos,
                    active_move_clone.as_ref()
                )
            }
            "AnyPrepareHit" => {
                // For AnyPrepareHit, called when any Pokemon is about to use a snatchable move
                // Snatch intercepts the move and uses it for the snatch user instead
                // source_pos is the Pokemon about to use the move
                // target_pos is their target
                let source_pos = self.event.as_ref().and_then(|e| e.target);
                let target_pos = self.event.as_ref().and_then(|e| e.source);
                condition_callbacks::dispatch_on_any_prepare_hit(
                    self,
                    condition_id,
                    source_pos,
                    target_pos,
                    active_move_clone.as_ref()
                )
            }
            "Invulnerability" => {
                // For onInvulnerability, call the regular dispatch for condition callbacks
                // This handles two-turn moves like Dive, Fly, Dig, etc.
                let source_pos = self.event.as_ref().and_then(|e| e.source).unwrap_or((0, 0));
                crate::data::move_callbacks::dispatch_condition_on_invulnerability(
                    self,
                    condition_id,
                    source_pos,
                    active_move_clone.as_ref()
                )
            }
            "AnyModifyDamage" | "ModifyDamage" => {
                // For side conditions like Aurora Veil, Light Screen, and Reflect
                // Extract damage from relay_var, source and target from event
                // Side condition callbacks are in move_callbacks module (move-embedded conditions)
                eprintln!("[HANDLE_CONDITION_EVENT] AnyModifyDamage case: condition_id={}, event.modifier BEFORE callback={}",
                    condition_id, self.event.as_ref().map(|e| e.modifier).unwrap_or(0));

                // Debug: print event details
                if let Some(ref curr_ev) = self.event {
                    eprintln!("[HANDLE_CONDITION_EVENT] event: source={:?}, target={:?}, effect={:?}",
                        curr_ev.source, curr_ev.target, curr_ev.effect);
                }

                // In runEvent("ModifyDamage", pokemon, target, move, baseDamage):
                // - pokemon (attacker) becomes event.target
                // - target (defender) becomes event.source
                //
                // But JS callback onAnyModifyDamage(damage, source, target, move) expects:
                // - source = attacker
                // - target = defender
                //
                // So we need to swap: event.target -> callback source, event.source -> callback target
                let event_target = self.event.as_ref().and_then(|e| e.target).unwrap_or((0, 0)); // attacker
                let event_source = self.event.as_ref().and_then(|e| e.source).unwrap_or((0, 0)); // defender

                // For the callback: source_pos = attacker = event.target, target_pos = defender = event.source
                let callback_source = event_target; // attacker
                let callback_target = event_source; // defender

                // Extract the active move ID for logging
                let move_id_for_log = self.active_move.as_ref()
                    .map(|m| m.id.to_string())
                    .unwrap_or_default();

                eprintln!("[HANDLE_CONDITION_EVENT] Calling dispatch_condition_on_any_modify_damage for condition={}, move={}, source(attacker)={:?}, target(defender)={:?}",
                    condition_id, move_id_for_log, callback_source, callback_target);

                // Call dispatcher in move_callbacks (for move-embedded conditions like auroraveil)
                // Pass the active move, not just the ID
                let result = crate::data::move_callbacks::dispatch_condition_on_any_modify_damage(
                    self,
                    condition_id,
                    callback_source, // attacker
                    callback_target, // defender
                    active_move_clone.as_ref(),
                );
                eprintln!("[HANDLE_CONDITION_EVENT] Result: {:?}, event.modifier AFTER callback={}",
                    result, self.event.as_ref().map(|e| e.modifier).unwrap_or(0));
                result
            }
            "NegateImmunity" => {
                // onNegateImmunity callback for conditions (foresight, miracleeye)
                // Returns false to negate type immunity (e.g., Normal/Fighting vs Ghost)
                // pokemon_pos is the Pokemon with the volatile
                crate::data::move_callbacks::dispatch_condition_on_negate_immunity(
                    self,
                    condition_id,
                    pokemon_pos,
                )
            }
            "Update" => {
                // onUpdate callback for conditions (volatiles like fling)
                // pokemon_pos is the Pokemon with the volatile
                condition_callbacks::dispatch_on_update(
                    self,
                    condition_id,
                    pokemon_pos,
                    active_move_clone.as_ref(),
                )
            }
            _ => EventResult::Continue,
        };

        result
    }
}
