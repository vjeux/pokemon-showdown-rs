//! Simulator Field
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Represents the battle field - weather, terrain, and pseudo-weather conditions.

use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use crate::dex_data::{ID, EffectState};

/// The battle field - contains weather, terrain, and field-wide effects
#[derive(Debug, Clone, Serialize, Deserialize)]
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
    pub fn set_weather(&mut self, weather_id: ID, duration: Option<u32>) -> bool {
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
    pub fn effective_weather(&self) -> &ID {
        &self.weather
    }

    /// Check if weather matches
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
    pub fn set_terrain(&mut self, terrain_id: ID, duration: Option<u32>) -> bool {
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
    pub fn clear_terrain(&mut self) -> bool {
        if self.terrain.is_empty() {
            return false;
        }
        self.terrain = ID::empty();
        self.terrain_state = EffectState::new(ID::empty());
        true
    }

    /// Check if terrain matches
    pub fn is_terrain(&self, terrain: &str) -> bool {
        self.terrain.as_str() == terrain.to_lowercase()
    }

    /// Add a pseudo-weather condition
    pub fn add_pseudo_weather(&mut self, id: ID, duration: Option<u32>) -> bool {
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
    pub fn get_pseudo_weather(&self, id: &ID) -> Option<&EffectState> {
        self.pseudo_weather.get(id)
    }

    /// Get mutable pseudo-weather state
    pub fn get_pseudo_weather_mut(&mut self, id: &ID) -> Option<&mut EffectState> {
        self.pseudo_weather.get_mut(id)
    }

    /// Remove a pseudo-weather
    pub fn remove_pseudo_weather(&mut self, id: &ID) -> bool {
        self.pseudo_weather.remove(id).is_some()
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
