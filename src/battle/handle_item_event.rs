// NOTE: This method is NOT in JavaScript - Rust-specific implementation

use crate::*;

impl Battle {

    /// Handle item events
    /// Rust helper method - JavaScript's singleEvent() directly invokes item[`on${eventId}`] callbacks
    /// This method dispatches to item_callbacks module based on event name
    /// Routes to item-specific handlers for all event types (AfterBoost, ModifyDamage, Eat, etc.)
    ///
    /// ALL SIGNATURES MATCH TYPESCRIPT DEFINITIONS - TypeScript is the source of truth
    pub fn handle_item_event(
        &mut self,
        event_id: &str,
        item_id: &ID,
        target: Option<(usize, usize)>,
    ) -> crate::event::EventResult {
        use crate::data::item_callbacks;
        use crate::event::EventResult;

        eprintln!("[HANDLE_ITEM_EVENT] event_id={}, item_id={}, target={:?}",
            event_id, item_id.as_str(), target);

        let source = self.current_event.as_ref().and_then(|e| e.source);
        let relay_var = self.current_event.as_ref().and_then(|e| e.relay_var);
        let relay_var_float = self.current_event.as_ref().and_then(|e| e.relay_var_float);
        let relay_var_boost = self.current_event.as_ref().and_then(|e| e.relay_var_boost.clone());
        let relay_var_type = self.current_event.as_ref().and_then(|e| e.relay_var_type.clone());
        let pokemon_pos = target.unwrap_or((0, 0));

        match event_id {
            // TypeScript: onAfterBoost(target:Pokemon, boost:BoostsTable)
            "AfterBoost" => {
                let boost = match relay_var_boost.as_ref() {
                    Some(b) => b,
                    None => return EventResult::Continue,
                };
                item_callbacks::dispatch_on_after_boost(
                    self,
                    item_id.as_str(),
                    pokemon_pos,
                    boost,
                )
            }

            // TypeScript: onAfterMoveSecondary(target:Pokemon?, source:Pokemon?, move:Move)
            "AfterMoveSecondary" => {
                let move_id_str = if let Some(ref active_move) = self.active_move {
                    active_move.id.to_string()
                } else {
                    String::new()
                };
                item_callbacks::dispatch_on_after_move_secondary(
                    self,
                    item_id.as_str(),
                    target,
                    source,
                    &move_id_str,
                )
            }

            "AfterMoveSecondaryPriority" => {
                item_callbacks::dispatch_on_after_move_secondary_priority(
                    self,
                    item_id.as_str(),
                    pokemon_pos,
                )
            }

            // TypeScript: onAfterMoveSecondarySelf(source:Pokemon, target:Pokemon?, move:Move)
            "AfterMoveSecondarySelf" => {
                if let Some(source_pos) = target {
                    let target_pos = source;
                    let move_id_str = if let Some(ref active_move) = self.active_move {
                        active_move.id.to_string()
                    } else {
                        String::new()
                    };
                    item_callbacks::dispatch_on_after_move_secondary_self(
                        self,
                        item_id.as_str(),
                        source_pos,
                        target_pos,
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

            // TypeScript: onAfterSetStatus(target:Pokemon)
            "AfterSetStatus" => {
                item_callbacks::dispatch_on_after_set_status(self, item_id.as_str(), pokemon_pos)
            }

            "AfterSetStatusPriority" => item_callbacks::dispatch_on_after_set_status_priority(
                self,
                item_id.as_str(),
                pokemon_pos,
            ),

            // TypeScript: onAfterSubDamage(damage:number, target:Pokemon?, source:Pokemon?, effect:Effect?)
            "AfterSubDamage" => {
                let damage = relay_var.unwrap_or(0);
                let effect_id_clone = self.current_effect.clone();
                let effect_id_str = effect_id_clone.as_ref().map(|e| e.as_str());
                item_callbacks::dispatch_on_after_sub_damage(
                    self,
                    item_id.as_str(),
                    damage,
                    target,
                    source,
                    effect_id_str,
                )
            }

            // TypeScript: onAnyAfterMega() - no params
            "AnyAfterMega" => {
                item_callbacks::dispatch_on_any_after_mega(self, item_id.as_str())
            }

            // TypeScript: onAnyAfterMove() - no params
            "AnyAfterMove" => {
                item_callbacks::dispatch_on_any_after_move(self, item_id.as_str())
            }

            // TypeScript: onAnyAfterTerastallization() - no params
            "AnyAfterTerastallization" => {
                item_callbacks::dispatch_on_any_after_terastallization(self, item_id.as_str())
            }

            // TypeScript: onAnyPseudoWeatherChange() - no params
            "AnyPseudoWeatherChange" => {
                item_callbacks::dispatch_on_any_pseudo_weather_change(self, item_id.as_str())
            }

            // TypeScript: onAnySwitchIn() - no params
            "AnySwitchIn" => {
                item_callbacks::dispatch_on_any_switch_in(self, item_id.as_str())
            }

            "AnySwitchInPriority" => item_callbacks::dispatch_on_any_switch_in_priority(
                self,
                item_id.as_str(),
                pokemon_pos,
            ),

            // TypeScript: onAttract(target:Pokemon?, source:Pokemon?)
            "Attract" => {
                item_callbacks::dispatch_on_attract(self, item_id.as_str(), target, source)
            }

            "AttractPriority" => {
                item_callbacks::dispatch_on_attract_priority(self, item_id.as_str(), pokemon_pos)
            }

            // TypeScript: onBasePower(basePower:number, pokemon:Pokemon, target:Pokemon?)
            "BasePower" => {
                let base_power = if let Some(ref active_move) = self.active_move {
                    active_move.base_power
                } else {
                    0
                };
                let target_pos = self.current_event.as_ref().and_then(|e| e.target);
                item_callbacks::dispatch_on_base_power(
                    self,
                    item_id.as_str(),
                    base_power,
                    pokemon_pos,
                    target_pos,
                )
            }

            "BasePowerPriority" => {
                item_callbacks::dispatch_on_base_power_priority(self, item_id.as_str(), pokemon_pos)
            }

            // TypeScript: onChargeMove(pokemon:Pokemon, target:Pokemon?, move:Move)
            "ChargeMove" => {
                let move_id_str = if let Some(ref active_move) = self.active_move {
                    active_move.id.to_string()
                } else {
                    String::new()
                };
                let target_pos = self.current_event.as_ref().and_then(|e| e.target);
                item_callbacks::dispatch_on_charge_move(
                    self,
                    item_id.as_str(),
                    pokemon_pos,
                    target_pos,
                    &move_id_str,
                )
            }

            // TypeScript: onDamage(damage:number, target:Pokemon?, source:Pokemon?, effect:Effect?)
            "Damage" => {
                let damage = relay_var.unwrap_or(0);
                let effect_id_clone = self.current_effect.clone();
                let effect_id_str = effect_id_clone.as_ref().map(|e| e.as_str());
                item_callbacks::dispatch_on_damage(
                    self,
                    item_id.as_str(),
                    damage,
                    target,
                    source,
                    effect_id_str,
                )
            }

            "DamagePriority" => {
                item_callbacks::dispatch_on_damage_priority(self, item_id.as_str(), pokemon_pos)
            }

            // TypeScript: onDamagingHit(damage:number, target:Pokemon, source:Pokemon)
            "DamagingHit" => {
                let damage = relay_var.unwrap_or(0);
                let source_pos = source.unwrap_or(pokemon_pos);
                item_callbacks::dispatch_on_damaging_hit(
                    self,
                    item_id.as_str(),
                    damage,
                    pokemon_pos,
                    source_pos,
                )
            }

            "DamagingHitOrder" => {
                item_callbacks::dispatch_on_damaging_hit_order(self, item_id.as_str(), pokemon_pos)
            }

            // TypeScript: onDisableMove(pokemon:Pokemon)
            "DisableMove" => {
                item_callbacks::dispatch_on_disable_move(self, item_id.as_str(), pokemon_pos)
            }

            "Drive" => item_callbacks::dispatch_on_drive(self, item_id.as_str(), pokemon_pos),

            // TypeScript: onEat(pokemon:Pokemon)
            "Eat" => item_callbacks::dispatch_on_eat(self, item_id.as_str(), pokemon_pos),

            // TypeScript: onEffectiveness(target:Pokemon?)
            "Effectiveness" => {
                item_callbacks::dispatch_on_effectiveness(self, item_id.as_str(), target)
            }

            // TypeScript: onEnd(pokemon:Pokemon)
            "End" => item_callbacks::dispatch_on_end(self, item_id.as_str(), pokemon_pos),

            // TypeScript: onFoeAfterBoost(target:Pokemon?, source:Pokemon?, effect:Effect?, boost:BoostsTable)
            "FoeAfterBoost" => {
                let boost = match relay_var_boost.as_ref() {
                    Some(b) => b,
                    None => return EventResult::Continue,
                };
                let effect_id_clone = self.current_effect.clone();
                let effect_id_str = effect_id_clone.as_ref().map(|e| e.as_str());
                item_callbacks::dispatch_on_foe_after_boost(
                    self,
                    item_id.as_str(),
                    target,
                    source,
                    effect_id_str,
                    boost,
                )
            }

            // TypeScript: onFractionalPriority(pokemon:Pokemon, priority:number)
            "FractionalPriority" => {
                let priority = relay_var_float.unwrap_or(0.0);
                item_callbacks::dispatch_on_fractional_priority(
                    self,
                    item_id.as_str(),
                    pokemon_pos,
                    priority,
                )
            }

            "FractionalPriorityPriority" => {
                item_callbacks::dispatch_on_fractional_priority_priority(
                    self,
                    item_id.as_str(),
                    pokemon_pos,
                )
            }

            // TypeScript: onHit(target:Pokemon?, source:Pokemon?, move:Move)
            "Hit" => {
                let move_id_str = if let Some(ref active_move) = self.active_move {
                    active_move.id.to_string()
                } else {
                    String::new()
                };
                item_callbacks::dispatch_on_hit(
                    self,
                    item_id.as_str(),
                    target,
                    source,
                    &move_id_str,
                )
            }

            // TypeScript: onImmunity(pokemon:Pokemon, type:string)
            "Immunity" => {
                let immunity_type = relay_var_type.as_deref().unwrap_or("");
                item_callbacks::dispatch_on_immunity(
                    self,
                    item_id.as_str(),
                    pokemon_pos,
                    immunity_type,
                )
            }

            // TypeScript: onMaybeTrapPokemon(pokemon:Pokemon)
            "MaybeTrapPokemon" => {
                item_callbacks::dispatch_on_maybe_trap_pokemon(self, item_id.as_str(), pokemon_pos)
            }

            "MaybeTrapPokemonPriority" => item_callbacks::dispatch_on_maybe_trap_pokemon_priority(
                self,
                item_id.as_str(),
                pokemon_pos,
            ),

            "Memory" => item_callbacks::dispatch_on_memory(self, item_id.as_str(), pokemon_pos),

            // TypeScript: onModifyAccuracy() - no params
            "ModifyAccuracy" => {
                item_callbacks::dispatch_on_modify_accuracy(self, item_id.as_str())
            }

            "ModifyAccuracyPriority" => item_callbacks::dispatch_on_modify_accuracy_priority(
                self,
                item_id.as_str(),
                pokemon_pos,
            ),

            // TypeScript: onModifyAtk(pokemon:Pokemon)
            "ModifyAtk" => {
                item_callbacks::dispatch_on_modify_atk(self, item_id.as_str(), pokemon_pos)
            }

            "ModifyAtkPriority" => {
                item_callbacks::dispatch_on_modify_atk_priority(self, item_id.as_str(), pokemon_pos)
            }

            // TypeScript: onModifyCritRatio(pokemon:Pokemon, critRatio:number)
            "ModifyCritRatio" => {
                let crit_ratio = relay_var.unwrap_or(0);
                item_callbacks::dispatch_on_modify_crit_ratio(
                    self,
                    item_id.as_str(),
                    pokemon_pos,
                    crit_ratio,
                )
            }

            // TypeScript: onModifyDamage(damage:number, pokemon:Pokemon, target:Pokemon?)
            "ModifyDamage" => {
                let damage = relay_var.unwrap_or(0);
                let target_pos = self.current_event.as_ref().and_then(|e| e.target);
                item_callbacks::dispatch_on_modify_damage(
                    self,
                    item_id.as_str(),
                    damage,
                    pokemon_pos,
                    target_pos,
                )
            }

            // TypeScript: onModifyDef(pokemon:Pokemon)
            "ModifyDef" => {
                item_callbacks::dispatch_on_modify_def(self, item_id.as_str(), pokemon_pos)
            }

            "ModifyDefPriority" => {
                item_callbacks::dispatch_on_modify_def_priority(self, item_id.as_str(), pokemon_pos)
            }

            // TypeScript: onModifyMove(pokemon:Pokemon, target:Pokemon?)
            "ModifyMove" => {
                let target_pos = self.current_event.as_ref().and_then(|e| e.target);
                item_callbacks::dispatch_on_modify_move(
                    self,
                    item_id.as_str(),
                    pokemon_pos,
                    target_pos,
                )
            }

            "ModifyMovePriority" => item_callbacks::dispatch_on_modify_move_priority(
                self,
                item_id.as_str(),
                pokemon_pos,
            ),

            // TypeScript: onModifySecondaries(pokemon:Pokemon, secondaries:any)
            "ModifySecondaries" => {
                // Temporarily take secondaries out of current_event to get mutable access
                let mut secondaries = self.current_event.as_mut().and_then(|e| e.relay_var_secondaries.take());
                let result = if let Some(ref mut sec) = secondaries {
                    item_callbacks::dispatch_on_modify_secondaries(
                        self,
                        item_id.as_str(),
                        pokemon_pos,
                        sec,
                    )
                } else {
                    EventResult::Continue
                };
                // Put secondaries back into current_event
                if let Some(ref mut event) = self.current_event {
                    event.relay_var_secondaries = secondaries;
                }
                result
            }

            // TypeScript: onModifySpA(pokemon:Pokemon)
            "ModifySpA" => {
                item_callbacks::dispatch_on_modify_sp_a(self, item_id.as_str(), pokemon_pos)
            }

            "ModifySpAPriority" => item_callbacks::dispatch_on_modify_sp_a_priority(
                self,
                item_id.as_str(),
                pokemon_pos,
            ),

            // TypeScript: onModifySpD() - no params
            "ModifySpD" => {
                item_callbacks::dispatch_on_modify_sp_d(self, item_id.as_str(), pokemon_pos)
            }

            "ModifySpDPriority" => item_callbacks::dispatch_on_modify_sp_d_priority(
                self,
                item_id.as_str(),
                pokemon_pos,
            ),

            // TypeScript: onModifySpe(pokemon:Pokemon)
            "ModifySpe" => {
                item_callbacks::dispatch_on_modify_spe(self, item_id.as_str(), pokemon_pos)
            }

            // TypeScript: onModifyWeight(weighthg:number)
            "ModifyWeight" => {
                let weighthg = relay_var.unwrap_or(0);
                item_callbacks::dispatch_on_modify_weight(
                    self,
                    item_id.as_str(),
                    weighthg,
                )
            }

            "NegateImmunity" => {
                item_callbacks::dispatch_on_negate_immunity(self, item_id.as_str(), pokemon_pos)
            }

            "Plate" => item_callbacks::dispatch_on_plate(self, item_id.as_str(), pokemon_pos),

            // TypeScript: onResidual(pokemon:Pokemon)
            "Residual" => item_callbacks::dispatch_on_residual(self, item_id.as_str(), pokemon_pos),

            "ResidualOrder" => {
                item_callbacks::dispatch_on_residual_order(self, item_id.as_str(), pokemon_pos)
            }

            "ResidualSubOrder" => {
                item_callbacks::dispatch_on_residual_sub_order(self, item_id.as_str(), pokemon_pos)
            }

            // TypeScript: onSetAbility(target:Pokemon?, source:Pokemon?, effect:Effect?)
            "SetAbility" => {
                let effect_id_clone = self.current_effect.clone();
                let effect_id_str = effect_id_clone.as_ref().map(|e| e.as_str());
                item_callbacks::dispatch_on_set_ability(
                    self,
                    item_id.as_str(),
                    target,
                    source,
                    effect_id_str,
                )
            }

            // TypeScript: onSourceModifyAccuracy() - no params
            "SourceModifyAccuracy" => {
                let accuracy = relay_var.unwrap_or(0);
                let target_pos = self.current_event.as_ref().and_then(|e| e.target);
                item_callbacks::dispatch_on_source_modify_accuracy(self, item_id.as_str(), accuracy, target_pos)
            }

            "SourceModifyAccuracyPriority" => {
                item_callbacks::dispatch_on_source_modify_accuracy_priority(
                    self,
                    item_id.as_str(),
                    pokemon_pos,
                )
            }

            // TypeScript: onSourceModifyDamage(damage:number, source:Pokemon, target:Pokemon)
            "SourceModifyDamage" => {
                let damage = relay_var.unwrap_or(0);
                let source_pos = pokemon_pos;
                let target_pos = target.unwrap_or(pokemon_pos);
                item_callbacks::dispatch_on_source_modify_damage(
                    self,
                    item_id.as_str(),
                    damage,
                    source_pos,
                    target_pos,
                )
            }

            // TypeScript: onSourceTryPrimaryHit(target:Pokemon?, source:Pokemon?, move:Move)
            "SourceTryPrimaryHit" => {
                let move_id_str = if let Some(ref active_move) = self.active_move {
                    active_move.id.to_string()
                } else {
                    String::new()
                };
                item_callbacks::dispatch_on_source_try_primary_hit(
                    self,
                    item_id.as_str(),
                    target,
                    source,
                    &move_id_str,
                )
            }

            // TypeScript: onStart(target:Pokemon?)
            "Start" => item_callbacks::dispatch_on_start(self, item_id.as_str(), target),

            // JavaScript getCallback() special logic:
            // In gen >= 5, items use onStart callback during SwitchIn event
            // instead of onSwitchIn callback
            // TypeScript: onSwitchIn(pokemon:Pokemon)
            "SwitchIn" => {
                if self.gen >= 5 {
                    let result = item_callbacks::dispatch_on_start(self, item_id.as_str(), target);
                    if !matches!(result, EventResult::Continue) {
                        return result;
                    }
                }
                item_callbacks::dispatch_on_switch_in(self, item_id.as_str(), pokemon_pos)
            }

            "SwitchInPriority" => {
                item_callbacks::dispatch_on_switch_in_priority(self, item_id.as_str(), pokemon_pos)
            }

            // TypeScript: onTakeItem(item:Pokemon?, pokemon:Pokemon, source:Pokemon?)
            "TakeItem" => {
                // item_pos is the position of the item holder being taken from
                // This is a bit odd - in TypeScript it's called "item" but it's actually a Pokemon
                // We'll use None for item_pos and pokemon_pos for the target
                item_callbacks::dispatch_on_take_item(
                    self,
                    item_id.as_str(),
                    None,
                    pokemon_pos,
                    source,
                )
            }

            // TypeScript: onTerrainChange(pokemon:Pokemon)
            "TerrainChange" => {
                item_callbacks::dispatch_on_terrain_change(self, item_id.as_str(), pokemon_pos)
            }

            // TypeScript: onTrapPokemon(pokemon:Pokemon)
            "TrapPokemon" => {
                item_callbacks::dispatch_on_trap_pokemon(self, item_id.as_str(), pokemon_pos)
            }

            "TrapPokemonPriority" => item_callbacks::dispatch_on_trap_pokemon_priority(
                self,
                item_id.as_str(),
                pokemon_pos,
            ),

            // TypeScript: onTryBoost(target:Pokemon, boost:BoostsTable)
            "TryBoost" => {
                // Temporarily take boost out of current_event to get mutable access
                let mut boost = self.current_event.as_mut().and_then(|e| e.relay_var_boost.take());
                let result = if let Some(ref mut b) = boost {
                    item_callbacks::dispatch_on_try_boost(
                        self,
                        item_id.as_str(),
                        pokemon_pos,
                        b,
                    )
                } else {
                    EventResult::Continue
                };
                // Put boost back into current_event
                if let Some(ref mut event) = self.current_event {
                    event.relay_var_boost = boost;
                }
                result
            }

            "TryBoostPriority" => {
                item_callbacks::dispatch_on_try_boost_priority(self, item_id.as_str(), pokemon_pos)
            }

            // TypeScript: onTryEatItem(item:string, pokemon:Pokemon)
            "TryEatItem" => {
                // Get the item being eaten from relay_var_type or active_move
                let item_being_eaten = relay_var_type.as_deref().unwrap_or("");
                item_callbacks::dispatch_on_try_eat_item(
                    self,
                    item_id.as_str(),
                    item_being_eaten,
                    pokemon_pos,
                )
            }

            // TypeScript: onTryHeal(damage:number, target:Pokemon?, source:Pokemon?, effect:Effect?)
            "TryHeal" => {
                let damage = relay_var.unwrap_or(0);
                let effect_id_clone = self.current_effect.clone();
                let effect_id_str = effect_id_clone.as_ref().map(|e| e.as_str());
                item_callbacks::dispatch_on_try_heal(
                    self,
                    item_id.as_str(),
                    damage,
                    target,
                    source,
                    effect_id_str,
                )
            }

            "TryHealPriority" => {
                item_callbacks::dispatch_on_try_heal_priority(self, item_id.as_str(), pokemon_pos)
            }

            // TypeScript: onTryHit(target:Pokemon, source:Pokemon)
            "TryHit" => {
                if let Some(source_pos) = source {
                    item_callbacks::dispatch_on_try_hit(
                        self,
                        item_id.as_str(),
                        pokemon_pos,
                        source_pos,
                    )
                } else {
                    EventResult::Continue
                }
            }

            // TypeScript: onUpdate(pokemon:Pokemon)
            "Update" => item_callbacks::dispatch_on_update(self, item_id.as_str(), pokemon_pos),

            // TypeScript: onUse(pokemon:Pokemon)
            "Use" => item_callbacks::dispatch_on_use(self, item_id.as_str(), pokemon_pos),

            // TypeScript: onUseItem(item:string, pokemon:Pokemon)
            "UseItem" => {
                // Get the item being used from relay_var_type
                let item_being_used = relay_var_type.as_deref().unwrap_or("");
                item_callbacks::dispatch_on_use_item(
                    self,
                    item_id.as_str(),
                    item_being_used,
                    pokemon_pos,
                )
            }

            _ => EventResult::Continue,
        }
    }
}
