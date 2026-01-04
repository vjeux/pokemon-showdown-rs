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
        if let Some(condition) = self.dex.conditions().get_by_id(effect_id) {
            return condition.effect_type();
        }

        "Unknown"
    }

    /// Get effect fullname in the format "effectType: name"
    /// JavaScript equivalent: effect.fullname property
    /// Examples: "ability: Mold Breaker", "item: Life Orb", "move: Tackle"
    pub fn get_effect_fullname(&self, effect_id: &ID) -> String {
        let effect_str = effect_id.as_str();

        // Check abilities - get both type and name in one lookup
        // JavaScript: ability.fullname = `ability: ${ability.name}`
        if let Some(ability) = self.dex.abilities().get(effect_str) {
            return format!("{}: {}", ability.effect_type().to_lowercase(), ability.name);
        }

        // Check items - get both type and name in one lookup
        // JavaScript: item.fullname = `item: ${item.name}`
        if let Some(item) = self.dex.items().get(effect_str) {
            return format!("{}: {}", item.effect_type().to_lowercase(), item.name);
        }

        // Check moves - get both type and name in one lookup
        // JavaScript: move.fullname = `move: ${move.name}`
        if let Some(move_data) = self.dex.moves().get(effect_str) {
            return format!("{}: {}", move_data.effect_type().to_lowercase(), move_data.name);
        }

        // Check conditions - get both type and name in one lookup
        // JavaScript: condition.fullname format varies by effectType
        if let Some(condition) = self.dex.conditions().get_by_id(effect_id) {
            let name = condition.name.as_deref().unwrap_or(effect_str);
            return format!("{}: {}", condition.effect_type().to_lowercase(), name);
        }

        // Fallback - unknown effect
        format!("unknown: {}", effect_str)
    }
}
