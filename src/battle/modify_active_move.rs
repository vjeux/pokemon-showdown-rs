//! Battle::modify_active_move - Modify properties of the active move
//!
//! Infrastructure for move callbacks to modify the current active move's properties
//! Used by moves like Magnitude (modify basePower) and Max Guard (modify smartTarget)

use crate::*;

impl Battle {
    /// Modify the base power of the active move
    /// Equivalent to JavaScript: move.basePower = value
    ///
    /// JavaScript (used in Magnitude, Electro Ball, etc.):
    ///   move.basePower = 150;
    pub fn modify_active_move_base_power(&mut self, base_power: i32) {
        if let Some(ref mut active_move) = self.active_move {
            active_move.base_power = base_power;
        }
    }

    /// Modify the smart target property of the active move
    /// Equivalent to JavaScript: move.smartTarget = value
    ///
    /// JavaScript (used in Max Guard, Protect, etc.):
    ///   move.smartTarget = false;
    pub fn modify_active_move_smart_target(&mut self, smart_target: bool) {
        if let Some(ref mut active_move) = self.active_move {
            active_move.smart_target = Some(smart_target);
        }
    }

    /// Set a custom property on the active move
    /// Equivalent to JavaScript: move[property] = value
    ///
    /// JavaScript (used in Magnitude):
    ///   move.magnitude = 7;
    pub fn set_active_move_property(&mut self, property: &str, value: serde_json::Value) {
        if self.active_move.is_some() {
            // Store custom properties in current_effect_state
            // This is used for custom properties that don't have dedicated fields in ActiveMove
            match property {
                "magnitude" => {
                    self.with_effect_state(|state| {
                        state.magnitude = value.as_i64().map(|v| v as i32);
                    });
                }
                _ => {
                    // Unknown property - log warning
                    self.debug(&format!("Unknown active move property: {}", property));
                }
            }
        }
    }

    /// Get a custom property from the active move
    /// Equivalent to JavaScript: move[property]
    ///
    /// JavaScript (used in Magnitude onUseMoveMessage):
    ///   const magnitude = move.magnitude;
    pub fn get_active_move_property(&self, property: &str) -> Option<serde_json::Value> {
        match property {
            "magnitude" => self
                .with_effect_state_ref(|state| state.magnitude.map(|v| serde_json::json!(v)))
                .flatten(),
            _ => None,
        }
    }
}
