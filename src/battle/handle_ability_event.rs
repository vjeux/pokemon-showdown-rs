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
        target: Option<(usize, usize)>,
    ) -> crate::event::EventResult {
        use crate::data::ability_callbacks;
        use crate::event::EventResult;

        let pokemon_pos = target.unwrap_or((0, 0));

        match event_id {
            "AfterBoost" => {
                ability_callbacks::dispatch_on_after_boost(self, ability_id.as_str(), Some(pokemon_pos), None, None)
            }
            "AfterEachBoost" => ability_callbacks::dispatch_on_after_each_boost(
                self,
                ability_id.as_str(),
                Some(pokemon_pos),
                None,
                None,
            ),
            "AfterMoveSecondary" => ability_callbacks::dispatch_on_after_move_secondary(
                self,
                ability_id.as_str(),
                pokemon_pos,
                (0, 0), // TODO: Wire through actual source_pos
                "", // TODO: Wire through actual move_id
            ),
            "AfterMoveSecondarySelf" => ability_callbacks::dispatch_on_after_move_secondary_self(
                self,
                ability_id.as_str(),
                pokemon_pos,
                (0, 0), // TODO: Wire through actual target_pos
                "", // TODO: Wire through actual move_id
            ),
            "AfterSetStatus" => ability_callbacks::dispatch_on_after_set_status(
                self,
                ability_id.as_str(),
                None, // TODO: Wire through actual status
                Some(pokemon_pos),
                None,
                None,
            ),
            "AfterTerastallization" => ability_callbacks::dispatch_on_after_terastallization(
                self,
                ability_id.as_str(),
                pokemon_pos,
            ),
            "AfterUseItem" => ability_callbacks::dispatch_on_after_use_item(
                self,
                ability_id.as_str(),
                pokemon_pos,
            ),
            "AllyAfterUseItem" => ability_callbacks::dispatch_on_ally_after_use_item(
                self,
                ability_id.as_str(),
                pokemon_pos,
            ),
            "AllyBasePower" => ability_callbacks::dispatch_on_ally_base_power(
                self,
                ability_id.as_str(),
                0,
            "", // TODO: Wire through actual move_id
            ),
            "AllyBasePowerPriority" => ability_callbacks::dispatch_on_ally_base_power_priority(
                self,
                ability_id.as_str(),
                0,
            "", // TODO: Wire through actual move_id
            ),
            "AllyFaint" => {
                ability_callbacks::dispatch_on_ally_faint(self, ability_id.as_str(), Some(pokemon_pos))
            }
            "AllyModifyAtk" => ability_callbacks::dispatch_on_ally_modify_atk(
                self,
                ability_id.as_str(),
                pokemon_pos,
            ),
            "AllyModifyAtkPriority" => ability_callbacks::dispatch_on_ally_modify_atk_priority(
                self,
                ability_id.as_str(),
                pokemon_pos,
            ),
            "AllyModifySpD" => ability_callbacks::dispatch_on_ally_modify_sp_d(
                self,
                ability_id.as_str(),
                pokemon_pos,
            ),
            "AllyModifySpDPriority" => ability_callbacks::dispatch_on_ally_modify_sp_d_priority(
                self,
                ability_id.as_str(),
                pokemon_pos,
            ),
            "AllySetStatus" => ability_callbacks::dispatch_on_ally_set_status(
                self,
                ability_id.as_str(),
                "", // TODO: Wire through actual status_id
                pokemon_pos,
                None,
                None,
            ),
            "AllyTryAddVolatile" => ability_callbacks::dispatch_on_ally_try_add_volatile(
                self,
                ability_id.as_str(),
                None, // TODO: Wire through actual status
                Some(pokemon_pos),
                None,
                None,
            ),
            "AllyTryBoost" => ability_callbacks::dispatch_on_ally_try_boost(
                self,
                ability_id.as_str(),
                Some(pokemon_pos),
                None,
                None,
            ),
            "AllyTryHitSide" => ability_callbacks::dispatch_on_ally_try_hit_side(
                self,
                ability_id.as_str(),
                Some(pokemon_pos),
                None,
                "", // TODO: Wire through actual move_id
            ),
            "AnyAccuracy" => ability_callbacks::dispatch_on_any_accuracy(
                self,
                ability_id.as_str(),
                100, // TODO: Wire through actual accuracy
                Some(pokemon_pos),
                None,
                "", // TODO: Wire through actual move_id
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
                None, // TODO: Wire through actual status
                Some(pokemon_pos),
                None,
                None,
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
                0, // TODO: Wire through actual base_power
                Some(pokemon_pos),
                None, // TODO: Wire through actual target_pos
                "", // TODO: Wire through actual move_id
            ),
            "AnyBasePowerPriority" => ability_callbacks::dispatch_on_any_base_power_priority(
                self,
                ability_id.as_str(),
                0, // TODO: Wire through actual base_power
                Some(pokemon_pos),
                None, // TODO: Wire through actual target_pos
                "", // TODO: Wire through actual move_id
            ),
            "AnyBeforeMove" => ability_callbacks::dispatch_on_any_before_move(
                self,
                ability_id.as_str(),
            ),
            "AnyDamage" => {
                ability_callbacks::dispatch_on_any_damage(self, ability_id.as_str(), 0, Some(pokemon_pos), None, None)
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
                None,
                "", // TODO: Wire through actual move_id
            ),
            "AnyInvulnerabilityPriority" => {
                ability_callbacks::dispatch_on_any_invulnerability_priority(
                    self,
                    ability_id.as_str(),
                    Some(pokemon_pos),
                    None,
                    "", // TODO: Wire through actual move_id
                )
            }
            "AnyModifyAccuracy" => ability_callbacks::dispatch_on_any_modify_accuracy(self, ability_id.as_str(), 0, Some(pokemon_pos), None),
            "AnyModifyAccuracyPriority" => {
                ability_callbacks::dispatch_on_any_modify_accuracy_priority(self, ability_id.as_str(), 0, Some(pokemon_pos), None)
            }
            "AnyModifyAtk" => ability_callbacks::dispatch_on_any_modify_atk(
                self,
                ability_id.as_str(),
                None, // TODO: Wire through source_pos
                Some(pokemon_pos),
                "", // TODO: Wire through actual move_id
            ),
            "AnyModifyBoost" => ability_callbacks::dispatch_on_any_modify_boost(
                self,
                ability_id.as_str(),
                "", // TODO: Wire through boosts
                pokemon_pos,
            ),
            "AnyModifyDamage" => ability_callbacks::dispatch_on_any_modify_damage(
                self,
                ability_id.as_str(),
                0, // TODO: Wire through damage
                None, // TODO: Wire through source_pos
                Some(pokemon_pos),
                "", // TODO: Wire through actual move_id
            ),
            "AnyModifyDef" => ability_callbacks::dispatch_on_any_modify_def(
                self,
                ability_id.as_str(),
                None, // TODO: Wire through source_pos
                Some(pokemon_pos),
                "", // TODO: Wire through actual move_id
            ),
            "AnyModifySpA" => ability_callbacks::dispatch_on_any_modify_sp_a(
                self,
                ability_id.as_str(),
                None, // TODO: Wire through source_pos
                Some(pokemon_pos),
                "", // TODO: Wire through actual move_id
            ),
            "AnyModifySpD" => ability_callbacks::dispatch_on_any_modify_sp_d(
                self,
                ability_id.as_str(),
                None, // TODO: Wire through source_pos
                Some(pokemon_pos),
                "", // TODO: Wire through actual move_id
            ),
            "AnyRedirectTarget" => ability_callbacks::dispatch_on_any_redirect_target(
                self,
                ability_id.as_str(),
                Some(pokemon_pos),
                None, // TODO: Wire through source_pos
                "", // TODO: Wire through actual move_id
            ),
            "AnySetWeather" => ability_callbacks::dispatch_on_any_set_weather(self, ability_id.as_str(), Some(pokemon_pos), None),
            "AnySwitchIn" => {
                ability_callbacks::dispatch_on_any_switch_in(self, ability_id.as_str())
            }
            "AnySwitchInPriority" => ability_callbacks::dispatch_on_any_switch_in_priority(
                self,
                ability_id.as_str(),
            ),
            "AnyTryMove" => {
                ability_callbacks::dispatch_on_any_try_move(self, ability_id.as_str(), Some(pokemon_pos), None, None)
            }
            "AnyTryPrimaryHit" => ability_callbacks::dispatch_on_any_try_primary_hit(
                self,
                ability_id.as_str(),
                Some(pokemon_pos),
                None, // TODO: Wire through source_pos
                "", // TODO: Wire through actual move_id
            ),
            "BasePower" => {
                ability_callbacks::dispatch_on_base_power(self, ability_id.as_str(), 0, pokemon_pos, (0, 0), "") // TODO: Wire through actual move_id
            }
            "BasePowerPriority" => ability_callbacks::dispatch_on_base_power_priority(
                self,
                ability_id.as_str(),
                0, pokemon_pos, pokemon_pos,
            "", // TODO: Wire through actual move_id
            ),
            "BeforeMove" => {
                ability_callbacks::dispatch_on_before_move(self, ability_id.as_str(), pokemon_pos, None, "") // TODO: Wire through actual move_id
            }
            "BeforeMovePriority" => ability_callbacks::dispatch_on_before_move_priority(
                self,
                ability_id.as_str(),
                pokemon_pos,
            None,
            "", // TODO: Wire through actual move_id
            ),
            "BeforeSwitchIn" => ability_callbacks::dispatch_on_before_switch_in(
                self,
                ability_id.as_str(),
                pokemon_pos,
            ),
            "ChangeBoost" => {
                ability_callbacks::dispatch_on_change_boost(self, ability_id.as_str(), Some(pokemon_pos), None, None)
            }
            "CheckShow" => {
                ability_callbacks::dispatch_on_check_show(self, ability_id.as_str(), pokemon_pos)
            }
            "CriticalHit" => {
                ability_callbacks::dispatch_on_critical_hit(self, ability_id.as_str(), Some(pokemon_pos), None, "") // TODO: Wire through actual move_id
            }
            "Damage" => {
                ability_callbacks::dispatch_on_damage(self, ability_id.as_str(), 0, pokemon_pos, None, None)
            }
            "DamagePriority" => ability_callbacks::dispatch_on_damage_priority(
                self,
                ability_id.as_str(),
                0, pokemon_pos, None, None
            ),
            "DamagingHit" => {
                ability_callbacks::dispatch_on_damaging_hit(self, ability_id.as_str(), 0, Some(pokemon_pos), None, "") // TODO: Wire through actual move_id
            }
            "DamagingHitOrder" => ability_callbacks::dispatch_on_damaging_hit_order(
                self,
                ability_id.as_str(),
                0,
            Some(pokemon_pos), None,
            "", // TODO: Wire through actual move_id
            ),
            "DeductPP" => {
                ability_callbacks::dispatch_on_deduct_p_p(self, ability_id.as_str(), Some(pokemon_pos), None)
            }
            "DisableMove" => {
                ability_callbacks::dispatch_on_disable_move(self, ability_id.as_str(), pokemon_pos)
            }
            "DragOut" => {
                ability_callbacks::dispatch_on_drag_out(self, ability_id.as_str(), pokemon_pos)
            }
            "DragOutPriority" => ability_callbacks::dispatch_on_drag_out_priority(
                self,
                ability_id.as_str(),
                pokemon_pos,
            ),
            "EatItem" => {
                ability_callbacks::dispatch_on_eat_item(self, ability_id.as_str(), pokemon_pos)
            }
            "Effectiveness" => {
                ability_callbacks::dispatch_on_effectiveness(self, ability_id.as_str(), 0, pokemon_pos, "", "") // TODO: Wire through actual move_id
            }
            "EmergencyExit" => ability_callbacks::dispatch_on_emergency_exit(
                self,
                ability_id.as_str(),
                Some(pokemon_pos)
            ),
            "End" => ability_callbacks::dispatch_on_end(self, ability_id.as_str(), pokemon_pos),
            "Faint" => ability_callbacks::dispatch_on_faint(self, ability_id.as_str(), pokemon_pos),
            "Flinch" => {
                ability_callbacks::dispatch_on_flinch(self, ability_id.as_str(), pokemon_pos)
            }
            "FoeAfterBoost" => ability_callbacks::dispatch_on_foe_after_boost(
                self,
                ability_id.as_str(),
                Some(pokemon_pos), None, None
            ),
            "FoeMaybeTrapPokemon" => ability_callbacks::dispatch_on_foe_maybe_trap_pokemon(
                self,
                ability_id.as_str(),
                pokemon_pos, None
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
                ability_callbacks::dispatch_on_foe_try_move(self, ability_id.as_str(), Some(pokemon_pos), None, "") // TODO: Wire through actual move_id
            }
            "FractionalPriority" => ability_callbacks::dispatch_on_fractional_priority(
                self,
                ability_id.as_str(),
                pokemon_pos,
            None,
            "", // TODO: Wire through actual move_id
            ),
            "FractionalPriorityPriority" => {
                ability_callbacks::dispatch_on_fractional_priority_priority(
                    self,
                    ability_id.as_str(),
                    pokemon_pos,
                None,
                "", // TODO: Wire through actual move_id
            )
            }
            "Hit" => ability_callbacks::dispatch_on_hit(self, ability_id.as_str(), pokemon_pos, (0, 0), ""), // TODO: Wire through actual move_id
            "Immunity" => {
                ability_callbacks::dispatch_on_immunity(self, ability_id.as_str(), "", pokemon_pos)
            }
            "ModifyAccuracy" => ability_callbacks::dispatch_on_modify_accuracy(
                self,
                ability_id.as_str(),
                0, pokemon_pos, pokemon_pos,
            "", // TODO: Wire through actual move_id
            ),
            "ModifyAccuracyPriority" => ability_callbacks::dispatch_on_modify_accuracy_priority(
                self,
                ability_id.as_str(),
                0, pokemon_pos, pokemon_pos,
            "", // TODO: Wire through actual move_id
            ),
            "ModifyAtk" => {
                ability_callbacks::dispatch_on_modify_atk(self, ability_id.as_str(), 0, pokemon_pos, (0, 0), "") // TODO: Wire through actual move_id
            }
            "ModifyAtkPriority" => ability_callbacks::dispatch_on_modify_atk_priority(
                self,
                ability_id.as_str(),
                0, pokemon_pos, pokemon_pos,
            "", // TODO: Wire through actual move_id
            ),
            "ModifyCritRatio" => ability_callbacks::dispatch_on_modify_crit_ratio(self, ability_id.as_str(), 0, pokemon_pos, None),
            "ModifyDamage" => {
                ability_callbacks::dispatch_on_modify_damage(self, ability_id.as_str(), 0, pokemon_pos, (0, 0), "") // TODO: Wire through actual move_id
            }
            "ModifyDef" => {
                ability_callbacks::dispatch_on_modify_def(self, ability_id.as_str(), 0, pokemon_pos, (0, 0), "")
            }
            "ModifyDefPriority" => ability_callbacks::dispatch_on_modify_def_priority(self, ability_id.as_str(), 0, pokemon_pos, (0, 0), ""),
            "ModifyMove" => {
                ability_callbacks::dispatch_on_modify_move(self, ability_id.as_str(), "") // TODO: Wire through actual move_id
            }
            "ModifyMovePriority" => ability_callbacks::dispatch_on_modify_move_priority(
                self,
                ability_id.as_str(),
            "", // TODO: Wire through actual move_id
            ),
            "ModifyPriority" => ability_callbacks::dispatch_on_modify_priority(
                self,
                ability_id.as_str(),
                0, pokemon_pos, None,
            "", // TODO: Wire through actual move_id
            ),
            "ModifySTAB" => ability_callbacks::dispatch_on_modify_s_t_a_b(
                self,
                ability_id.as_str(),
                None, Some(pokemon_pos),
            "", // TODO: Wire through actual move_id
            ),
            "ModifySecondaries" => ability_callbacks::dispatch_on_modify_secondaries(self, ability_id.as_str()),
            "ModifySpA" => {
                ability_callbacks::dispatch_on_modify_sp_a(self, ability_id.as_str(), 0, pokemon_pos, (0, 0), "") // TODO: Wire through actual move_id
            }
            "ModifySpAPriority" => ability_callbacks::dispatch_on_modify_sp_a_priority(
                self,
                ability_id.as_str(),
                0, pokemon_pos, pokemon_pos,
            "", // TODO: Wire through actual move_id
            ),
            "ModifySpe" => {
                ability_callbacks::dispatch_on_modify_spe(self, ability_id.as_str(), 0, pokemon_pos)
            }
            "ModifyType" => {
                ability_callbacks::dispatch_on_modify_type(self, ability_id.as_str(), "", pokemon_pos, None) // TODO: Wire through actual move_id
            }
            "ModifyTypePriority" => ability_callbacks::dispatch_on_modify_type_priority(
                self,
                ability_id.as_str(),
                "",
            pokemon_pos, None, // TODO: Wire through actual move_id
            ),
            "ModifyWeight" => {
                ability_callbacks::dispatch_on_modify_weight(self, ability_id.as_str(), 0, pokemon_pos)
            }
            "ModifyWeightPriority" => ability_callbacks::dispatch_on_modify_weight_priority(self, ability_id.as_str(), 0, pokemon_pos),
            "PrepareHit" => {
                ability_callbacks::dispatch_on_prepare_hit(self, ability_id.as_str(), None, Some(pokemon_pos), "") // TODO: Wire through actual move_id
            }
            "Residual" => {
                ability_callbacks::dispatch_on_residual(self, ability_id.as_str(), pokemon_pos)
            }
            "ResidualOrder" => ability_callbacks::dispatch_on_residual_order(
                self,
                ability_id.as_str(),
                pokemon_pos,
            ),
            "ResidualSubOrder" => ability_callbacks::dispatch_on_residual_sub_order(
                self,
                ability_id.as_str(),
                pokemon_pos,
            ),
            "SetStatus" => {
                ability_callbacks::dispatch_on_set_status(self, ability_id.as_str(), "", pokemon_pos, None, None)
            }
            "SideConditionStart" => ability_callbacks::dispatch_on_side_condition_start(
                self,
                ability_id.as_str(),
                None
            ),
            "SourceAfterFaint" => ability_callbacks::dispatch_on_source_after_faint(self, ability_id.as_str(), Some(pokemon_pos), None, None),
            "SourceBasePower" => ability_callbacks::dispatch_on_source_base_power(
                self,
                ability_id.as_str(),
                0,
            "", // TODO: Wire through actual move_id
            ),
            "SourceBasePowerPriority" => ability_callbacks::dispatch_on_source_base_power_priority(
                self,
                ability_id.as_str(),
                0,
            "", // TODO: Wire through actual move_id
            ),
            "SourceDamagingHit" => ability_callbacks::dispatch_on_source_damaging_hit(
                self,
                ability_id.as_str(),
                0, Some(pokemon_pos), None,
            "", // TODO: Wire through actual move_id
            ),
            "SourceModifyAccuracy" => ability_callbacks::dispatch_on_source_modify_accuracy(
                self,
                ability_id.as_str(),
                0, pokemon_pos, pokemon_pos,
            "", // TODO: Wire through actual move_id
            ),
            "SourceModifyAccuracyPriority" => {
                ability_callbacks::dispatch_on_source_modify_accuracy_priority(
                    self,
                    ability_id.as_str(),
                    0, pokemon_pos, pokemon_pos,
                "", // TODO: Wire through actual move_id
            )
            }
            "SourceModifyAtk" => ability_callbacks::dispatch_on_source_modify_atk(
                self,
                ability_id.as_str(),
            "", // TODO: Wire through actual move_id
            ),
            "SourceModifyAtkPriority" => ability_callbacks::dispatch_on_source_modify_atk_priority(
                self,
                ability_id.as_str(),
            "", // TODO: Wire through actual move_id
            ),
            "SourceModifyDamage" => ability_callbacks::dispatch_on_source_modify_damage(
                self,
                ability_id.as_str(),
                0, pokemon_pos, pokemon_pos,
            "", // TODO: Wire through actual move_id
            ),
            "SourceModifyDamagePriority" => {
                ability_callbacks::dispatch_on_source_modify_damage_priority(
                    self,
                    ability_id.as_str(),
                    0, pokemon_pos, pokemon_pos,
                "", // TODO: Wire through actual move_id
            )
            }
            "SourceModifySecondaries" => ability_callbacks::dispatch_on_source_modify_secondaries(
                self,
                ability_id.as_str(),
                Some(pokemon_pos), None,
            "", // TODO: Wire through actual move_id
            ),
            "SourceModifySpA" => ability_callbacks::dispatch_on_source_modify_sp_a(
                self,
                ability_id.as_str(),
            "", // TODO: Wire through actual move_id
            ),
            "SourceModifySpAPriority" => {
                ability_callbacks::dispatch_on_source_modify_sp_a_priority(
                    self,
                    ability_id.as_str(),
                "", // TODO: Wire through actual move_id
            )
            }
            "SourceTryHeal" => ability_callbacks::dispatch_on_source_try_heal(
                self,
                ability_id.as_str(),
                0, Some(pokemon_pos), None, None
            ),
            "SourceTryPrimaryHit" => ability_callbacks::dispatch_on_source_try_primary_hit(
                self,
                ability_id.as_str(),
                Some(pokemon_pos), None, None
            ),
            "Start" => ability_callbacks::dispatch_on_start(self, ability_id.as_str(), pokemon_pos),
            "SwitchIn" => {
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
            "TryAddVolatile" => ability_callbacks::dispatch_on_try_add_volatile(
                self,
                ability_id.as_str(),
                "", pokemon_pos, None, None
            ),
            "TryBoost" => {
                ability_callbacks::dispatch_on_try_boost(self, ability_id.as_str(), "", pokemon_pos, None, None)
            }
            "TryBoostPriority" => ability_callbacks::dispatch_on_try_boost_priority(
                self,
                ability_id.as_str(),
                "", pokemon_pos, None, None
            ),
            "TryEatItem" => {
                ability_callbacks::dispatch_on_try_eat_item(self, ability_id.as_str())
            }
            "TryEatItemPriority" => ability_callbacks::dispatch_on_try_eat_item_priority(
                self,
                ability_id.as_str()
            ),
            "TryHeal" => {
                ability_callbacks::dispatch_on_try_heal(self, ability_id.as_str(), 0, Some(pokemon_pos), None, None)
            }
            "TryHit" => {
                ability_callbacks::dispatch_on_try_hit(self, ability_id.as_str(), pokemon_pos, (0, 0), "") // TODO: Wire through actual move_id
            }
            "TryHitPriority" => ability_callbacks::dispatch_on_try_hit_priority(
                self,
                ability_id.as_str(),
                pokemon_pos, pokemon_pos,
            "", // TODO: Wire through actual move_id
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
        }
    }
}
