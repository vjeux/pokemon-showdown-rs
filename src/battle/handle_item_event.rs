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
        target: Option<&crate::event::EventTarget>,
    ) -> crate::event::EventResult {
        use crate::data::item_callbacks;
        use crate::event::EventResult;

        // Extract pokemon position from EventTarget
        let pokemon_pos = target.and_then(|t| t.as_pokemon()).unwrap_or((0, 0));
        let target_opt = Some(pokemon_pos);

        eprintln!("[HANDLE_ITEM_EVENT] event_id={}, item_id={}, target={:?}",
            event_id, item_id.as_str(), target_opt);

        let source = self.current_event.as_ref().and_then(|e| e.source);
        let relay_var = self.current_event.as_ref().and_then(|e| e.relay_var.clone());

        match event_id {
            // TypeScript: onAfterBoost(target:Pokemon, boost:BoostsTable)
            "AfterBoost" => {
                let boost = match &relay_var {
                    Some(EventResult::Boost(b)) => b,
                    _ => return EventResult::Continue,
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
                    target_opt,
                    source,
                    &move_id_str,
                )
            }

            // TypeScript: onAfterMoveSecondarySelf(source:Pokemon, target:Pokemon?, move:Move)
            "AfterMoveSecondarySelf" => {
                if let Some(source_pos) = target_opt {
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

            // TypeScript: onAfterSetStatus(target:Pokemon)
            "AfterSetStatus" => {
                item_callbacks::dispatch_on_after_set_status(self, item_id.as_str(), pokemon_pos)
            }

            // TypeScript: onAfterSubDamage(damage:number, target:Pokemon?, source:Pokemon?, effect:Effect?)
            "AfterSubDamage" => {
                let damage = match &relay_var { Some(EventResult::Number(n)) => *n, _ => 0 };
                let effect_id_clone = self.current_effect.clone();
                let effect_id_str = effect_id_clone.as_ref().map(|e| e.as_str());
                item_callbacks::dispatch_on_after_sub_damage(
                    self,
                    item_id.as_str(),
                    damage,
                    target_opt,
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

            // TypeScript: onAttract(target:Pokemon?, source:Pokemon?)
            "Attract" => {
                item_callbacks::dispatch_on_attract(self, item_id.as_str(), target_opt, source)
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
                let damage = match &relay_var { Some(EventResult::Number(n)) => *n, _ => 0 };
                let effect_id_clone = self.current_effect.clone();
                let effect_id_str = effect_id_clone.as_ref().map(|e| e.as_str());
                item_callbacks::dispatch_on_damage(
                    self,
                    item_id.as_str(),
                    damage,
                    target_opt,
                    source,
                    effect_id_str,
                )
            }

            // TypeScript: onDamagingHit(damage:number, target:Pokemon, source:Pokemon)
            "DamagingHit" => {
                let damage = match &relay_var { Some(EventResult::Number(n)) => *n, _ => 0 };
                let source_pos = source.unwrap_or(pokemon_pos);
                item_callbacks::dispatch_on_damaging_hit(
                    self,
                    item_id.as_str(),
                    damage,
                    pokemon_pos,
                    source_pos,
                )
            }

            // TypeScript: onDisableMove(pokemon:Pokemon)
            "DisableMove" => {
                item_callbacks::dispatch_on_disable_move(self, item_id.as_str(), pokemon_pos)
            }

            // TypeScript: onEat(pokemon:Pokemon)
            "Eat" => item_callbacks::dispatch_on_eat(self, item_id.as_str(), pokemon_pos),

            // TypeScript: onEffectiveness(target:Pokemon?)
            "Effectiveness" => {
                item_callbacks::dispatch_on_effectiveness(self, item_id.as_str(), target_opt)
            }

            // TypeScript: onEnd(pokemon:Pokemon)
            "End" => item_callbacks::dispatch_on_end(self, item_id.as_str(), pokemon_pos),

            // TypeScript: onFoeAfterBoost(target:Pokemon?, source:Pokemon?, effect:Effect?, boost:BoostsTable)
            "FoeAfterBoost" => {
                let boost = match &relay_var {
                    Some(EventResult::Boost(b)) => b,
                    _ => return EventResult::Continue,
                };
                let effect_id_clone = self.current_effect.clone();
                let effect_id_str = effect_id_clone.as_ref().map(|e| e.as_str());
                item_callbacks::dispatch_on_foe_after_boost(
                    self,
                    item_id.as_str(),
                    target_opt,
                    source,
                    effect_id_str,
                    boost,
                )
            }

            // TypeScript: onFractionalPriority(pokemon:Pokemon, priority:number)
            "FractionalPriority" => {
                let priority = match &relay_var {
                    Some(EventResult::Float(f)) => *f,
                    _ => 0.0,
                };
                item_callbacks::dispatch_on_fractional_priority(
                    self,
                    item_id.as_str(),
                    pokemon_pos,
                    priority,
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
                    target_opt,
                    source,
                    &move_id_str,
                )
            }

            // TypeScript: onImmunity(pokemon:Pokemon, type:string)
            "Immunity" => {
                let immunity_type = match &relay_var {
                    Some(EventResult::String(s)) => s.as_str(),
                    _ => "",
                };
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

            // TypeScript: onModifyAccuracy() - no params
            "ModifyAccuracy" => {
                item_callbacks::dispatch_on_modify_accuracy(self, item_id.as_str())
            }

            // TypeScript: onModifyAtk(pokemon:Pokemon)
            "ModifyAtk" => {
                item_callbacks::dispatch_on_modify_atk(self, item_id.as_str(), pokemon_pos)
            }

            // TypeScript: onModifyCritRatio(pokemon:Pokemon, critRatio:number)
            "ModifyCritRatio" => {
                let crit_ratio = match &relay_var { Some(EventResult::Number(n)) => *n, _ => 0 };
                item_callbacks::dispatch_on_modify_crit_ratio(
                    self,
                    item_id.as_str(),
                    pokemon_pos,
                    crit_ratio,
                )
            }

            // TypeScript: onModifyDamage(damage:number, pokemon:Pokemon, target:Pokemon?)
            "ModifyDamage" => {
                let damage = match &relay_var { Some(EventResult::Number(n)) => *n, _ => 0 };
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

            // TypeScript: onModifySecondaries(pokemon:Pokemon, secondaries:any)
            "ModifySecondaries" => {
                // Temporarily take secondaries out of current_event to get mutable access
                let mut secondaries = self.current_event.as_mut().and_then(|e| {
                    let relay_var = e.relay_var.take();
                    match relay_var {
                        Some(EventResult::Secondaries(s)) => Some(s),
                        _ => {
                            // Put it back if it wasn't Secondaries
                            e.relay_var = relay_var;
                            None
                        }
                    }
                });
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
                if let Some(sec) = secondaries {
                    if let Some(ref mut event) = self.current_event {
                        event.relay_var = Some(EventResult::Secondaries(sec));
                    }
                }
                result
            }

            // TypeScript: onModifySpA(pokemon:Pokemon)
            "ModifySpA" => {
                item_callbacks::dispatch_on_modify_sp_a(self, item_id.as_str(), pokemon_pos)
            }

            // TypeScript: onModifySpD() - no params
            "ModifySpD" => {
                item_callbacks::dispatch_on_modify_sp_d(self, item_id.as_str(), pokemon_pos)
            }

            // TypeScript: onModifySpe(pokemon:Pokemon)
            "ModifySpe" => {
                item_callbacks::dispatch_on_modify_spe(self, item_id.as_str(), pokemon_pos)
            }

            // TypeScript: onModifyWeight(weighthg:number)
            "ModifyWeight" => {
                let weighthg = match &relay_var { Some(EventResult::Number(n)) => *n, _ => 0 };
                item_callbacks::dispatch_on_modify_weight(
                    self,
                    item_id.as_str(),
                    weighthg,
                )
            }

            // TypeScript: onResidual(pokemon:Pokemon)
            "Residual" => item_callbacks::dispatch_on_residual(self, item_id.as_str(), pokemon_pos),

            // TypeScript: onSetAbility(target:Pokemon?, source:Pokemon?, effect:Effect?)
            "SetAbility" => {
                let effect_id_clone = self.current_effect.clone();
                let effect_id_str = effect_id_clone.as_ref().map(|e| e.as_str());
                item_callbacks::dispatch_on_set_ability(
                    self,
                    item_id.as_str(),
                    target_opt,
                    source,
                    effect_id_str,
                )
            }

            // TypeScript: onSourceModifyAccuracy() - no params
            "SourceModifyAccuracy" => {
                let accuracy = match &relay_var { Some(EventResult::Number(n)) => *n, _ => 0 };
                let target_pos = self.current_event.as_ref().and_then(|e| e.target);
                item_callbacks::dispatch_on_source_modify_accuracy(self, item_id.as_str(), accuracy, target_pos)
            }

            // TypeScript: onSourceModifyDamage(damage:number, source:Pokemon, target:Pokemon)
            "SourceModifyDamage" => {
                let damage = match &relay_var { Some(EventResult::Number(n)) => *n, _ => 0 };
                let source_pos = pokemon_pos;
                let target_pos = target_opt.unwrap_or(pokemon_pos);
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
                    target_opt,
                    source,
                    &move_id_str,
                )
            }

            // TypeScript: onStart(target:Pokemon?)
            "Start" => item_callbacks::dispatch_on_start(self, item_id.as_str(), target_opt),

            // JavaScript getCallback() special logic:
            // In gen >= 5, items use onStart callback during SwitchIn event
            // instead of onSwitchIn callback
            // TypeScript: onSwitchIn(pokemon:Pokemon)
            "SwitchIn" => {
                if self.gen >= 5 {
                    let result = item_callbacks::dispatch_on_start(self, item_id.as_str(), target_opt);
                    if !matches!(result, EventResult::Continue) {
                        return result;
                    }
                }
                item_callbacks::dispatch_on_switch_in(self, item_id.as_str(), pokemon_pos)
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

            // TypeScript: onTryBoost(target:Pokemon, boost:BoostsTable)
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
                if let Some(b) = boost {
                    if let Some(ref mut event) = self.current_event {
                        event.relay_var = Some(EventResult::Boost(b));
                    }
                }
                result
            }

            // TypeScript: onTryEatItem(item:string, pokemon:Pokemon)
            "TryEatItem" => {
                // Get the item being eaten from relay_var
                let item_being_eaten = match &relay_var {
                    Some(EventResult::String(s)) => s.as_str(),
                    _ => "",
                };
                item_callbacks::dispatch_on_try_eat_item(
                    self,
                    item_id.as_str(),
                    item_being_eaten,
                    pokemon_pos,
                )
            }

            // TypeScript: onTryHeal(damage:number, target:Pokemon?, source:Pokemon?, effect:Effect?)
            "TryHeal" => {
                let damage = match &relay_var { Some(EventResult::Number(n)) => *n, _ => 0 };
                let effect_id_clone = self.current_effect.clone();
                let effect_id_str = effect_id_clone.as_ref().map(|e| e.as_str());
                item_callbacks::dispatch_on_try_heal(
                    self,
                    item_id.as_str(),
                    damage,
                    target_opt,
                    source,
                    effect_id_str,
                )
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
                // Get the item being used from relay_var
                let item_being_used = match &relay_var {
                    Some(EventResult::String(s)) => s.as_str(),
                    _ => "",
                };
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
