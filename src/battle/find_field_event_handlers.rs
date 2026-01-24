// 1:1 port of findFieldEventHandlers from battle.ts

use crate::*;
use crate::battle::{Effect, EffectHolder, EventListener, EffectType};
use crate::event_system::SharedEffectState;

impl Battle {
    /// Find field event handlers
    /// Equivalent to battle.ts findFieldEventHandlers()
    ///
    // JS Source:
    // 	findFieldEventHandlers(field: Field, callbackName: string, getKey?: 'duration', customHolder?: Pokemon) {
    // 		const handlers: EventListener[] = [];
    //
    // 		let callback;
    // 		for (const id in field.pseudoWeather) {
    // 			const pseudoWeatherState = field.pseudoWeather[id];
    // 			const pseudoWeather = this.dex.conditions.getByID(id as ID);
    // 			callback = this.getCallback(field, pseudoWeather, callbackName);
    // 			if (callback !== undefined || (getKey && pseudoWeatherState[getKey])) {
    // 				handlers.push(this.resolvePriority({
    // 					effect: pseudoWeather, callback, state: pseudoWeatherState,
    // 					end: customHolder ? null : field.removePseudoWeather, effectHolder: customHolder || field,
    // 				}, callbackName));
    // 			}
    // 		}
    // 		const weather = field.getWeather();
    // 		callback = this.getCallback(field, weather, callbackName);
    // 		if (callback !== undefined || (getKey && this.field.weatherState[getKey])) {
    // 			handlers.push(this.resolvePriority({
    // 				effect: weather, callback, state: this.field.weatherState,
    // 				end: customHolder ? null : field.clearWeather, effectHolder: customHolder || field,
    // 			}, callbackName));
    // 		}
    // 		const terrain = field.getTerrain();
    // 		callback = this.getCallback(field, terrain, callbackName);
    // 		if (callback !== undefined || (getKey && field.terrainState[getKey])) {
    // 			handlers.push(this.resolvePriority({
    // 				effect: terrain, callback, state: field.terrainState,
    // 				end: customHolder ? null : field.clearTerrain, effectHolder: customHolder || field,
    // 			}, callbackName));
    // 		}
    //
    // 		return handlers;
    // 	}
    pub fn find_field_event_handlers(
        &mut self,
        callback_name: &str,
        get_key: Option<&str>,
        custom_holder: Option<(usize, usize)>,
    ) -> Vec<EventListener> {
        // JS: const handlers: EventListener[] = [];
        let mut handlers: Vec<EventListener> = Vec::new();

        // Collect pseudo weather IDs that have callbacks or getKey
        // (need to extract before calling resolve_priority due to borrow checker)
        let pseudo_weather_handlers: Vec<(ID, SharedEffectState)> = self.field.pseudo_weather.iter()
            .filter(|(pw_id, pw_state)| {
                let has_callback = self.has_pseudo_weather_callback(pw_id, callback_name);
                let has_get_key = get_key.is_some_and(|key| {
                    // JavaScript checks pseudoWeatherState[getKey], which means checking if duration exists
                    key == "duration" && pw_state.borrow().duration.is_some()
                });
                has_callback || has_get_key
            })
            .map(|(pw_id, pw_state)| (pw_id.clone(), pw_state.clone()))
            .collect();

        // JS: for (const id in field.pseudoWeather) {
        for (pw_id, pw_state) in pseudo_weather_handlers {
            // Get pseudo weather name from dex
            let pw_name = self.dex.conditions().get_by_id(&pw_id)
                .and_then(|c| c.name.clone())
                .unwrap_or_else(|| pw_id.to_string());

            // JS: handlers.push(this.resolvePriority({...}, callbackName));
            // JavaScript does NOT set handler.effectOrder in findFieldEventHandlers
            // Only resolve_priority sets it for SwitchIn/RedirectTarget events
            //
            // IMPORTANT: In JavaScript, resolvePriority determines subOrder based on state.target:
            // - If state.target instanceof Field -> subOrder = 5 (field condition)
            // - Otherwise -> subOrder = 2 (generic condition)
            //
            // state.target gets set by runEvent (line 912: this.effectState.target = effectHolder)
            // when a callback for this effect runs. For pseudo-weathers like fairylock that have
            // callbacks running via runEvent (e.g., onTrapPokemon), state.target becomes Field.
            // For pseudo-weathers like echoedvoice that only have FieldStart/FieldRestart callbacks
            // (which run via singleEvent, not runEvent), state.target stays undefined.
            //
            // In Rust, we track this via EffectState.target_is_field flag, which gets set
            // during run_event callback execution when effectHolder is a field-level entity.
            // We use this to determine the correct effectType for subOrder calculation.
            let effect_type = if pw_state.borrow().target_is_field {
                EffectType::FieldCondition // subOrder = 5
            } else {
                EffectType::Condition // subOrder = 2
            };

            let mut handler = EventListener {
                callback_name: String::new(),
                effect: Effect {
                    id: pw_id,
                    name: pw_name,
                    effect_type,
                    effect_holder: custom_holder.map(|(s, p)| EffectHolder::Pokemon(s, p)).or(Some(EffectHolder::Field)),
                    side_index: None,
                    prankster_boosted: false,
                },
                target: None,
                index: None,
                state: Some(pw_state.clone()),
                effect_holder: custom_holder.map(|(s, p)| EffectHolder::Pokemon(s, p)).or(Some(EffectHolder::Field)), // JS: customHolder || field (field not representable as tuple)
                order: None,
                priority: 0,
                sub_order: 0,
                effect_order: None,
                speed: None,
            };

            // Call resolve_priority to fill in order/priority/sub_order
            self.resolve_priority(&mut handler, callback_name);

            handlers.push(handler);
        }

        // JS: const weather = field.getWeather();
        // JS: callback = this.getCallback(field, weather, callbackName);
        // JS: if (callback !== undefined || (getKey && this.field.weatherState[getKey])) {
        let weather_handler = if !self.field.weather.is_empty() {
            debug_elog!("[FIND_FIELD_EVENT T{}] Checking weather '{}' for callback '{}'", self.turn, self.field.weather.as_str(), callback_name);
            let has_callback = self.has_weather_callback(&self.field.weather, callback_name);
            debug_elog!("[FIND_FIELD_EVENT T{}] has_callback={}", self.turn, has_callback);
            let has_get_key = if let Some(key) = get_key {
                // JavaScript checks weatherState[getKey], which in Rust means checking if the duration field exists
                key == "duration" && self.field.weather_state.borrow().duration.is_some()
            } else {
                false
            };
            debug_elog!("[FIND_FIELD_EVENT T{}] has_get_key={}", self.turn, has_get_key);
            if has_callback || has_get_key {
                debug_elog!("[FIND_FIELD_EVENT T{}] Adding weather handler for '{}'", self.turn, self.field.weather.as_str());
                Some((self.field.weather.clone(), self.field.weather_state.clone()))
            } else {
                None
            }
        } else {
            None
        };

        if let Some((weather_id, weather_state)) = weather_handler {
            // Get weather name from dex
            let weather_name = self.dex.conditions().get_by_id(&weather_id)
                .and_then(|c| c.name.clone())
                .unwrap_or_else(|| weather_id.to_string());

            // JS: handlers.push(this.resolvePriority({...}, callbackName));
            // JavaScript does NOT set handler.effectOrder in findFieldEventHandlers
            // Only resolve_priority sets it for SwitchIn/RedirectTarget events
            let mut handler = EventListener {
                callback_name: String::new(),
                effect: Effect {
                    id: weather_id,
                    name: weather_name,
                    effect_type: EffectType::Weather,
                    effect_holder: custom_holder.map(|(s, p)| EffectHolder::Pokemon(s, p)).or(Some(EffectHolder::Field)),
                    side_index: None,
                    prankster_boosted: false,
                },
                target: None,
                index: None,
                state: Some(weather_state.clone()),
                effect_holder: custom_holder.map(|(s, p)| EffectHolder::Pokemon(s, p)).or(Some(EffectHolder::Field)), // JS: customHolder || field (field not representable as tuple)
                order: None,
                priority: 0,
                sub_order: 0,
                effect_order: None,
                speed: None,
            };

            // Call resolve_priority to fill in order/priority/sub_order
            self.resolve_priority(&mut handler, callback_name);

            handlers.push(handler);
        }

        // JS: const terrain = field.getTerrain();
        // JS: callback = this.getCallback(field, terrain, callbackName);
        // JS: if (callback !== undefined || (getKey && field.terrainState[getKey])) {
        let terrain_handler = if !self.field.terrain.is_empty() {
            let has_callback = self.has_terrain_callback(&self.field.terrain, callback_name);
            let has_get_key = get_key.is_some_and(|key| {
                // JavaScript checks terrainState[getKey], which means checking if duration exists
                key == "duration" && self.field.terrain_state.borrow().duration.is_some()
            });
            if has_callback || has_get_key {
                Some((self.field.terrain.clone(), self.field.terrain_state.clone()))
            } else {
                None
            }
        } else {
            None
        };

        if let Some((terrain_id, terrain_state)) = terrain_handler {
            // Get terrain name from dex
            let terrain_name = self.dex.conditions().get_by_id(&terrain_id)
                .and_then(|c| c.name.clone())
                .unwrap_or_else(|| terrain_id.to_string());

            // JS: handlers.push(this.resolvePriority({...}, callbackName));
            // JavaScript does NOT set handler.effectOrder in findFieldEventHandlers
            // Only resolve_priority sets it for SwitchIn/RedirectTarget events
            let mut handler = EventListener {
                callback_name: String::new(),
                effect: Effect {
                    id: terrain_id,
                    name: terrain_name,
                    effect_type: EffectType::Terrain,
                    effect_holder: custom_holder.map(|(s, p)| EffectHolder::Pokemon(s, p)).or(Some(EffectHolder::Field)),
                    side_index: None,
                    prankster_boosted: false,
                },
                target: None,
                index: None,
                state: Some(terrain_state.clone()),
                effect_holder: custom_holder.map(|(s, p)| EffectHolder::Pokemon(s, p)).or(Some(EffectHolder::Field)), // JS: customHolder || field (field not representable as tuple)
                order: None,
                priority: 0,
                sub_order: 0,
                effect_order: None,
                speed: None,
            };

            // Call resolve_priority to fill in order/priority/sub_order
            self.resolve_priority(&mut handler, callback_name);

            handlers.push(handler);
        }

        // JS: return handlers;
        handlers
    }
}
