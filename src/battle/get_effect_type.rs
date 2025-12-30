use crate::*;

impl Battle {

    /// Get effect type for an effect ID
    /// Rust helper method - JavaScript determines effect type dynamically via duck typing
    /// This method checks the effect ID against dex lookups to categorize it
    /// Returns: "Ability", "Item", "Move", "Status", "Volatile", "Weather", "Terrain", or "Unknown"
    pub fn get_effect_type(&self, effect_id: &ID) -> &str {
        // Check if it's an ability
        if self.dex.get_ability(effect_id.as_str()).is_some() {
            return "Ability";
        }
        // Check if it's an item
        if self.dex.get_item(effect_id.as_str()).is_some() {
            return "Item";
        }
        // Check if it's a move
        if self.dex.get_move(effect_id.as_str()).is_some() {
            return "Move";
        }
        // Check if it's a condition
        if let Some(_condition) = crate::data::conditions::get_condition(effect_id) {
            // Conditions can be Status, Volatile, Weather, Terrain, etc.
            if crate::data::conditions::is_status_condition(effect_id) {
                return "Status";
            }
            if crate::data::conditions::is_volatile_condition(effect_id) {
                return "Volatile";
            }
            // Check for weather/terrain by ID
            let id_str = effect_id.as_str();
            if [
                "sunnyday",
                "raindance",
                "sandstorm",
                "hail",
                "snow",
                "harsh sunshine",
                "heavy rain",
                "strong winds",
            ]
            .contains(&id_str)
            {
                return "Weather";
            }
            if [
                "electricterrain",
                "grassyterrain",
                "mistyterrain",
                "psychicterrain",
            ]
            .contains(&id_str)
            {
                return "Terrain";
            }
        }
        "Unknown"
    }
}
