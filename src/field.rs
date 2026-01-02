//! Simulator Field
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Represents the battle field - weather, terrain, and pseudo-weather conditions.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::dex_data::{EffectState, ID};

/// The battle field - contains weather, terrain, and field-wide effects
#[derive(Debug, Clone, Serialize, Deserialize)]
/// JavaScript equivalent: Field (sim/field.ts)
/// 14 fields in JavaScript
/// JavaScript equivalent: Field (sim/global-types.ts)
pub struct Field {
    /// Current weather ID (empty string = no weather)
    pub weather: ID,
    /// Weather effect state
    pub weather_state: EffectState,

    /// Current terrain ID (empty string = no terrain)
    pub terrain: ID,
    /// Terrain effect state
    pub terrain_state: EffectState,

    /// Pseudo-weather conditions (like Trick Room, Magic Room, etc.)
    pub pseudo_weather: HashMap<ID, EffectState>,
}

impl Default for Field {
    fn default() -> Self {
        Self::new()
    }
}

impl Field {
    /// Create a new empty field
    pub fn new() -> Self {
        Self {
            weather: ID::empty(),
            weather_state: EffectState::new(ID::empty()),
            terrain: ID::empty(),
            terrain_state: EffectState::new(ID::empty()),
            pseudo_weather: HashMap::new(),
        }
    }

    /// Check if a weather is active
    pub fn has_weather(&self, weather_id: &ID) -> bool {
        &self.weather == weather_id && !weather_id.is_empty()
    }

    /// Check if any weather is active
    pub fn is_weather_active(&self) -> bool {
        !self.weather.is_empty()
    }

    /// Set the weather
    // TypeScript source:
    //
    //
    // 	setWeather(status: string | Condition, source: Pokemon | 'debug' | null = null, sourceEffect: Effect | null = null) {
    // 		status = this.battle.dex.conditions.get(status);
    // 		if (!sourceEffect && this.battle.effect) sourceEffect = this.battle.effect;
    // 		if (!source && this.battle.event?.target) source = this.battle.event.target;
    // 		if (source === 'debug') source = this.battle.sides[0].active[0];
    //
    // 		if (this.weather === status.id) {
    // 			if (sourceEffect && sourceEffect.effectType === 'Ability') {
    // 				if (this.battle.gen > 5 || this.weatherState.duration === 0) {
    // 					return false;
    // 				}
    // 			} else if (this.battle.gen > 2 || status.id === 'sandstorm') {
    // 				return false;
    // 			}
    // 		}
    // 		if (source) {
    // 			const result = this.battle.runEvent('SetWeather', source, source, status);
    // 			if (!result) {
    // 				if (result === false) {
    // 					if ((sourceEffect as Move)?.weather) {
    // 						this.battle.add('-fail', source, sourceEffect, '[from] ' + this.weather);
    // 					} else if (sourceEffect && sourceEffect.effectType === 'Ability') {
    // 						this.battle.add('-ability', source, sourceEffect, '[from] ' + this.weather, '[fail]');
    // 					}
    // 				}
    // 				return null;
    // 			}
    // 		}
    // 		const prevWeather = this.weather;
    // 		const prevWeatherState = this.weatherState;
    // 		this.weather = status.id;
    // 		this.weatherState = this.battle.initEffectState({ id: status.id });
    // 		if (source) {
    // 			this.weatherState.source = source;
    // 			this.weatherState.sourceSlot = source.getSlot();
    // 		}
    // 		if (status.duration) {
    // 			this.weatherState.duration = status.duration;
    // 		}
    // 		if (status.durationCallback) {
    // 			if (!source) throw new Error(`setting weather without a source`);
    // 			this.weatherState.duration = status.durationCallback.call(this.battle, source, source, sourceEffect);
    // 		}
    // 		if (!this.battle.singleEvent('FieldStart', status, this.weatherState, this, source, sourceEffect)) {
    // 			this.weather = prevWeather;
    // 			this.weatherState = prevWeatherState;
    // 			return false;
    // 		}
    // 		this.battle.eachEvent('WeatherChange', sourceEffect);
    // 		return true;
    // 	}
    //
    pub fn set_weather(&mut self, weather_id: ID, duration: Option<i32>) -> bool {
        if weather_id == self.weather {
            return false;
        }
        self.weather = weather_id.clone();
        self.weather_state = EffectState::new(weather_id);
        if let Some(d) = duration {
            self.weather_state.duration = Some(d);
        }
        true
    }

    /// Clear the weather
    //
    // 	clearWeather() {
    // 		if (!this.weather) return false;
    // 		const prevWeather = this.getWeather();
    // 		this.battle.singleEvent('FieldEnd', prevWeather, this.weatherState, this);
    // 		this.weather = '';
    // 		this.battle.clearEffectState(this.weatherState);
    // 		this.battle.eachEvent('WeatherChange');
    // 		return true;
    // 	}
    //
    pub fn clear_weather(&mut self) -> bool {
        if self.weather.is_empty() {
            return false;
        }
        self.weather = ID::empty();
        self.weather_state = EffectState::new(ID::empty());
        true
    }

    /// Get effective weather (accounting for abilities that suppress weather)
    /// For now, just returns the weather (suppression will be handled at battle level)
    //
    // 	effectiveWeather() {
    // 		if (this.suppressingWeather()) return '';
    // 		return this.weather;
    // 	}
    //
    pub fn effective_weather(&self) -> &ID {
        &self.weather
    }

    /// Check if weather matches
    //
    // 	isWeather(weather: string | string[]) {
    // 		const ourWeather = this.effectiveWeather();
    // 		if (!Array.isArray(weather)) {
    // 			return ourWeather === toID(weather);
    // 		}
    // 		return weather.map(toID).includes(ourWeather);
    // 	}
    //
    pub fn is_weather(&self, weather: &str) -> bool {
        self.weather.as_str() == weather.to_lowercase()
    }

    /// Check if any of the given weathers are active
    pub fn is_weather_any(&self, weathers: &[&str]) -> bool {
        weathers.iter().any(|w| self.is_weather(w))
    }

    /// Check if a terrain is active
    pub fn has_terrain(&self, terrain_id: &ID) -> bool {
        &self.terrain == terrain_id && !terrain_id.is_empty()
    }

    /// Check if any terrain is active
    pub fn is_terrain_active(&self) -> bool {
        !self.terrain.is_empty()
    }

    /// Set the terrain
    //
    // 	setTerrain(status: string | Effect, source: Pokemon | 'debug' | null = null, sourceEffect: Effect | null = null) {
    // 		status = this.battle.dex.conditions.get(status);
    // 		if (!sourceEffect && this.battle.effect) sourceEffect = this.battle.effect;
    // 		if (!source && this.battle.event?.target) source = this.battle.event.target;
    // 		if (source === 'debug') source = this.battle.sides[0].active[0];
    // 		if (!source) throw new Error(`setting terrain without a source`);
    //
    // 		if (this.terrain === status.id) return false;
    // 		const prevTerrain = this.terrain;
    // 		const prevTerrainState = this.terrainState;
    // 		this.terrain = status.id;
    // 		this.terrainState = this.battle.initEffectState({
    // 			id: status.id,
    // 			source,
    // 			sourceSlot: source.getSlot(),
    // 			duration: status.duration,
    // 		});
    // 		if (status.durationCallback) {
    // 			this.terrainState.duration = status.durationCallback.call(this.battle, source, source, sourceEffect);
    // 		}
    // 		if (!this.battle.singleEvent('FieldStart', status, this.terrainState, this, source, sourceEffect)) {
    // 			this.terrain = prevTerrain;
    // 			this.terrainState = prevTerrainState;
    // 			return false;
    // 		}
    // 		this.battle.eachEvent('TerrainChange', sourceEffect);
    // 		return true;
    // 	}
    //
    pub fn set_terrain(&mut self, terrain_id: ID, duration: Option<i32>) -> bool {
        if terrain_id == self.terrain {
            return false;
        }
        self.terrain = terrain_id.clone();
        self.terrain_state = EffectState::new(terrain_id);
        if let Some(d) = duration {
            self.terrain_state.duration = Some(d);
        }
        true
    }

    /// Clear the terrain
    //
    // 	clearTerrain() {
    // 		if (!this.terrain) return false;
    // 		const prevTerrain = this.getTerrain();
    // 		this.battle.singleEvent('FieldEnd', prevTerrain, this.terrainState, this);
    // 		this.terrain = '';
    // 		this.battle.clearEffectState(this.terrainState);
    // 		this.battle.eachEvent('TerrainChange');
    // 		return true;
    // 	}
    //
    pub fn clear_terrain(&mut self) -> bool {
        if self.terrain.is_empty() {
            return false;
        }
        self.terrain = ID::empty();
        self.terrain_state = EffectState::new(ID::empty());
        true
    }

    /// Check if terrain matches
    //
    // 	isTerrain(terrain: string | string[], target?: Pokemon | Side | Battle) {
    // 		const ourTerrain = this.effectiveTerrain(target);
    // 		if (!Array.isArray(terrain)) {
    // 			return ourTerrain === toID(terrain);
    // 		}
    // 		return terrain.map(toID).includes(ourTerrain);
    // 	}
    //
    pub fn is_terrain(&self, terrain: &str) -> bool {
        self.terrain.as_str() == terrain.to_lowercase()
    }

    /// Add a pseudo-weather condition
    //
    // 	addPseudoWeather(
    // 		status: string | Condition,
    // 		source: Pokemon | 'debug' | null = null,
    // 		sourceEffect: Effect | null = null
    // 	): boolean {
    // 		if (!source && this.battle.event?.target) source = this.battle.event.target;
    // 		if (source === 'debug') source = this.battle.sides[0].active[0];
    // 		status = this.battle.dex.conditions.get(status);
    //
    // 		let state = this.pseudoWeather[status.id];
    // 		if (state) {
    // 			if (!(status as any).onFieldRestart) return false;
    // 			return this.battle.singleEvent('FieldRestart', status, state, this, source, sourceEffect);
    // 		}
    // 		state = this.pseudoWeather[status.id] = this.battle.initEffectState({
    // 			id: status.id,
    // 			source,
    // 			sourceSlot: source?.getSlot(),
    // 			duration: status.duration,
    // 		});
    // 		if (status.durationCallback) {
    // 			if (!source) throw new Error(`setting fieldcond without a source`);
    // 			state.duration = status.durationCallback.call(this.battle, source, source, sourceEffect);
    // 		}
    // 		if (!this.battle.singleEvent('FieldStart', status, state, this, source, sourceEffect)) {
    // 			delete this.pseudoWeather[status.id];
    // 			return false;
    // 		}
    // 		this.battle.runEvent('PseudoWeatherChange', source, source, status);
    // 		return true;
    // 	}
    //
    pub fn add_pseudo_weather(&mut self, id: ID, duration: Option<i32>) -> bool {
        if self.pseudo_weather.contains_key(&id) {
            return false;
        }
        let mut state = EffectState::new(id.clone());
        state.duration = duration;
        self.pseudo_weather.insert(id, state);
        true
    }

    /// Check if a pseudo-weather is active
    pub fn has_pseudo_weather(&self, id: &ID) -> bool {
        self.pseudo_weather.contains_key(id)
    }

    /// Get pseudo-weather state
    //
    // 	getPseudoWeather(status: string | Effect) {
    // 		status = this.battle.dex.conditions.get(status);
    // 		return this.pseudoWeather[status.id] ? status : null;
    // 	}
    //
    pub fn get_pseudo_weather(&self, id: &ID) -> Option<&EffectState> {
        self.pseudo_weather.get(id)
    }

    /// Get mutable pseudo-weather state
    pub fn get_pseudo_weather_mut(&mut self, id: &ID) -> Option<&mut EffectState> {
        self.pseudo_weather.get_mut(id)
    }

    /// Remove a pseudo-weather
    //
    // 	removePseudoWeather(status: string | Effect) {
    // 		status = this.battle.dex.conditions.get(status);
    // 		const state = this.pseudoWeather[status.id];
    // 		if (!state) return false;
    // 		this.battle.singleEvent('FieldEnd', status, state, this);
    // 		delete this.pseudoWeather[status.id];
    // 		return true;
    // 	}
    //
    pub fn remove_pseudo_weather(&mut self, id: &ID) -> bool {
        self.pseudo_weather.remove(id).is_some()
    }

    // ==========================================
    // Methods ported from field.ts
    // ==========================================

    /// Get weather state
    //
    // 	getWeather() {
    // 		return this.battle.dex.conditions.getByID(this.weather);
    // 	}
    //
    pub fn get_weather(&self) -> &ID {
        &self.weather
    }

    /// Get weather state struct
    pub fn get_weather_state(&self) -> &EffectState {
        &self.weather_state
    }

    /// Get mutable weather state
    pub fn get_weather_state_mut(&mut self) -> &mut EffectState {
        &mut self.weather_state
    }

    /// Get terrain
    //
    // 	getTerrain() {
    // 		return this.battle.dex.conditions.getByID(this.terrain);
    // 	}
    //
    pub fn get_terrain(&self) -> &ID {
        &self.terrain
    }

    /// Get terrain state struct
    pub fn get_terrain_state(&self) -> &EffectState {
        &self.terrain_state
    }

    /// Get mutable terrain state
    pub fn get_terrain_state_mut(&mut self) -> &mut EffectState {
        &mut self.terrain_state
    }

    /// Effective terrain (accounts for Pokemon abilities)
    /// For now, just returns the terrain (suppression at battle level)
    //
    // 	effectiveTerrain(target?: Pokemon | Side | Battle) {
    // 		if (this.battle.event && !target) target = this.battle.event.target;
    // 		return this.battle.runEvent('TryTerrain', target) ? this.terrain : '';
    // 	}
    //
    pub fn effective_terrain(&self) -> &ID {
        &self.terrain
    }

    /// Check if terrain matches any in list
    pub fn is_terrain_any(&self, terrains: &[&str]) -> bool {
        terrains.iter().any(|t| self.is_terrain(t))
    }

    /// Check if suppressing weather (would need battle context)
    //
    // 	suppressingWeather() {
    // 		for (const side of this.battle.sides) {
    // 			for (const pokemon of side.active) {
    // 				if (pokemon && !pokemon.fainted && !pokemon.ignoringAbility() &&
    // 					pokemon.getAbility().suppressWeather && !pokemon.abilityState.ending) {
    // 					return true;
    // 				}
    // 			}
    // 		}
    // 		return false;
    // 	}
    //
    pub fn suppressing_weather(&self) -> bool {
        // Would need to check active Pokemon abilities
        // Stub implementation - battle handles this
        false
    }

    /// Destroy field (cleanup)
    //
    // 	destroy() {
    // 		// deallocate ourself
    //
    // 		// get rid of some possibly-circular references
    // 		(this as any).battle = null!;
    // 	}
    //
    pub fn destroy(&mut self) {
        self.weather = ID::empty();
        self.weather_state = EffectState::new(ID::empty());
        self.terrain = ID::empty();
        self.terrain_state = EffectState::new(ID::empty());
        self.pseudo_weather.clear();
    }

    /// Convert field to JSON
    /// Equivalent to field.ts toJSON()
    //
    // 	toJSON(): AnyObject {
    // 		return State.serializeField(this);
    // 	}
    //
    pub fn to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "weather": if self.weather.is_empty() { serde_json::Value::Null } else { serde_json::Value::String(self.weather.as_str().to_string()) },
            "weatherDuration": self.weather_state.duration,
            "terrain": if self.terrain.is_empty() { serde_json::Value::Null } else { serde_json::Value::String(self.terrain.as_str().to_string()) },
            "terrainDuration": self.terrain_state.duration,
            "pseudoWeather": self.pseudo_weather.keys().map(|k| k.as_str().to_string()).collect::<Vec<_>>()
        })
    }

    /// Decrement durations and remove expired conditions
    /// Returns list of expired effect IDs
    pub fn decrement_durations(&mut self) -> Vec<ID> {
        let mut expired = Vec::new();

        // Weather
        if let Some(ref mut duration) = self.weather_state.duration {
            if *duration > 0 {
                *duration -= 1;
                if *duration == 0 {
                    expired.push(self.weather.clone());
                    self.clear_weather();
                }
            }
        }

        // Terrain
        if let Some(ref mut duration) = self.terrain_state.duration {
            if *duration > 0 {
                *duration -= 1;
                if *duration == 0 {
                    expired.push(self.terrain.clone());
                    self.clear_terrain();
                }
            }
        }

        // Pseudo-weather
        let mut to_remove = Vec::new();
        for (id, state) in &mut self.pseudo_weather {
            if let Some(ref mut duration) = state.duration {
                if *duration > 0 {
                    *duration -= 1;
                    if *duration == 0 {
                        to_remove.push(id.clone());
                    }
                }
            }
        }
        for id in to_remove {
            expired.push(id.clone());
            self.pseudo_weather.remove(&id);
        }

        expired
    }
}

/// Weather damage calculation helper
pub fn get_weather_damage_fraction(weather: &str, pokemon_types: &[String]) -> f64 {
    let has_type = |t: &str| {
        pokemon_types
            .iter()
            .any(|pt| pt.to_lowercase() == t.to_lowercase())
    };

    match weather {
        // Sandstorm damages non-Rock/Ground/Steel types
        "sandstorm" => {
            if has_type("rock") || has_type("ground") || has_type("steel") {
                0.0
            } else {
                1.0 / 16.0
            }
        }
        // Hail/Snow damages non-Ice types (Gen 1-8), Gen 9 Snow doesn't damage
        "hail" => {
            if has_type("ice") {
                0.0
            } else {
                1.0 / 16.0
            }
        }
        _ => 0.0,
    }
}

/// Weather type boost (1.5x for weather-boosted types, 0.5x for weakened)
pub fn get_weather_type_modifier(weather: &str, move_type: &str) -> f64 {
    match (weather, move_type.to_lowercase().as_str()) {
        // Rain boosts Water, weakens Fire
        ("raindance" | "rain" | "primordialsea", "water") => 1.5,
        ("raindance" | "rain" | "primordialsea", "fire") => 0.5,

        // Sun boosts Fire, weakens Water
        ("sunnyday" | "sun" | "desolateland", "fire") => 1.5,
        ("sunnyday" | "sun" | "desolateland", "water") => 0.5,

        _ => 1.0,
    }
}

/// Get terrain damage modifier for grounded Pokemon
pub fn get_terrain_damage_modifier(terrain: &str, move_type: &str, is_grounded: bool) -> f64 {
    if !is_grounded {
        return 1.0;
    }

    match (terrain, move_type.to_lowercase().as_str()) {
        // Electric Terrain: 1.3x Electric moves
        ("electricterrain", "electric") => 1.3,
        // Grassy Terrain: 1.3x Grass moves
        ("grassyterrain", "grass") => 1.3,
        // Psychic Terrain: 1.3x Psychic moves
        ("psychicterrain", "psychic") => 1.3,
        _ => 1.0,
    }
}

/// Check if terrain prevents priority moves
pub fn terrain_blocks_priority(terrain: &str, target_is_grounded: bool) -> bool {
    // Psychic Terrain blocks priority moves targeting grounded Pokemon
    terrain == "psychicterrain" && target_is_grounded
}

/// Check if terrain prevents status
pub fn terrain_prevents_status(terrain: &str, status: &str, is_grounded: bool) -> bool {
    if !is_grounded {
        return false;
    }

    match (terrain, status) {
        // Electric Terrain prevents Sleep
        ("electricterrain", "slp") => true,
        // Misty Terrain prevents all non-volatile status
        ("mistyterrain", "brn" | "par" | "psn" | "tox" | "slp" | "frz") => true,
        _ => false,
    }
}

/// Grassy Terrain end-of-turn healing
pub fn get_grassy_terrain_heal(terrain: &str, is_grounded: bool) -> f64 {
    if terrain == "grassyterrain" && is_grounded {
        1.0 / 16.0
    } else {
        0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field_weather() {
        let mut field = Field::new();
        assert!(!field.is_weather_active());

        field.set_weather(ID::new("sunnyday"), Some(5));
        assert!(field.is_weather_active());
        assert!(field.is_weather("sunnyday"));
        assert!(!field.is_weather("raindance"));

        field.clear_weather();
        assert!(!field.is_weather_active());
    }

    #[test]
    fn test_field_terrain() {
        let mut field = Field::new();
        assert!(!field.is_terrain_active());

        field.set_terrain(ID::new("electricterrain"), Some(5));
        assert!(field.is_terrain_active());
        assert!(field.is_terrain("electricterrain"));

        field.clear_terrain();
        assert!(!field.is_terrain_active());
    }

    #[test]
    fn test_field_pseudo_weather() {
        let mut field = Field::new();
        let trick_room = ID::new("trickroom");

        assert!(!field.has_pseudo_weather(&trick_room));

        field.add_pseudo_weather(trick_room.clone(), Some(5));
        assert!(field.has_pseudo_weather(&trick_room));

        field.remove_pseudo_weather(&trick_room);
        assert!(!field.has_pseudo_weather(&trick_room));
    }

    #[test]
    fn test_duration_decrement() {
        let mut field = Field::new();
        field.set_weather(ID::new("raindance"), Some(2));

        // First decrement: duration goes from 2 to 1
        let expired = field.decrement_durations();
        assert!(expired.is_empty());
        assert!(field.is_weather_active());

        // Second decrement: duration goes from 1 to 0, weather expires
        let expired = field.decrement_durations();
        assert_eq!(expired.len(), 1);
        assert_eq!(expired[0].as_str(), "raindance");
        assert!(!field.is_weather_active());
    }
}
