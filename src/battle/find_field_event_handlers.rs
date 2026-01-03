// 1:1 port of findFieldEventHandlers from battle.ts

use crate::*;
use crate::battle::{EventListener, EffectType};
use crate::event_system::EffectState;

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

        eprintln!("[DEBUG find_field_event_handlers] callback={}, get_key={:?}", callback_name, get_key);
        eprintln!("[DEBUG find_field_event_handlers] weather={}, duration={:?}",
            self.field.weather, self.field.weather_state.duration);

        // Collect pseudo weather IDs that have callbacks or getKey
        // (need to extract before calling resolve_priority due to borrow checker)
        let pseudo_weather_handlers: Vec<(ID, EffectState)> = self.field.pseudo_weather.iter()
            .filter(|(pw_id, pw_state)| {
                let has_callback = self.has_callback(pw_id, callback_name);
                let has_get_key = get_key.is_some_and(|key| {
                    // JavaScript checks pseudoWeatherState[getKey], which means checking if duration exists
                    key == "duration" && pw_state.duration.is_some()
                });
                has_callback || has_get_key
            })
            .map(|(pw_id, pw_state)| (pw_id.clone(), pw_state.clone()))
            .collect();

        // JS: for (const id in field.pseudoWeather) {
        for (pw_id, pw_state) in pseudo_weather_handlers {
            // JS: handlers.push(this.resolvePriority({...}, callbackName));
            let mut handler = EventListener {
                effect_id: pw_id,
                effect_type: EffectType::Condition,
                target: None,
                index: None,
                state: Some(pw_state),
                effect_holder: custom_holder, // JS: customHolder || field (field not representable as tuple)
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
            eprintln!("[DEBUG] Weather='{}', get_key={:?}, duration={:?}",
                self.field.weather, get_key, self.field.weather_state.duration);
            let has_callback = self.has_callback(&self.field.weather, callback_name);
            let has_get_key = if let Some(key) = get_key {
                // JavaScript checks weatherState[getKey], which in Rust means checking if the duration field exists
                let result = key == "duration" && self.field.weather_state.duration.is_some();
                eprintln!("[DEBUG] Weather handler check: key={}, has_duration={:?}, result={}",
                    key, self.field.weather_state.duration, result);
                result
            } else {
                false
            };
            eprintln!("[DEBUG] Weather handler: has_callback={}, has_get_key={}", has_callback, has_get_key);
            if has_callback || has_get_key {
                eprintln!("[DEBUG] Creating weather handler for '{}'", self.field.weather);
                Some((self.field.weather.clone(), self.field.weather_state.clone()))
            } else {
                eprintln!("[DEBUG] NOT creating weather handler");
                None
            }
        } else {
            eprintln!("[DEBUG] Weather is empty, skipping");
            None
        };

        if let Some((weather_id, weather_state)) = weather_handler {
            // JS: handlers.push(this.resolvePriority({...}, callbackName));
            eprintln!("[DEBUG] Adding weather handler to handlers vector");
            let mut handler = EventListener {
                effect_id: weather_id,
                effect_type: EffectType::Weather,
                target: None,
                index: None,
                state: Some(weather_state),
                effect_holder: custom_holder, // JS: customHolder || field (field not representable as tuple)
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
            let has_callback = self.has_callback(&self.field.terrain, callback_name);
            let has_get_key = get_key.is_some_and(|key| {
                // JavaScript checks terrainState[getKey], which means checking if duration exists
                key == "duration" && self.field.terrain_state.duration.is_some()
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
            // JS: handlers.push(this.resolvePriority({...}, callbackName));
            let mut handler = EventListener {
                effect_id: terrain_id,
                effect_type: EffectType::Terrain,
                target: None,
                index: None,
                state: Some(terrain_state),
                effect_holder: custom_holder, // JS: customHolder || field (field not representable as tuple)
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
        eprintln!("[DEBUG] find_field_event_handlers returning {} handlers", handlers.len());
        handlers
    }
}
