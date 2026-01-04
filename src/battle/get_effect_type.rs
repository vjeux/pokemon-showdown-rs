// NOTE: This method is NOT in JavaScript - Rust-specific implementation

use crate::*;

impl Battle {

    /// Get effect type for an effect ID
    /// Rust helper method - JavaScript determines effect type dynamically via duck typing
    /// This method checks the effect ID against dex lookups to categorize it
    /// Returns: "Ability", "Item", "Move", "Status", "Volatile", "Weather", "Terrain", or "Unknown"
    pub fn get_effect_type(&self, effect_id: &ID) -> &str {
        // Check if it's an ability
        if self.dex.abilities().get(effect_id.as_str()).is_some() {
            return "Ability";
        }
        // Check if it's an item
        if self.dex.items().get(effect_id.as_str()).is_some() {
            return "Item";
        }

        // IMPORTANT: Check if this effect is the active field weather/terrain BEFORE checking moves
        // Some IDs like "sandstorm" exist as both moves and weather conditions
        // When the weather is active, we should return "Weather", not "Move"
        let effect_str = effect_id.as_str();
        if !self.field.weather.is_empty() && self.field.weather.as_str() == effect_str {
            return "Weather";
        }
        if !self.field.terrain.is_empty() && self.field.terrain.as_str() == effect_str {
            return "Terrain";
        }

        // Check if it's a move
        if self.dex.moves().get(effect_id.as_str()).is_some() {
            return "Move";
        }
        // Check if it's a condition
        if let Some(condition) = self.dex.conditions.get(effect_id) {
            // Conditions can be Status, Volatile, Weather, Terrain, etc.
            // JavaScript checks effectType field: if (effect.effectType === 'Status')
            use crate::dex::ConditionType;
            match condition.condition_type() {
                ConditionType::Status => return "Status",
                ConditionType::Volatile => return "Volatile",
                ConditionType::Weather => return "Weather",
                ConditionType::Terrain => return "Terrain",
                ConditionType::SideCondition => return "Condition",
                ConditionType::SlotCondition => return "Condition",
                ConditionType::PseudoWeather => return "Condition",
            }
        }
        "Unknown"
    }

    /// Get effect fullname in the format "effectType: name"
    /// JavaScript equivalent: effect.fullname property
    /// Examples: "ability: Mold Breaker", "item: Life Orb", "move: Tackle"
    pub fn get_effect_fullname(&self, effect_id: &ID) -> String {
        // Get effect type
        let effect_type = self.get_effect_type(effect_id);

        // Get effect name based on type
        let name = match effect_type {
            "Ability" => {
                self.dex
                    .abilities()
                    .get_by_id(effect_id)
                    .map(|a| a.name.clone())
                    .unwrap_or_else(|| effect_id.as_str().to_string())
            }
            "Item" => {
                self.dex
                    .items()
                    .get_by_id(effect_id)
                    .map(|i| i.name.clone())
                    .unwrap_or_else(|| effect_id.as_str().to_string())
            }
            "Move" => {
                self.dex
                    .moves()
                    .get_by_id(effect_id)
                    .map(|m| m.name.clone())
                    .unwrap_or_else(|| effect_id.as_str().to_string())
            }
            "Status" | "Volatile" | "Weather" | "Terrain" => {
                // For conditions, get name from condition data if available
                self.dex.conditions.get(effect_id)
                    .map(|c| c.name.clone())
                    .unwrap_or_else(|| effect_id.as_str().to_string())
            }
            _ => effect_id.as_str().to_string(),
        };

        // JavaScript fullname format: "effectType: name"
        // Note: JavaScript uses lowercase effect type (e.g., "ability: Mold Breaker")
        format!("{}: {}", effect_type.to_lowercase(), name)
    }
}
