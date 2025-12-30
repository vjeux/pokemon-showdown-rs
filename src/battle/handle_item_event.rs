use crate::*;

impl Battle {

    /// Handle item events
    /// Rust helper method - JavaScript's singleEvent() directly invokes item[`on${eventId}`] callbacks
    /// This method dispatches to item_callbacks module based on event name
    /// Routes to item-specific handlers for all event types (AfterBoost, ModifyDamage, Eat, etc.)
    pub fn handle_item_event(
        &mut self,
        event_id: &str,
        item_id: &ID,
        target: Option<(usize, usize)>,
    ) -> crate::event::EventResult {
        use crate::data::item_callbacks;
        use crate::event::EventResult;

        let source = self.current_event.as_ref().and_then(|e| e.source);
        let relay_var = self.current_event.as_ref().and_then(|e| e.relay_var);
        let relay_var_float = self.current_event.as_ref().and_then(|e| e.relay_var_float);
        let relay_var_boost = self.current_event.as_ref().and_then(|e| e.relay_var_boost.clone());
        let relay_var_type = self.current_event.as_ref().and_then(|e| e.relay_var_type.clone());
        let pokemon_pos = target.unwrap_or((0, 0));

        match event_id {
            "AfterBoost" => {
                let boost = relay_var_boost.as_ref();
                item_callbacks::dispatch_on_after_boost(
                    self,
                    item_id.as_str(),
                    pokemon_pos,
                    boost,
                )
            }
            "AfterMoveSecondary" => item_callbacks::dispatch_on_after_move_secondary(
                self,
                item_id.as_str(),
                pokemon_pos,
            ),
            "AfterMoveSecondaryPriority" => {
                item_callbacks::dispatch_on_after_move_secondary_priority(
                    self,
                    item_id.as_str(),
                    pokemon_pos,
                )
            }
            "AfterMoveSecondarySelf" => {
                if let Some(source_pos) = source {
                    let move_id_str = if let Some(ref active_move) = self.active_move {
                        active_move.id.to_string()
                    } else {
                        String::new()
                    };
                    item_callbacks::dispatch_on_after_move_secondary_self(
                        self,
                        item_id.as_str(),
                        source_pos,
                        target,
                        &move_id_str,
                    )
                } else {
                    EventResult::Continue
                }
            }
            "AfterMoveSecondarySelfPriority" => {
                item_callbacks::dispatch_on_after_move_secondary_self_priority(
                    self,
                    item_id.as_str(),
                    pokemon_pos,
                )
            }
            "AfterSetStatus" => {
                item_callbacks::dispatch_on_after_set_status(self, item_id.as_str(), pokemon_pos)
            }
            "AfterSetStatusPriority" => item_callbacks::dispatch_on_after_set_status_priority(
                self,
                item_id.as_str(),
                pokemon_pos,
            ),
            "AfterSubDamage" => {
                item_callbacks::dispatch_on_after_sub_damage(self, item_id.as_str(), pokemon_pos)
            }
            "AnyAfterMega" => {
                item_callbacks::dispatch_on_any_after_mega(self, item_id.as_str(), pokemon_pos)
            }
            "AnyAfterMove" => {
                item_callbacks::dispatch_on_any_after_move(self, item_id.as_str(), pokemon_pos)
            }
            "AnyAfterTerastallization" => item_callbacks::dispatch_on_any_after_terastallization(
                self,
                item_id.as_str(),
                pokemon_pos,
            ),
            "AnyPseudoWeatherChange" => item_callbacks::dispatch_on_any_pseudo_weather_change(
                self,
                item_id.as_str(),
                pokemon_pos,
            ),
            "AnySwitchIn" => {
                item_callbacks::dispatch_on_any_switch_in(self, item_id.as_str(), pokemon_pos)
            }
            "AnySwitchInPriority" => item_callbacks::dispatch_on_any_switch_in_priority(
                self,
                item_id.as_str(),
                pokemon_pos,
            ),
            "Attract" => item_callbacks::dispatch_on_attract(self, item_id.as_str(), pokemon_pos),
            "AttractPriority" => {
                item_callbacks::dispatch_on_attract_priority(self, item_id.as_str(), pokemon_pos)
            }
            "BasePower" => {
                // Extract move info from active_move (clone to avoid borrow issues)
                let (base_power, move_id_str) = if let Some(ref active_move) = self.active_move {
                    (active_move.base_power, active_move.id.to_string())
                } else {
                    (0, String::new())
                };
                let target_pos = self.current_event.as_ref().and_then(|e| e.target);
                item_callbacks::dispatch_on_base_power(self, item_id.as_str(), base_power, pokemon_pos, target_pos, &move_id_str)
            }
            "BasePowerPriority" => {
                item_callbacks::dispatch_on_base_power_priority(self, item_id.as_str(), pokemon_pos)
            }
            "ChargeMove" => {
                item_callbacks::dispatch_on_charge_move(self, item_id.as_str(), pokemon_pos)
            }
            "Damage" => item_callbacks::dispatch_on_damage(self, item_id.as_str(), pokemon_pos),
            "DamagePriority" => {
                item_callbacks::dispatch_on_damage_priority(self, item_id.as_str(), pokemon_pos)
            }
            "DamagingHit" => {
                let damage = relay_var.unwrap_or(0);
                let move_id_str = if let Some(ref active_move) = self.active_move {
                    active_move.id.to_string()
                } else {
                    String::new()
                };
                // target is the pokemon being hit, source is the attacker
                let target_pos = pokemon_pos;
                let source_pos = source.unwrap_or(pokemon_pos);
                item_callbacks::dispatch_on_damaging_hit(self, item_id.as_str(), damage, target_pos, source_pos, &move_id_str)
            }
            "DamagingHitOrder" => {
                item_callbacks::dispatch_on_damaging_hit_order(self, item_id.as_str(), pokemon_pos)
            }
            "DisableMove" => {
                item_callbacks::dispatch_on_disable_move(self, item_id.as_str(), pokemon_pos)
            }
            "Drive" => item_callbacks::dispatch_on_drive(self, item_id.as_str(), pokemon_pos),
            "Eat" => item_callbacks::dispatch_on_eat(self, item_id.as_str(), pokemon_pos),
            "Effectiveness" => {
                item_callbacks::dispatch_on_effectiveness(self, item_id.as_str(), pokemon_pos)
            }
            "End" => item_callbacks::dispatch_on_end(self, item_id.as_str(), pokemon_pos),
            "FoeAfterBoost" => {
                let boost = relay_var_boost.as_ref();
                let effect_id_clone = self.current_effect.clone();
                let effect_id_str = effect_id_clone.as_ref().map(|e| e.as_str());
                item_callbacks::dispatch_on_foe_after_boost(
                    self,
                    item_id.as_str(),
                    pokemon_pos,
                    target,
                    source,
                    effect_id_str,
                    boost,
                )
            }
            "FractionalPriority" => {
                item_callbacks::dispatch_on_fractional_priority(
                    self,
                    item_id.as_str(),
                    pokemon_pos,
                    relay_var_float.unwrap_or(0.0),
                )
            }
            "FractionalPriorityPriority" => {
                item_callbacks::dispatch_on_fractional_priority_priority(
                    self,
                    item_id.as_str(),
                    pokemon_pos,
                )
            }
            "Hit" => item_callbacks::dispatch_on_hit(self, item_id.as_str(), pokemon_pos),
            "Immunity" => item_callbacks::dispatch_on_immunity(
                self,
                item_id.as_str(),
                pokemon_pos,
                relay_var_type.as_deref(),
            ),
            "MaybeTrapPokemon" => {
                item_callbacks::dispatch_on_maybe_trap_pokemon(self, item_id.as_str(), pokemon_pos)
            }
            "MaybeTrapPokemonPriority" => item_callbacks::dispatch_on_maybe_trap_pokemon_priority(
                self,
                item_id.as_str(),
                pokemon_pos,
            ),
            "Memory" => item_callbacks::dispatch_on_memory(self, item_id.as_str(), pokemon_pos),
            "ModifyAccuracy" => {
                item_callbacks::dispatch_on_modify_accuracy(self, item_id.as_str(), pokemon_pos)
            }
            "ModifyAccuracyPriority" => item_callbacks::dispatch_on_modify_accuracy_priority(
                self,
                item_id.as_str(),
                pokemon_pos,
            ),
            "ModifyAtk" => {
                item_callbacks::dispatch_on_modify_atk(self, item_id.as_str(), pokemon_pos)
            }
            "ModifyAtkPriority" => {
                item_callbacks::dispatch_on_modify_atk_priority(self, item_id.as_str(), pokemon_pos)
            }
            "ModifyCritRatio" => {
                item_callbacks::dispatch_on_modify_crit_ratio(
                    self,
                    item_id.as_str(),
                    pokemon_pos,
                    relay_var.unwrap_or(0),
                )
            }
            "ModifyDamage" => {
                let damage = relay_var.unwrap_or(0);
                let move_id_str = if let Some(ref active_move) = self.active_move {
                    active_move.id.to_string()
                } else {
                    String::new()
                };
                let target_pos = self.current_event.as_ref().and_then(|e| e.target);
                item_callbacks::dispatch_on_modify_damage(self, item_id.as_str(), damage, pokemon_pos, target_pos, &move_id_str)
            }
            "ModifyDef" => {
                item_callbacks::dispatch_on_modify_def(self, item_id.as_str(), pokemon_pos)
            }
            "ModifyDefPriority" => {
                item_callbacks::dispatch_on_modify_def_priority(self, item_id.as_str(), pokemon_pos)
            }
            "ModifyMove" => {
                let move_id_str = if let Some(ref active_move) = self.active_move {
                    active_move.id.to_string()
                } else {
                    String::new()
                };
                item_callbacks::dispatch_on_modify_move(self, item_id.as_str(), &move_id_str, pokemon_pos)
            }
            "ModifyMovePriority" => item_callbacks::dispatch_on_modify_move_priority(
                self,
                item_id.as_str(),
                pokemon_pos,
            ),
            "ModifySecondaries" => {
                // Temporarily take secondaries out of current_event to get mutable access
                let mut secondaries = self.current_event.as_mut().and_then(|e| e.relay_var_secondaries.take());
                let result = item_callbacks::dispatch_on_modify_secondaries(
                    self,
                    item_id.as_str(),
                    pokemon_pos,
                    secondaries.as_mut(),
                );
                // Put secondaries back into current_event
                if let Some(ref mut event) = self.current_event {
                    event.relay_var_secondaries = secondaries;
                }
                result
            }
            "ModifySpA" => {
                item_callbacks::dispatch_on_modify_sp_a(self, item_id.as_str(), pokemon_pos)
            }
            "ModifySpAPriority" => item_callbacks::dispatch_on_modify_sp_a_priority(
                self,
                item_id.as_str(),
                pokemon_pos,
            ),
            "ModifySpD" => {
                item_callbacks::dispatch_on_modify_sp_d(self, item_id.as_str(), pokemon_pos)
            }
            "ModifySpDPriority" => item_callbacks::dispatch_on_modify_sp_d_priority(
                self,
                item_id.as_str(),
                pokemon_pos,
            ),
            "ModifySpe" => {
                item_callbacks::dispatch_on_modify_spe(self, item_id.as_str(), pokemon_pos)
            }
            "ModifyWeight" => {
                item_callbacks::dispatch_on_modify_weight(
                    self,
                    item_id.as_str(),
                    pokemon_pos,
                    relay_var.unwrap_or(0),
                )
            }
            "NegateImmunity" => {
                item_callbacks::dispatch_on_negate_immunity(self, item_id.as_str(), pokemon_pos)
            }
            "Plate" => item_callbacks::dispatch_on_plate(self, item_id.as_str(), pokemon_pos),
            "Residual" => item_callbacks::dispatch_on_residual(self, item_id.as_str(), pokemon_pos),
            "ResidualOrder" => {
                item_callbacks::dispatch_on_residual_order(self, item_id.as_str(), pokemon_pos)
            }
            "ResidualSubOrder" => {
                item_callbacks::dispatch_on_residual_sub_order(self, item_id.as_str(), pokemon_pos)
            }
            "SetAbility" => {
                item_callbacks::dispatch_on_set_ability(self, item_id.as_str(), pokemon_pos)
            }
            "SourceModifyAccuracy" => item_callbacks::dispatch_on_source_modify_accuracy(
                self,
                item_id.as_str(),
                pokemon_pos,
            ),
            "SourceModifyAccuracyPriority" => {
                item_callbacks::dispatch_on_source_modify_accuracy_priority(
                    self,
                    item_id.as_str(),
                    pokemon_pos,
                )
            }
            "SourceModifyDamage" => {
                let damage = relay_var.unwrap_or(0);
                let move_id_str = if let Some(ref active_move) = self.active_move {
                    active_move.id.to_string()
                } else {
                    String::new()
                };
                let source_pos = pokemon_pos;
                let target_pos = target.unwrap_or(pokemon_pos);
                item_callbacks::dispatch_on_source_modify_damage(self, item_id.as_str(), damage, source_pos, target_pos, &move_id_str)
            },
            "SourceTryPrimaryHit" => item_callbacks::dispatch_on_source_try_primary_hit(
                self,
                item_id.as_str(),
                pokemon_pos,
            ),
            "Start" => item_callbacks::dispatch_on_start(self, item_id.as_str(), pokemon_pos),
            "SwitchIn" => {
                item_callbacks::dispatch_on_switch_in(self, item_id.as_str(), pokemon_pos)
            }
            "SwitchInPriority" => {
                item_callbacks::dispatch_on_switch_in_priority(self, item_id.as_str(), pokemon_pos)
            }
            "TakeItem" => {
                let source = self.current_event.as_ref().and_then(|e| e.source);
                item_callbacks::dispatch_on_take_item(self, item_id.as_str(), pokemon_pos, source)
            }
            "TerrainChange" => {
                item_callbacks::dispatch_on_terrain_change(self, item_id.as_str(), pokemon_pos)
            }
            "TrapPokemon" => {
                item_callbacks::dispatch_on_trap_pokemon(self, item_id.as_str(), pokemon_pos)
            }
            "TrapPokemonPriority" => item_callbacks::dispatch_on_trap_pokemon_priority(
                self,
                item_id.as_str(),
                pokemon_pos,
            ),
            "TryBoost" => {
                // Temporarily take boost out of current_event to get mutable access
                let mut boost = self.current_event.as_mut().and_then(|e| e.relay_var_boost.take());
                let result = item_callbacks::dispatch_on_try_boost(
                    self,
                    item_id.as_str(),
                    pokemon_pos,
                    boost.as_mut(),
                );
                // Put boost back into current_event
                if let Some(ref mut event) = self.current_event {
                    event.relay_var_boost = boost;
                }
                result
            }
            "TryBoostPriority" => {
                item_callbacks::dispatch_on_try_boost_priority(self, item_id.as_str(), pokemon_pos)
            }
            "TryEatItem" => {
                item_callbacks::dispatch_on_try_eat_item(self, item_id.as_str(), pokemon_pos)
            }
            "TryHeal" => item_callbacks::dispatch_on_try_heal(self, item_id.as_str(), pokemon_pos),
            "TryHealPriority" => {
                item_callbacks::dispatch_on_try_heal_priority(self, item_id.as_str(), pokemon_pos)
            }
            "TryHit" => {
                if let Some(source_pos) = source {
                    let move_id_str = if let Some(ref active_move) = self.active_move {
                        active_move.id.to_string()
                    } else {
                        String::new()
                    };
                    item_callbacks::dispatch_on_try_hit(self, item_id.as_str(), pokemon_pos, source_pos, &move_id_str)
                } else {
                    EventResult::Continue
                }
            }
            "Update" => item_callbacks::dispatch_on_update(self, item_id.as_str(), pokemon_pos),
            "Use" => item_callbacks::dispatch_on_use(self, item_id.as_str(), pokemon_pos),
            "UseItem" => item_callbacks::dispatch_on_use_item(self, item_id.as_str(), pokemon_pos),
            _ => EventResult::Continue,
        }
    }
}
