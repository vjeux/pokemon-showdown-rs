// NOTE: This method is NOT in JavaScript - Rust-specific implementation

use crate::*;

impl Battle {

    /// Get effect type for an effect ID
    /// Rust helper method - JavaScript has effectType property on effect objects
    /// This method looks up the effect by ID and returns its effectType
    /// Returns: "Ability", "Item", "Move", "Status", "Weather", "Terrain", "Condition", or "Unknown"
    ///
    /// JavaScript equivalent:
    /// In JavaScript, you have the effect object directly: `effect.effectType`
    /// In Rust, we need to look it up by ID first, then call .effect_type()
    pub fn get_effect_type(&self, effect_id: &ID) -> &str {
        let effect_str = effect_id.as_str();

        // Check abilities
        // JavaScript: ability.effectType === 'Ability'
        if let Some(ability) = self.dex.abilities().get(effect_str) {
            return ability.effect_type();
        }

        // Check items
        // JavaScript: item.effectType === 'Item'
        if let Some(item) = self.dex.items().get(effect_str) {
            return item.effect_type();
        }

        // Check moves
        // JavaScript: move.effectType === 'Move'
        if let Some(move_data) = self.dex.moves().get(effect_str) {
            return move_data.effect_type();
        }

        // Check conditions
        // JavaScript: condition.effectType (can be 'Status', 'Weather', 'Terrain', 'Condition')
        if let Some(condition) = self.dex.conditions.get(effect_id) {
            return condition.effect_type();
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
            "Status" | "Weather" | "Terrain" | "Condition" => {
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
