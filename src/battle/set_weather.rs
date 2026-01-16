// Set Weather
//
// JavaScript: field.ts setWeather()
//
// setWeather(status: string | Condition, source: Pokemon | 'debug' | null = null, sourceEffect: Effect | null = null) {
//     status = this.battle.dex.conditions.get(status);
//     if (!sourceEffect && this.battle.effect) sourceEffect = this.battle.effect;
//     if (!source && this.battle.event?.target) source = this.battle.event.target;
//     if (source === 'debug') source = this.battle.sides[0].active[0];
//
//     if (this.weather === status.id) {
//         if (sourceEffect && sourceEffect.effectType === 'Ability') {
//             if (this.battle.gen > 5 || this.weatherState.duration === 0) {
//                 return false;
//             }
//         } else if (this.battle.gen > 2 || status.id === 'sandstorm') {
//             return false;
//         }
//     }
//     if (source) {
//         const result = this.battle.runEvent('SetWeather', source, source, status);
//         if (!result) {
//             if (result === false) {
//                 if ((sourceEffect as Move)?.weather) {
//                     this.battle.add('-fail', source, sourceEffect, '[from] ' + this.weather);
//                 } else if (sourceEffect && sourceEffect.effectType === 'Ability') {
//                     this.battle.add('-ability', source, sourceEffect, '[from] ' + this.weather, '[fail]');
//                 }
//             }
//             return null;
//         }
//     }
//     const prevWeather = this.weather;
//     const prevWeatherState = this.weatherState;
//     this.weather = status.id;
//     this.weatherState = this.battle.initEffectState({ id: status.id });
//     if (source) {
//         this.weatherState.source = source;
//         this.weatherState.sourceSlot = source.getSlot();
//     }
//     if (status.duration) {
//         this.weatherState.duration = status.duration;
//     }
//     if (status.durationCallback) {
//         if (!source) throw new Error(`setting weather without a source`);
//         this.weatherState.duration = status.durationCallback.call(this.battle, source, source, sourceEffect);
//     }
//     if (!this.battle.singleEvent('FieldStart', status, this.weatherState, this, source, sourceEffect)) {
//         this.weather = prevWeather;
//         this.weatherState = prevWeatherState;
//         return false;
//     }
//     this.battle.eachEvent('WeatherChange', sourceEffect);
//     return true;
// }

use crate::Battle;
use crate::event::EventResult;
use crate::dex_data::ID;
use crate::battle::Effect;

impl Battle {
    /// Set the weather
    /// JavaScript: field.ts setWeather()
    ///
    /// Returns:
    /// - Some(true) if weather was set successfully
    /// - Some(false) if weather couldn't be set but was blocked by an event
    /// - None if weather is already active and nothing changed
    pub fn set_weather(
        &mut self,
        weather_id: ID,
        source_pos: Option<(usize, usize)>,
        source_effect: Option<Effect>,
    ) -> Option<bool> {
        // JavaScript: if (!sourceEffect && this.battle.effect) sourceEffect = this.battle.effect;
        // JavaScript: if (!source && this.battle.event?.target) source = this.battle.event.target;
        let source_effect = source_effect.or_else(|| self.effect.clone());
        let source_pos = source_pos.or_else(|| self.event.as_ref().and_then(|e| e.target));

        // Get current weather for comparison
        let current_weather = self.field.weather.clone();

        // If weather is already active, check if we should skip
        // JavaScript: if (this.weather === status.id) {...}
        if current_weather == weather_id {
            // JavaScript: if (sourceEffect && sourceEffect.effectType === 'Ability') {
            //     if (this.battle.gen > 5 || this.weatherState.duration === 0) {
            //         return false;
            //     }
            // } else if (this.battle.gen > 2 || status.id === 'sandstorm') {
            //     return false;
            // }
            if let Some(ref eff) = source_effect {
                let is_ability = matches!(eff.effect_type, crate::battle::EffectType::Ability);
                if is_ability {
                    // Ability weather - check gen or permanent duration
                    if self.gen > 5 || self.field.weather_state.duration == Some(0) {
                        return Some(false);
                    }
                } else if self.gen > 2 || weather_id.as_str() == "sandstorm" {
                    // Non-ability weather - return false for gen 3+ or sandstorm
                    return Some(false);
                }
            } else if self.gen > 2 || weather_id.as_str() == "sandstorm" {
                // No source effect - return false for gen 3+ or sandstorm
                return Some(false);
            }
        }

        // Run 'SetWeather' event - can block the weather change
        if source_pos.is_some() {
            let result = self.run_event(
                "SetWeather",
                crate::event::EventTarget::from_pokemon(source_pos),
                source_pos,
                Some(&Effect::weather(weather_id.clone())),
                crate::event::EventResult::Number(1),
                false,
                false,
            ).is_truthy();

            // If event returned false, weather change is blocked
            if !result {
                // JavaScript: if (result === false) {
                //     if ((sourceEffect as Move)?.weather) {
                //         this.battle.add('-fail', source, sourceEffect, '[from] ' + this.weather);
                //     } else if (sourceEffect && sourceEffect.effectType === 'Ability') {
                //         this.battle.add('-ability', source, sourceEffect, '[from] ' + this.weather, '[fail]');
                //     }
                // }

                if let Some(ref eff) = source_effect {
                    let is_ability = matches!(eff.effect_type, crate::battle::EffectType::Ability);
                    let is_move = matches!(eff.effect_type, crate::battle::EffectType::Move);

                    // Get source pokemon identifier for message
                    let source_ident = if let Some(pos) = source_pos {
                        if let Some(pokemon) = self.pokemon_at(pos.0, pos.1) {
                            pokemon.get_slot()
                        } else {
                            String::new()
                        }
                    } else {
                        String::new()
                    };

                    // Get source effect name
                    let source_effect_name = if let Some(move_data) = self.dex.moves().get_by_id(&eff.id) {
                        move_data.name.clone()
                    } else if let Some(ability_data) = self.dex.abilities().get_by_id(&eff.id) {
                        ability_data.name.clone()
                    } else {
                        eff.id.to_string()
                    };

                    // Check if sourceEffect is a move with weather property
                    let is_weather_move = if is_move {
                        if let Some(move_data) = self.dex.moves().get_by_id(&eff.id) {
                            move_data.weather.is_some()
                        } else {
                            false
                        }
                    } else {
                        false
                    };

                    let current_weather_name = if !current_weather.is_empty() {
                        if let Some(cond) = self.dex.conditions().get_by_id(&current_weather) {
                            cond.name.clone().unwrap_or_else(|| current_weather.to_string())
                        } else {
                            current_weather.to_string()
                        }
                    } else {
                        String::new()
                    };

                    if is_weather_move {
                        // Move with weather property: add('-fail', source, sourceEffect, '[from] ' + this.weather)
                        let from_str = format!("[from] {}", current_weather_name);
                        self.add("-fail", &[
                            source_ident.into(),
                            source_effect_name.into(),
                            from_str.into(),
                        ]);
                    } else if is_ability {
                        // Ability: add('-ability', source, sourceEffect, '[from] ' + this.weather, '[fail]')
                        let from_str = format!("[from] {}", current_weather_name);
                        self.add("-ability", &[
                            source_ident.into(),
                            source_effect_name.into(),
                            from_str.into(),
                            "[fail]".into(),
                        ]);
                    }
                }

                return None;
            }
        }

        // Save previous weather state in case we need to revert
        let prev_weather = self.field.weather.clone();
        let prev_weather_state = self.field.weather_state.clone();

        // Set new weather
        self.field.weather = weather_id.clone();
        self.field.weather_state = crate::event_system::EffectState::new(weather_id.clone());

        // Set source if provided
        if let Some(source_position) = source_pos {
            self.field.weather_state.source = Some(source_position);
        }

        // Set duration from condition data
        // JavaScript: if (status.duration) this.weatherState.duration = status.duration;
        // JavaScript: if (status.durationCallback) {
        //     if (!source) throw new Error(`setting weather without a source`);
        //     this.weatherState.duration = status.durationCallback.call(this.battle, source, source, sourceEffect);
        // }
        let weather_duration = {
            if let Some(condition) = self.dex.conditions().get_by_id(&weather_id) {
                debug_elog!("[SET_WEATHER] Found condition for '{}', duration={:?}", weather_id.as_str(), condition.duration);
                condition.duration
            } else {
                debug_elog!("[SET_WEATHER] No condition found for '{}', defaulting to Some(5)", weather_id.as_str());
                Some(5) // Default to 5 turns if condition not found
            }
        };

        debug_elog!("[SET_WEATHER] Setting weather_state.duration to {:?}", weather_duration);
        self.field.weather_state.duration = weather_duration;

        // Call durationCallback if it exists
        // Note: The callback name is "durationCallback" (lowercase d) in conditions.json
        debug_elog!("[SET_WEATHER] Checking if has_callback for durationCallback");
        if self.has_weather_callback(&weather_id, "durationCallback") {
            debug_elog!("[SET_WEATHER] Calling duration callback for '{}'", weather_id.as_str());
            let result = self.call_duration_callback(
                &weather_id,
                source_pos,
                source_pos,
                source_effect.as_ref(),
            );

            debug_elog!("[SET_WEATHER] Duration callback returned: {:?}", result);
            if let EventResult::Number(duration) = result {
                debug_elog!("[SET_WEATHER] Setting duration from callback: {}", duration);
                self.field.weather_state.duration = Some(duration);
            }
        } else {
            debug_elog!("[SET_WEATHER] No duration callback found for '{}'", weather_id.as_str());
        }

        // Fire 'FieldStart' event on the weather
        // If it returns false, revert to previous weather
        let field_start_result = self.single_event(
            "FieldStart",
            &crate::battle::Effect::weather(weather_id.clone()),
            None,
            None, // field as target - we don't pass this in current arch
            source_pos,
            None,
            None,
        );

        // Check if the event returned false (using the EventResult enum)
        if matches!(field_start_result, crate::event::EventResult::Boolean(false)) {
            // Revert to previous weather
            self.field.weather = prev_weather;
            self.field.weather_state = prev_weather_state;
            return Some(false);
        }

        // Fire 'WeatherChange' event
        self.each_event("WeatherChange", None, None);

        Some(true)
    }
}
