//! Simulator Field
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Represents the battle field - weather, terrain, and pseudo-weather conditions.

use serde::{Deserialize, Serialize};
use indexmap::IndexMap;

use crate::dex_data::ID;
use crate::event_system::{EffectState, SharedEffectState};

/// The battle field - contains weather, terrain, and field-wide effects
#[derive(Debug, Clone, Serialize, Deserialize)]
/// JavaScript equivalent: Field (sim/field.ts)
/// 7 fields in JavaScript
pub struct Field {
    /// Field ID (readonly)
    /// JavaScript: readonly id: ID
    pub id: ID,

    /// Current weather ID (empty string = no weather)
    /// JavaScript: weather: ID
    pub weather: ID,

    /// Weather effect state
    /// JavaScript: weatherState: EffectState
    pub weather_state: SharedEffectState,

    /// Current terrain ID (empty string = no terrain)
    /// JavaScript: terrain: ID
    pub terrain: ID,

    /// Terrain effect state
    /// JavaScript: terrainState: EffectState
    pub terrain_state: SharedEffectState,

    /// Pseudo-weather conditions (like Trick Room, Magic Room, etc.)
    /// JavaScript: pseudoWeather: { [id: string]: EffectState }
    /// Uses IndexMap to preserve insertion order like JavaScript objects
    pub pseudo_weather: IndexMap<ID, SharedEffectState>,
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
            id: ID::empty(),
            weather: ID::empty(),
            weather_state: SharedEffectState::new(EffectState::new(ID::empty())),
            terrain: ID::empty(),
            terrain_state: SharedEffectState::new(EffectState::new(ID::empty())),
            pseudo_weather: IndexMap::new(),
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

    /// Check if a pseudo-weather is active
    pub fn has_pseudo_weather(&self, pseudo_weather_id: &ID) -> bool {
        self.pseudo_weather.contains_key(pseudo_weather_id)
    }

    // ==========================================
    // Methods ported from field.ts
    // ==========================================

    /// Get weather state
    ///
    /// JavaScript source (field.ts getWeather):
    // 	getWeather() {
    // 		return this.battle.dex.conditions.getByID(this.weather);
    // 	}
    pub fn get_weather(&self) -> &ID {
        // return this.battle.dex.conditions.getByID(this.weather);
        // NOTE: JavaScript returns Condition object from dex
        // Rust returns ID - caller can look up Condition via battle.dex if needed
        &self.weather
    }

    /// Get terrain
    ///
    /// JavaScript source (field.ts getTerrain):
    // 	getTerrain() {
    // 		return this.battle.dex.conditions.getByID(this.terrain);
    // 	}
    pub fn get_terrain(&self) -> &ID {
        // return this.battle.dex.conditions.getByID(this.terrain);
        // NOTE: JavaScript returns Condition object from dex
        // Rust returns ID - caller can look up Condition via battle.dex if needed
        &self.terrain
    }

    /// Destroy field (cleanup)
    ///
    /// JavaScript source (field.ts destroy):
    // 	destroy() {
    // 		// deallocate ourself
    // 		// get rid of some possibly-circular references
    // 		(this as any).battle = null!;
    // 	}
    pub fn destroy(&mut self) {
        // deallocate ourself
        // get rid of some possibly-circular references
        // (this as any).battle = null!;
        // NOTE: Rust handles deallocation via Drop trait
        // Clear collections to free memory
        self.pseudo_weather.clear();
    }

    /// Convert field to JSON
    ///
    /// JavaScript source (field.ts toJSON):
    // 	toJSON(): AnyObject {
    // 		return State.serializeField(this);
    // 	}
    pub fn to_json(&self) -> serde_json::Value {
        // return State.serializeField(this);
        // NOTE: State.serializeField not yet implemented
        // Using serde for now
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }

    /// Get pseudo-weather state
    pub fn get_pseudo_weather(&self, id: &ID) -> Option<&SharedEffectState> {
        self.pseudo_weather.get(id)
    }
}
