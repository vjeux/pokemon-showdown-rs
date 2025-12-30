use crate::*;

impl Battle {

    /// Find field event handlers
    /// Equivalent to battle.ts findFieldEventHandlers()
    //
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
    //
    pub fn find_field_event_handlers(&self, event_id: &str) -> Vec<(ID, Option<(usize, usize)>)> {
        let mut handlers = Vec::new();
        let _ = event_id;

        // Add weather handler
        if !self.field.weather.is_empty() {
            handlers.push((self.field.weather.clone(), None));
        }

        // Add terrain handler
        if !self.field.terrain.is_empty() {
            handlers.push((self.field.terrain.clone(), None));
        }

        // Add pseudo-weather handlers
        for pw_id in self.field.pseudo_weather.keys() {
            handlers.push((pw_id.clone(), None));
        }

        handlers
    }
}
