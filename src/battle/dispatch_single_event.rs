// NOTE: This method is NOT in JavaScript - Rust-specific implementation

use crate::*;

impl Battle {

    /// Dispatch a single event to the appropriate handler
    /// Rust helper method - JavaScript's singleEvent() calls handler functions directly
    /// This method routes events to specialized handlers based on effect type
    /// Routes to: handle_ability_event, handle_item_event, handle_move_event, handle_condition_event
    pub fn dispatch_single_event(
        &mut self,
        event_id: &str,
        effect_id: &ID,
        target: Option<(usize, usize)>,
        _source: Option<(usize, usize)>,
    ) -> crate::event::EventResult {
        use crate::event::EventResult;

        let effect_str = effect_id.as_str();

        // IMPORTANT: For PrepareHit, check if effect is a move FIRST before checking volatiles
        // PrepareHit should call the MOVE's handler (e.g., King's Shield's onPrepareHit),
        // not a volatile's handler (even if the Pokemon has a kingsshield volatile)
        if event_id == "PrepareHit" {
            if let Some(_move_def) = self.dex.moves().get(effect_id.as_str()) {
                return self.handle_move_event(event_id, effect_str, target);
            }
        }

        // IMPORTANT: Check if effect is a condition (volatile, status, etc.) on the target Pokemon FIRST
        // This prevents "substitute" volatile from being dispatched as "substitute" move
        // In JavaScript, the event handler is attached to pokemon.volatiles['substitute'], so it knows it's the volatile
        // In Rust, we need to check target.volatiles to determine if it's a volatile vs a move
        if let Some(target_pos) = target {
            if let Some(pokemon) = self.pokemon_at(target_pos.0, target_pos.1) {
                // Check if effect is in target's volatiles
                if pokemon.volatiles.contains_key(effect_id) {
                    return self.handle_condition_event(event_id, effect_str, target);
                }
                // Check if effect is target's status
                if !pokemon.status.is_empty() && pokemon.status.as_str() == effect_str {
                    return self.handle_condition_event(event_id, effect_str, target);
                }
            }
        }

        // Handle ability events
        if self.dex.abilities().get(effect_id.as_str()).is_some() {
            return self.handle_ability_event(event_id, effect_id, target);
        }

        // Handle item events
        if self.dex.items().get(effect_id.as_str()).is_some() {
            return self.handle_item_event(event_id, effect_id, target);
        }

        // IMPORTANT: Check field weather/terrain BEFORE moves
        // "sunnyday", "raindance", etc. exist both as moves AND as weather conditions
        // When weather is active, handlers should route to the condition, not the move
        if !self.field.weather.is_empty() && self.field.weather.as_str() == effect_str {
            return self.handle_condition_event(event_id, effect_str, target);
        }
        if !self.field.terrain.is_empty() && self.field.terrain.as_str() == effect_str {
            return self.handle_condition_event(event_id, effect_str, target);
        }

        // Handle move events
        if let Some(_move_def) = self.dex.moves().get(effect_id.as_str()) {
            return self.handle_move_event(event_id, effect_str, target);
        }

        // Handle condition events (status, volatile, weather, terrain)
        if let Some(_condition) = crate::data::conditions::get_condition(effect_id) {
            return self.handle_condition_event(event_id, effect_str, target);
        }

        EventResult::Continue
    }
}
