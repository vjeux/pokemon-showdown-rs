// 1:1 port of findFieldEventHandlers from battle.ts

use crate::*;
use crate::battle::{EventListener, EffectType};

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
        &self,
        callback_name: &str,
        get_key: Option<&str>,
        custom_holder: Option<(usize, usize)>,
    ) -> Vec<EventListener> {
        // JS: const handlers: EventListener[] = [];
        let mut handlers: Vec<EventListener> = Vec::new();

        // JS: let callback;
        // JS: for (const id in field.pseudoWeather) {
        for (pw_id, pw_state) in &self.field.pseudo_weather {
            // JS: const pseudoWeatherState = field.pseudoWeather[id];
            // JS: const pseudoWeather = this.dex.conditions.getByID(id as ID);
            // JS: callback = this.getCallback(field, pseudoWeather, callbackName);
            let has_callback = self.has_callback(pw_id, callback_name);

            // JS: if (callback !== undefined || (getKey && pseudoWeatherState[getKey])) {
            let has_get_key = get_key.is_some_and(|key| {
                pw_state.data.get(key).is_some()
            });

            if has_callback || has_get_key {
                // JS: handlers.push(this.resolvePriority({...}, callbackName));
                // TODO: Should call resolve_priority to fill in order/priority/sub_order
                // TODO: State type mismatch - field states are dex_data::EffectState but EventListener expects event_system::EffectState
                handlers.push(EventListener {
                    effect_id: pw_id.clone(),
                    effect_type: EffectType::Condition,
                    target: None,
                    index: None,
                    state: None, // TODO: Convert dex_data::EffectState to event_system::EffectState
                    effect_holder: custom_holder, // JS: customHolder || field (field not representable as tuple)
                    order: None,
                    priority: 0,
                    sub_order: 0,
                    effect_order: None,
                    speed: None,
                });
            }
        }

        // JS: const weather = field.getWeather();
        // JS: callback = this.getCallback(field, weather, callbackName);
        // JS: if (callback !== undefined || (getKey && this.field.weatherState[getKey])) {
        if !self.field.weather.is_empty() {
            let has_callback = self.has_callback(&self.field.weather, callback_name);
            let has_get_key = get_key.is_some_and(|key| {
                self.field.weather_state.data.get(key).is_some()
            });

            if has_callback || has_get_key {
                // JS: handlers.push(this.resolvePriority({...}, callbackName));
                // TODO: State type mismatch - field states are dex_data::EffectState but EventListener expects event_system::EffectState
                handlers.push(EventListener {
                    effect_id: self.field.weather.clone(),
                    effect_type: EffectType::Weather,
                    target: None,
                    index: None,
                    state: None, // TODO: Convert dex_data::EffectState to event_system::EffectState
                    effect_holder: custom_holder, // JS: customHolder || field (field not representable as tuple)
                    order: None,
                    priority: 0,
                    sub_order: 0,
                    effect_order: None,
                    speed: None,
                });
            }
        }

        // JS: const terrain = field.getTerrain();
        // JS: callback = this.getCallback(field, terrain, callbackName);
        // JS: if (callback !== undefined || (getKey && field.terrainState[getKey])) {
        if !self.field.terrain.is_empty() {
            let has_callback = self.has_callback(&self.field.terrain, callback_name);
            let has_get_key = get_key.is_some_and(|key| {
                self.field.terrain_state.data.get(key).is_some()
            });

            if has_callback || has_get_key {
                // JS: handlers.push(this.resolvePriority({...}, callbackName));
                // TODO: State type mismatch - field states are dex_data::EffectState but EventListener expects event_system::EffectState
                handlers.push(EventListener {
                    effect_id: self.field.terrain.clone(),
                    effect_type: EffectType::Terrain,
                    target: None,
                    index: None,
                    state: None, // TODO: Convert dex_data::EffectState to event_system::EffectState
                    effect_holder: custom_holder, // JS: customHolder || field (field not representable as tuple)
                    order: None,
                    priority: 0,
                    sub_order: 0,
                    effect_order: None,
                    speed: None,
                });
            }
        }

        // JS: return handlers;
        handlers
    }
}
