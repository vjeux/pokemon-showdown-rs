// NOTE: This method is NOT in JavaScript - Rust-specific implementation

use crate::*;

impl Battle {

    /// Handle ability events
    /// Rust helper method - JavaScript's singleEvent() directly invokes ability[`on${eventId}`] callbacks
    /// This method dispatches to ability_callbacks module based on event name
    /// Routes to ability-specific handlers for all event types (AfterBoost, ModifyDamage, etc.)
    pub fn handle_ability_event(
        &mut self,
        event_id: &str,
        ability_id: &ID,
        target: Option<&crate::event::EventTarget>,
    ) -> crate::event::EventResult {
        use crate::data::ability_callbacks;
        use crate::event::EventResult;

        // Extract pokemon position or side index from EventTarget
        let pokemon_pos = target.and_then(|t| t.as_pokemon()).unwrap_or((0, 0));
        let side_idx = target.and_then(|t| t.as_side());

        // Get Pokemon name for logging
        let pokemon_name = if let Some(pokemon) = self.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            pokemon.name.clone()
        } else {
            "Unknown".to_string()
        };

        crate::trace_ability!("turn={}, ability='{}' on {}, event='{}'",
            self.turn, ability_id.as_str(), pokemon_name, event_id);

        // Extract context from current_event for parameter wiring
        let (event_source_pos, event_target_pos, event_effect_id, event_status_id) = if let Some(ref event) = self.current_event {
            let effect_str = event.effect.as_ref().map(|id| id.to_string()).unwrap_or_else(|| String::new());
            (event.source, event.target, effect_str.clone(), effect_str)
        } else {
            (None, None, String::new(), String::new())
        };

        // Extract move_id from active_move
        let move_id_owned = if let Some(ref active_move) = self.active_move {
            active_move.id.to_string()
        } else {
            String::new()
        };
        let move_id = move_id_owned.as_str();

        // Extract relay variables from current_event
        let (relay_var_int, _relay_var_float, relay_var_boost, relay_var_string) = if let Some(ref event) = self.current_event {
            let relay_int = match &event.relay_var {
                Some(EventResult::Number(n)) => *n,
                _ => 0,
            };
            let relay_float = match &event.relay_var {
                Some(EventResult::Float(f)) => *f,
                _ => 0.0,
            };
            let relay_boost = match &event.relay_var {
                Some(EventResult::Boost(b)) => Some(b.clone()),
                _ => None,
            };
            let relay_string = match &event.relay_var {
                Some(EventResult::String(s)) => s.clone(),
                _ => String::new(),
            };
            (relay_int, relay_float, relay_boost, relay_string)
        } else {
            (0, 0.0, None, String::new())
        };

        let result = match event_id {
            "AfterBoost" => {
                let default_boost = crate::dex_data::BoostsTable::new();
                let boost = relay_var_boost.as_ref().unwrap_or(&default_boost);
                ability_callbacks::dispatch_on_after_boost(self, ability_id.as_str(), boost, Some(pokemon_pos), None, None)
            }
            "AfterEachBoost" => {
                let default_boost = crate::dex_data::BoostsTable::new();
                let boost = relay_var_boost.as_ref().unwrap_or(&default_boost);
                ability_callbacks::dispatch_on_after_each_boost(
                    self,
                    ability_id.as_str(),
                    boost,
                    Some(pokemon_pos),
                    event_source_pos,
                    if event_effect_id.is_empty() { None } else { Some(event_effect_id.as_str()) },
                )
            }
            "AfterMoveSecondary" => ability_callbacks::dispatch_on_after_move_secondary(
                self,
                ability_id.as_str(),
                pokemon_pos,
                event_source_pos.unwrap_or((0, 0)),
                move_id,
            ),
            "AfterMoveSecondarySelf" => ability_callbacks::dispatch_on_after_move_secondary_self(
                self,
                ability_id.as_str(),
                pokemon_pos,
                event_target_pos.unwrap_or((0, 0)),
                move_id,
            ),
            "AfterSetStatus" => ability_callbacks::dispatch_on_after_set_status(
                self,
                ability_id.as_str(),
                Some(event_status_id.as_str()),
                Some(pokemon_pos),
                event_source_pos,
                if event_effect_id.is_empty() { None } else { Some(event_effect_id.as_str()) },
            ),
            "AfterTerastallization" => ability_callbacks::dispatch_on_after_terastallization(
                self,
                ability_id.as_str(),
                pokemon_pos,
            ),
            "AfterUseItem" => ability_callbacks::dispatch_on_after_use_item(
                self,
                ability_id.as_str(),
                if event_effect_id.is_empty() { None } else { Some(event_effect_id.as_str()) },
                pokemon_pos,
            ),
            "AllyAfterUseItem" => ability_callbacks::dispatch_on_ally_after_use_item(
                self,
                ability_id.as_str(),
                if event_effect_id.is_empty() { None } else { Some(event_effect_id.as_str()) },
                pokemon_pos,
            ),
            "AllyBasePower" => ability_callbacks::dispatch_on_ally_base_power(
                self,
                ability_id.as_str(),
                relay_var_int,
                event_source_pos,
                event_target_pos,
                move_id,
            ),
            "AllyBasePowerPriority" => ability_callbacks::dispatch_on_ally_base_power_priority(
                self,
                ability_id.as_str(),
                relay_var_int,
                event_source_pos,
                event_target_pos,
                move_id,
            ),
            "AllyFaint" => {
                ability_callbacks::dispatch_on_ally_faint(self, ability_id.as_str(), Some(pokemon_pos))
            }
            "AllyModifyAtk" => ability_callbacks::dispatch_on_ally_modify_atk(
                self,
                ability_id.as_str(),
                relay_var_int,
                pokemon_pos,
            ),
            "AllyModifyAtkPriority" => ability_callbacks::dispatch_on_ally_modify_atk_priority(
                self,
                ability_id.as_str(),
                relay_var_int,
                pokemon_pos,
            ),
            "AllyModifySpD" => ability_callbacks::dispatch_on_ally_modify_sp_d(
                self,
                ability_id.as_str(),
                relay_var_int,
                pokemon_pos,
            ),
            "AllyModifySpDPriority" => ability_callbacks::dispatch_on_ally_modify_sp_d_priority(
                self,
                ability_id.as_str(),
                relay_var_int,
                pokemon_pos,
            ),
            "AllySetStatus" => ability_callbacks::dispatch_on_ally_set_status(
                self,
                ability_id.as_str(),
                event_status_id.as_str(),
                pokemon_pos,
                event_source_pos,
                if event_effect_id.is_empty() { None } else { Some(event_effect_id.as_str()) },
            ),
            "AllyTryAddVolatile" => ability_callbacks::dispatch_on_ally_try_add_volatile(
                self,
                ability_id.as_str(),
                Some(event_status_id.as_str()),
                Some(pokemon_pos),
                event_source_pos,
                if event_effect_id.is_empty() { None } else { Some(event_effect_id.as_str()) },
            ),
            "AllyTryBoost" => ability_callbacks::dispatch_on_ally_try_boost(
                self,
                ability_id.as_str(),
                Some(pokemon_pos),
                event_source_pos,
                if event_effect_id.is_empty() { None } else { Some(event_effect_id.as_str()) },
            ),
            "AllyTryHitSide" => ability_callbacks::dispatch_on_ally_try_hit_side(
                self,
                ability_id.as_str(),
                Some(pokemon_pos),
                event_source_pos,
                move_id,
            ),
            "AnyAccuracy" => ability_callbacks::dispatch_on_any_accuracy(
                self,
                ability_id.as_str(),
                relay_var_int,
                Some(pokemon_pos),
                event_source_pos,
                move_id,
            ),
            "AnyAfterMega" => ability_callbacks::dispatch_on_any_after_mega(
                self,
                ability_id.as_str(),
            ),
            "AnyAfterMove" => ability_callbacks::dispatch_on_any_after_move(
                self,
                ability_id.as_str(),
            ),
            "AnyAfterSetStatus" => ability_callbacks::dispatch_on_any_after_set_status(
                self,
                ability_id.as_str(),
                Some(event_status_id.as_str()),
                Some(pokemon_pos),
                event_source_pos,
                if event_effect_id.is_empty() { None } else { Some(event_effect_id.as_str()) },
            ),
            "AnyAfterTerastallization" => {
                ability_callbacks::dispatch_on_any_after_terastallization(
                    self,
                    ability_id.as_str(),
                )
            }
            "AnyBasePower" => ability_callbacks::dispatch_on_any_base_power(
                self,
                ability_id.as_str(),
                relay_var_int,
                Some(pokemon_pos),
                event_target_pos,
                move_id,
            ),
            "AnyBasePowerPriority" => ability_callbacks::dispatch_on_any_base_power_priority(
                self,
                ability_id.as_str(),
                relay_var_int,
                Some(pokemon_pos),
                event_target_pos,
                move_id,
            ),
            "AnyBeforeMove" => ability_callbacks::dispatch_on_any_before_move(
                self,
                ability_id.as_str(),
            ),
            "AnyDamage" => {
                ability_callbacks::dispatch_on_any_damage(
                    self,
                    ability_id.as_str(),
                    relay_var_int,
                    Some(pokemon_pos),
                    event_source_pos,
                    if event_effect_id.is_empty() { None } else { Some(event_effect_id.as_str()) },
                )
            }
            "AnyFaint" => {
                ability_callbacks::dispatch_on_any_faint(self, ability_id.as_str())
            }
            "AnyFaintPriority" => ability_callbacks::dispatch_on_any_faint_priority(
                self,
                ability_id.as_str(),
            ),
            "AnyInvulnerability" => ability_callbacks::dispatch_on_any_invulnerability(
                self,
                ability_id.as_str(),
                Some(pokemon_pos),
                event_source_pos,
                move_id,
            ),
            "AnyInvulnerabilityPriority" => {
                ability_callbacks::dispatch_on_any_invulnerability_priority(
                    self,
                    ability_id.as_str(),
                    Some(pokemon_pos),
                    None,
                    move_id,
                )
            }
            "AnyModifyAccuracy" => ability_callbacks::dispatch_on_any_modify_accuracy(
                self,
                ability_id.as_str(),
                relay_var_int,
                Some(pokemon_pos),
                event_source_pos,
            ),
            "AnyModifyAccuracyPriority" => {
                ability_callbacks::dispatch_on_any_modify_accuracy_priority(self, ability_id.as_str(), 0, Some(pokemon_pos), None)
            }
            "AnyModifyAtk" => ability_callbacks::dispatch_on_any_modify_atk(
                self,
                ability_id.as_str(),
                relay_var_int,
                event_source_pos,
                Some(pokemon_pos),
                move_id,
            ),
            "AnyModifyBoost" => ability_callbacks::dispatch_on_any_modify_boost(
                self,
                ability_id.as_str(),
                "",  // Note: boosts handled via relay_var_boost in current_event
                pokemon_pos,
            ),
            "AnyModifyDamage" => ability_callbacks::dispatch_on_any_modify_damage(
                self,
                ability_id.as_str(),
                relay_var_int,
                event_source_pos,
                Some(pokemon_pos),
                move_id,
            ),
            "AnyModifyDef" => ability_callbacks::dispatch_on_any_modify_def(
                self,
                ability_id.as_str(),
                relay_var_int,
                Some(pokemon_pos),
                event_source_pos,
                move_id,
            ),
            "AnyModifySpA" => ability_callbacks::dispatch_on_any_modify_sp_a(
                self,
                ability_id.as_str(),
                relay_var_int,
                event_source_pos,
                Some(pokemon_pos),
                move_id,
            ),
            "AnyModifySpD" => ability_callbacks::dispatch_on_any_modify_sp_d(
                self,
                ability_id.as_str(),
                relay_var_int,
                Some(pokemon_pos),
                event_source_pos,
                move_id,
            ),
            "AnyRedirectTarget" => ability_callbacks::dispatch_on_any_redirect_target(
                self,
                ability_id.as_str(),
                Some(pokemon_pos),
                event_source_pos,
                event_target_pos,
                move_id,
            ),
            "AnySetWeather" => ability_callbacks::dispatch_on_any_set_weather(
                self,
                ability_id.as_str(),
                Some(pokemon_pos),
                event_source_pos,
                relay_var_string.as_str(),
            ),
            "AnySwitchIn" => {
                ability_callbacks::dispatch_on_any_switch_in(self, ability_id.as_str())
            }
            "AnySwitchInPriority" => ability_callbacks::dispatch_on_any_switch_in_priority(
                self,
                ability_id.as_str(),
            ),
            "AnyTryMove" => {
                ability_callbacks::dispatch_on_any_try_move(
                    self,
                    ability_id.as_str(),
                    Some(pokemon_pos),
                    event_source_pos,
                    if event_effect_id.is_empty() { None } else { Some(event_effect_id.as_str()) },
                )
            }
            "AnyTryPrimaryHit" => ability_callbacks::dispatch_on_any_try_primary_hit(
                self,
                ability_id.as_str(),
                Some(pokemon_pos),
                event_source_pos,
                move_id,
            ),
            "BasePower" => {
                // BasePower is called via run_event, so attacker is in current_event.target, defender is in current_event.source
                let attacker_pos = self.current_event.as_ref().and_then(|e| e.target).unwrap_or((0, 0));
                let defender_pos = self.current_event.as_ref().and_then(|e| e.source).unwrap_or((0, 0));

                // Get base_power from relay_var
                let base_power = self.current_event.as_ref().and_then(|e| match &e.relay_var {
                    Some(EventResult::Number(n)) => Some(*n),
                    _ => None
                }).unwrap_or(0);

                // Get move_id from active_move (extract to owned String to avoid borrow issues)
                let move_id_owned = self.active_move.as_ref().map(|m| m.id.to_string()).unwrap_or_default();
                let move_id = move_id_owned.as_str();

                ability_callbacks::dispatch_on_base_power(self, ability_id.as_str(), base_power, attacker_pos, defender_pos, move_id)
            }
            "BasePowerPriority" => ability_callbacks::dispatch_on_base_power_priority(
                self,
                ability_id.as_str(),
                0, pokemon_pos, pokemon_pos,
            move_id,
            ),
            "BeforeMove" => {
                ability_callbacks::dispatch_on_before_move(self, ability_id.as_str(), pokemon_pos, event_target_pos, move_id)
            }
            "BeforeMovePriority" => ability_callbacks::dispatch_on_before_move_priority(
                self,
                ability_id.as_str(),
                pokemon_pos,
                event_target_pos,
                move_id,
            ),
            "BeforeSwitchIn" => ability_callbacks::dispatch_on_before_switch_in(
                self,
                ability_id.as_str(),
                pokemon_pos,
            ),
            "ChangeBoost" => {
                let (target_pos, source_pos, effect_id_string) = if let Some(ref event) = self.current_event {
                    (
                        event.target,
                        event.source,
                        event.effect.as_ref().map(|id| id.as_str().to_string())
                    )
                } else {
                    (Some(pokemon_pos), None, None)
                };
                let effect_id = effect_id_string.as_ref().map(|s| s.as_str());
                ability_callbacks::dispatch_on_change_boost(self, ability_id.as_str(), target_pos, source_pos, effect_id)
            }
            "CheckShow" => {
                ability_callbacks::dispatch_on_check_show(self, ability_id.as_str(), pokemon_pos)
            }
            "CriticalHit" => {
                ability_callbacks::dispatch_on_critical_hit(self, ability_id.as_str(), Some(pokemon_pos), event_source_pos, move_id)
            }
            "Damage" => {
                ability_callbacks::dispatch_on_damage(
                    self,
                    ability_id.as_str(),
                    relay_var_int,
                    pokemon_pos,
                    event_source_pos,
                    if event_effect_id.is_empty() { None } else { Some(event_effect_id.as_str()) },
                )
            }
            "DamagePriority" => ability_callbacks::dispatch_on_damage_priority(
                self,
                ability_id.as_str(),
                relay_var_int,
                pokemon_pos,
                event_source_pos,
                if event_effect_id.is_empty() { None } else { Some(event_effect_id.as_str()) },
            ),
            "DamagingHit" => {
                // Get move_id, source, and damage from current event context
                let (move_id, source_pos, damage) = self.current_event.as_ref()
                    .map(|e| (
                        e.effect.as_ref().map(|id| id.to_string()).unwrap_or_else(|| String::new()),
                        e.source,
                        match &e.relay_var { Some(EventResult::Number(n)) => *n, _ => 0 } // Extract damage from relay_var
                    ))
                    .unwrap_or_else(|| (String::new(), None, 0));
                ability_callbacks::dispatch_on_damaging_hit(self, ability_id.as_str(), damage, Some(pokemon_pos), source_pos, &move_id)
            }
            "DamagingHitOrder" => {
                // Get move_id and source from current event context
                let (move_id, source_pos) = self.current_event.as_ref()
                    .map(|e| (
                        e.effect.as_ref().map(|id| id.to_string()).unwrap_or_else(|| String::new()),
                        e.source
                    ))
                    .unwrap_or_else(|| (String::new(), None));
                ability_callbacks::dispatch_on_damaging_hit_order(
                    self,
                    ability_id.as_str(),
                    0,
                    Some(pokemon_pos),
                    source_pos,
                    &move_id,
                )
            }
            "DeductPP" => {
                ability_callbacks::dispatch_on_deduct_p_p(self, ability_id.as_str(), Some(pokemon_pos), event_source_pos)
            }
            "DisableMove" => {
                ability_callbacks::dispatch_on_disable_move(self, ability_id.as_str(), pokemon_pos)
            }
            "DragOut" => {
                ability_callbacks::dispatch_on_drag_out(self, ability_id.as_str(), pokemon_pos, event_source_pos, move_id)
            }
            "DragOutPriority" => ability_callbacks::dispatch_on_drag_out_priority(
                self,
                ability_id.as_str(),
                pokemon_pos,
                event_source_pos,
                move_id,
            ),
            "EatItem" => {
                ability_callbacks::dispatch_on_eat_item(
                    self,
                    ability_id.as_str(),
                    if event_effect_id.is_empty() { None } else { Some(event_effect_id.as_str()) },
                    pokemon_pos,
                    event_source_pos,
                    if event_effect_id.is_empty() { None } else { Some(event_effect_id.as_str()) },
                )
            }
            "Effectiveness" => {
                // Extract type_mod from relay_var and target_type from type_param
                let (type_mod, target_type) = {
                    let type_mod = self
                        .current_event
                        .as_ref()
                        .and_then(|e| match &e.relay_var { Some(EventResult::Number(n)) => Some(*n), _ => None })
                        .unwrap_or(0);

                    let target_type = self
                        .current_event
                        .as_ref()
                        .and_then(|e| e.type_param.clone())
                        .unwrap_or_default();

                    (type_mod, target_type)
                };

                ability_callbacks::dispatch_on_effectiveness(self, ability_id.as_str(), type_mod, pokemon_pos, &target_type, move_id)
            }
            "EmergencyExit" => ability_callbacks::dispatch_on_emergency_exit(
                self,
                ability_id.as_str(),
                Some(pokemon_pos)
            ),
            "End" => ability_callbacks::dispatch_on_end(self, ability_id.as_str(), pokemon_pos),
            "Faint" => ability_callbacks::dispatch_on_faint(self, ability_id.as_str(), pokemon_pos, event_source_pos, if event_effect_id.is_empty() { None } else { Some(event_effect_id.as_str()) }),
            "Flinch" => {
                ability_callbacks::dispatch_on_flinch(self, ability_id.as_str(), pokemon_pos)
            }
            "FoeAfterBoost" => ability_callbacks::dispatch_on_foe_after_boost(
                self,
                ability_id.as_str(),
                Some(pokemon_pos), event_source_pos, if event_effect_id.is_empty() { None } else { Some(event_effect_id.as_str()) }
            ),
            "FoeMaybeTrapPokemon" => ability_callbacks::dispatch_on_foe_maybe_trap_pokemon(
                self,
                ability_id.as_str(),
                pokemon_pos, event_source_pos
            ),
            "FoeTrapPokemon" => ability_callbacks::dispatch_on_foe_trap_pokemon(
                self,
                ability_id.as_str(),
                pokemon_pos,
            ),
            "FoeTryEatItem" => ability_callbacks::dispatch_on_foe_try_eat_item(
                self,
                ability_id.as_str()
            ),
            "FoeTryMove" => {
                ability_callbacks::dispatch_on_foe_try_move(self, ability_id.as_str(), Some(pokemon_pos), event_source_pos, move_id)
            }
            "FractionalPriority" => ability_callbacks::dispatch_on_fractional_priority(
                self,
                ability_id.as_str(),
                relay_var_int,
                pokemon_pos,
            None,
            move_id,
            ),
            "FractionalPriorityPriority" => {
                ability_callbacks::dispatch_on_fractional_priority_priority(
                    self,
                    ability_id.as_str(),
                    relay_var_int,
                    pokemon_pos,
                None,
                move_id,
            )
            }
            "Hit" => ability_callbacks::dispatch_on_hit(self, ability_id.as_str(), pokemon_pos, event_source_pos.unwrap_or((0, 0)), move_id),
            "Immunity" => {
                ability_callbacks::dispatch_on_immunity(self, ability_id.as_str(), "", pokemon_pos)
            }
            "ModifyAccuracy" => ability_callbacks::dispatch_on_modify_accuracy(
                self,
                ability_id.as_str(),
                relay_var_int, pokemon_pos, event_target_pos.unwrap_or((0, 0)),
            move_id,
            ),
            "ModifyAccuracyPriority" => ability_callbacks::dispatch_on_modify_accuracy_priority(
                self,
                ability_id.as_str(),
                0, pokemon_pos, pokemon_pos,
            move_id,
            ),
            "ModifyAtk" => {
                let (atk, defender_pos, move_id_str) = if let Some(ref event) = self.current_event {
                    (
                        match &event.relay_var { Some(EventResult::Number(n)) => *n, _ => 0 },
                        event.target.unwrap_or((0, 0)),
                        event.effect.as_ref().map(|id| id.as_str().to_string()).unwrap_or_default()
                    )
                } else {
                    (0, (0, 0), String::new())
                };
                ability_callbacks::dispatch_on_modify_atk(self, ability_id.as_str(), atk, pokemon_pos, defender_pos, &move_id_str)
            }
            "ModifyAtkPriority" => ability_callbacks::dispatch_on_modify_atk_priority(
                self,
                ability_id.as_str(),
                0, pokemon_pos, pokemon_pos,
            move_id,
            ),
            "ModifyCritRatio" => ability_callbacks::dispatch_on_modify_crit_ratio(self, ability_id.as_str(), relay_var_int, pokemon_pos, None, move_id),
            "ModifyDamage" => {
                ability_callbacks::dispatch_on_modify_damage(self, ability_id.as_str(), relay_var_int, pokemon_pos, event_source_pos.unwrap_or((0, 0)), move_id)
            }
            "ModifyDef" => {
                let (def, attacker_pos, move_id_str) = if let Some(ref event) = self.current_event {
                    (
                        match &event.relay_var { Some(EventResult::Number(n)) => *n, _ => 0 },
                        event.source.unwrap_or((0, 0)),
                        event.effect.as_ref().map(|id| id.as_str().to_string()).unwrap_or_default()
                    )
                } else {
                    (0, (0, 0), String::new())
                };
                ability_callbacks::dispatch_on_modify_def(self, ability_id.as_str(), def, pokemon_pos, attacker_pos, &move_id_str)
            }
            "ModifyDefPriority" => ability_callbacks::dispatch_on_modify_def_priority(self, ability_id.as_str(), 0, pokemon_pos, (0, 0), ""),
            "ModifyMove" => {
                ability_callbacks::dispatch_on_modify_move(self, ability_id.as_str(), move_id, pokemon_pos, event_target_pos)
            }
            "ModifyMovePriority" => ability_callbacks::dispatch_on_modify_move_priority(
                self,
                ability_id.as_str(),
            move_id,
            pokemon_pos,
            event_target_pos,
            ),
            "ModifyPriority" => ability_callbacks::dispatch_on_modify_priority(
                self,
                ability_id.as_str(),
                relay_var_int, pokemon_pos, event_target_pos,
            move_id,
            ),
            "ModifySTAB" => {
                let (stab, source_pos, target_pos, move_id_str) = if let Some(ref event) = self.current_event {
                    let stab_value = match &event.relay_var {
                        Some(EventResult::Float(f)) => *f,
                        _ => 1.0,
                    };
                    (
                        stab_value,
                        event.source,
                        event.target,
                        event.effect.as_ref().map(|id| id.as_str().to_string()).unwrap_or_default()
                    )
                } else {
                    (1.0, None, Some(pokemon_pos), String::new())
                };
                ability_callbacks::dispatch_on_modify_s_t_a_b(self, ability_id.as_str(), stab, source_pos, target_pos, &move_id_str)
            }
            "ModifySecondaries" => ability_callbacks::dispatch_on_modify_secondaries(self, ability_id.as_str()),
            "ModifySpA" => {
                let (spa, defender_pos, move_id_str) = if let Some(ref event) = self.current_event {
                    (
                        match &event.relay_var { Some(EventResult::Number(n)) => *n, _ => 0 },
                        event.target.unwrap_or((0, 0)),
                        event.effect.as_ref().map(|id| id.as_str().to_string()).unwrap_or_default()
                    )
                } else {
                    (0, (0, 0), String::new())
                };
                ability_callbacks::dispatch_on_modify_sp_a(self, ability_id.as_str(), spa, pokemon_pos, defender_pos, &move_id_str)
            }
            "ModifySpAPriority" => ability_callbacks::dispatch_on_modify_sp_a_priority(
                self,
                ability_id.as_str(),
                0, pokemon_pos, pokemon_pos,
            move_id,
            ),
            "ModifySpe" => {
                ability_callbacks::dispatch_on_modify_spe(self, ability_id.as_str(), relay_var_int, pokemon_pos)
            }
            "ModifyType" => {
                ability_callbacks::dispatch_on_modify_type(self, ability_id.as_str(), move_id, pokemon_pos, event_target_pos)
            }
            "ModifyTypePriority" => ability_callbacks::dispatch_on_modify_type_priority(
                self,
                ability_id.as_str(),
                move_id,
                pokemon_pos,
                event_target_pos,
            ),
            "ModifyWeight" => {
                ability_callbacks::dispatch_on_modify_weight(self, ability_id.as_str(), relay_var_int, pokemon_pos)
            }
            "ModifyWeightPriority" => ability_callbacks::dispatch_on_modify_weight_priority(self, ability_id.as_str(), relay_var_int, pokemon_pos),
            "PrepareHit" => {
                // Get move_id from active_move context (extract to avoid borrow issues)
                let move_id_string = if let Some(ref active_move) = self.active_move {
                    active_move.id.to_string()
                } else {
                    String::new()
                };
                // For PrepareHit, pokemon_pos is the source (Pokemon using the move)
                ability_callbacks::dispatch_on_prepare_hit(self, ability_id.as_str(), Some(pokemon_pos), event_target_pos, &move_id_string)
            }
            "Residual" => {
                ability_callbacks::dispatch_on_residual(self, ability_id.as_str(), pokemon_pos, event_source_pos, if event_effect_id.is_empty() { None } else { Some(event_effect_id.as_str()) })
            }
            "ResidualOrder" => ability_callbacks::dispatch_on_residual_order(
                self,
                ability_id.as_str(),
                pokemon_pos,
                event_source_pos,
                if event_effect_id.is_empty() { None } else { Some(event_effect_id.as_str()) },
            ),
            "ResidualSubOrder" => ability_callbacks::dispatch_on_residual_sub_order(
                self,
                ability_id.as_str(),
                pokemon_pos,
                event_source_pos,
                if event_effect_id.is_empty() { None } else { Some(event_effect_id.as_str()) },
            ),
            "SetStatus" => {
                ability_callbacks::dispatch_on_set_status(self, ability_id.as_str(), event_status_id.as_str(), pokemon_pos, event_source_pos, if event_effect_id.is_empty() { None } else { Some(event_effect_id.as_str()) })
            }
            "SideConditionStart" => {
                let side_condition_id_string = self.current_event.as_ref()
                    .and_then(|e| e.effect.as_ref())
                    .map(|id| id.as_str().to_string());
                let side_condition_id = side_condition_id_string.as_ref().map(|s| s.as_str()).unwrap_or("");

                // For side events, target in handle_ability_event is the Pokemon with the ability (effect_holder)
                // but we also need the side_idx from the original event target stored in current_event
                let side_index = side_idx.unwrap_or_else(|| {
                    // If target wasn't a Side, pokemon is on side 0 by default
                    pokemon_pos.0
                });

                ability_callbacks::dispatch_on_side_condition_start(
                    self,
                    ability_id.as_str(),
                    pokemon_pos,
                    side_index,
                    side_condition_id,
                    event_source_pos
                )
            }
            "SourceAfterFaint" => ability_callbacks::dispatch_on_source_after_faint(self, ability_id.as_str(), Some(pokemon_pos), None, None),
            "SourceBasePower" => ability_callbacks::dispatch_on_source_base_power(
                self,
                ability_id.as_str(),
                0,
            move_id,
            ),
            "SourceBasePowerPriority" => ability_callbacks::dispatch_on_source_base_power_priority(
                self,
                ability_id.as_str(),
                0,
            move_id,
            ),
            "SourceDamagingHit" => ability_callbacks::dispatch_on_source_damaging_hit(
                self,
                ability_id.as_str(),
                0, Some(pokemon_pos), None,
            move_id,
            ),
            "SourceModifyAccuracy" => ability_callbacks::dispatch_on_source_modify_accuracy(
                self,
                ability_id.as_str(),
                0, pokemon_pos, pokemon_pos,
            move_id,
            ),
            "SourceModifyAccuracyPriority" => {
                ability_callbacks::dispatch_on_source_modify_accuracy_priority(
                    self,
                    ability_id.as_str(),
                    0, pokemon_pos, pokemon_pos,
                move_id,
            )
            }
            "SourceModifyAtk" => ability_callbacks::dispatch_on_source_modify_atk(
                self,
                ability_id.as_str(),
            move_id,
            ),
            "SourceModifyAtkPriority" => ability_callbacks::dispatch_on_source_modify_atk_priority(
                self,
                ability_id.as_str(),
            move_id,
            ),
            "SourceModifyDamage" => {
                // Extract parameters from current_event
                let (damage, source_pos, move_id_str) = if let Some(ref event) = self.current_event {
                    (
                        match &event.relay_var { Some(EventResult::Number(n)) => *n, _ => 0 }, // damage value
                        event.target.unwrap_or((0, 0)), // attacker position
                        event.effect.as_ref().map(|id| id.as_str().to_string()).unwrap_or_default() // move id
                    )
                } else {
                    (0, (0, 0), String::new())
                };
                // pokemon_pos is the defender (pokemon with the ability, e.g., Houndstone with Fluffy)
                // source_pos is the attacker (e.g., Koraidon using Outrage)
                ability_callbacks::dispatch_on_source_modify_damage(
                    self,
                    ability_id.as_str(),
                    damage, source_pos, pokemon_pos,
                &move_id_str
                )
            }
            "SourceModifyDamagePriority" => {
                ability_callbacks::dispatch_on_source_modify_damage_priority(
                    self,
                    ability_id.as_str(),
                    0, pokemon_pos, pokemon_pos,
                move_id,
            )
            }
            "SourceModifySecondaries" => ability_callbacks::dispatch_on_source_modify_secondaries(
                self,
                ability_id.as_str(),
                Some(pokemon_pos), None,
            move_id,
            ),
            "SourceModifySpA" => ability_callbacks::dispatch_on_source_modify_sp_a(
                self,
                ability_id.as_str(),
            move_id,
            ),
            "SourceModifySpAPriority" => {
                ability_callbacks::dispatch_on_source_modify_sp_a_priority(
                    self,
                    ability_id.as_str(),
                move_id,
            )
            }
            "SourceTryHeal" => ability_callbacks::dispatch_on_source_try_heal(
                self,
                ability_id.as_str(),
                relay_var_int, Some(pokemon_pos), None, None
            ),
            "SourceTryPrimaryHit" => ability_callbacks::dispatch_on_source_try_primary_hit(
                self,
                ability_id.as_str(),
                Some(pokemon_pos), None, None
            ),
            "Start" => ability_callbacks::dispatch_on_start(self, ability_id.as_str(), pokemon_pos),
            "SwitchIn" => {
                // JavaScript getCallback() special logic:
                // In gen >= 5, abilities use onStart callback during SwitchIn event
                // instead of onSwitchIn callback (unless ability has onAnySwitchIn)
                if self.gen >= 5 {
                    let result = ability_callbacks::dispatch_on_start(self, ability_id.as_str(), pokemon_pos);
                    if !matches!(result, EventResult::Continue) {
                        return result;
                    }
                }
                ability_callbacks::dispatch_on_switch_in(self, ability_id.as_str(), pokemon_pos)
            },
            "SwitchInPriority" => ability_callbacks::dispatch_on_switch_in_priority(
                self,
                ability_id.as_str(),
                pokemon_pos,
            ),
            "SwitchOut" => {
                ability_callbacks::dispatch_on_switch_out(self, ability_id.as_str(), pokemon_pos)
            },
            "TakeItem" => {
                ability_callbacks::dispatch_on_take_item(self, ability_id.as_str(), pokemon_pos, None)
            },
            "TerrainChange" => ability_callbacks::dispatch_on_terrain_change(
                self,
                ability_id.as_str(),
                pokemon_pos
            ),
            "TryAddVolatile" => {
                // Extract status_id from relay_var (e.g., "confusion")
                let status_id = if let Some(ref event) = self.current_event {
                    match &event.relay_var {
                        Some(EventResult::String(s)) => s.clone(),
                        _ => String::new(),
                    }
                } else {
                    String::new()
                };
                ability_callbacks::dispatch_on_try_add_volatile(
                    self,
                    ability_id.as_str(),
                    &status_id, pokemon_pos, None, None
                )
            }
            "TryBoost" => {
                // Temporarily take boost out of current_event to get mutable access
                let mut boost = self.current_event.as_mut().and_then(|e| {
                    let relay_var = e.relay_var.take();
                    match relay_var {
                        Some(EventResult::Boost(b)) => Some(b),
                        _ => {
                            // Put it back if it wasn't a Boost
                            e.relay_var = relay_var;
                            None
                        }
                    }
                });
                let result = ability_callbacks::dispatch_on_try_boost(
                    self,
                    ability_id.as_str(),
                    pokemon_pos,
                    boost.as_mut(),
                );
                // Put it back
                if let Some(b) = boost {
                    if let Some(ref mut event) = self.current_event {
                        event.relay_var = Some(EventResult::Boost(b));
                    }
                }
                result
            }
            "TryBoostPriority" => {
                // Temporarily take boost out of current_event to get mutable access
                let mut boost = self.current_event.as_mut().and_then(|e| {
                    let relay_var = e.relay_var.take();
                    match relay_var {
                        Some(EventResult::Boost(b)) => Some(b),
                        _ => {
                            // Put it back if it wasn't a Boost
                            e.relay_var = relay_var;
                            None
                        }
                    }
                });
                let result = ability_callbacks::dispatch_on_try_boost_priority(
                    self,
                    ability_id.as_str(),
                    pokemon_pos,
                    boost.as_mut(),
                );
                // Put it back
                if let Some(b) = boost {
                    if let Some(ref mut event) = self.current_event {
                        event.relay_var = Some(EventResult::Boost(b));
                    }
                }
                result
            }
            "TryEatItem" => {
                ability_callbacks::dispatch_on_try_eat_item(self, ability_id.as_str())
            }
            "TryEatItemPriority" => ability_callbacks::dispatch_on_try_eat_item_priority(
                self,
                ability_id.as_str()
            ),
            "TryHeal" => {
                ability_callbacks::dispatch_on_try_heal(self, ability_id.as_str(), relay_var_int, Some(pokemon_pos), None, None)
            }
            "TryHit" => {
                ability_callbacks::dispatch_on_try_hit(self, ability_id.as_str(), pokemon_pos, event_source_pos.unwrap_or((0, 0)), move_id)
            }
            "TryHitPriority" => ability_callbacks::dispatch_on_try_hit_priority(
                self,
                ability_id.as_str(),
                pokemon_pos, pokemon_pos,
            move_id,
            ),
            "Update" => {
                ability_callbacks::dispatch_on_update(self, ability_id.as_str(), pokemon_pos)
            },
            "Weather" => {
                ability_callbacks::dispatch_on_weather(self, ability_id.as_str(), "", pokemon_pos, None)
            },
            "WeatherChange" => ability_callbacks::dispatch_on_weather_change(
                self,
                ability_id.as_str(),
                pokemon_pos,
            ),
            _ => EventResult::Continue,
        };

        crate::trace_ability!("  ← Ability '{}' on {} returned: {:?}", ability_id.as_str(), pokemon_name, result);

        result
    }
}
