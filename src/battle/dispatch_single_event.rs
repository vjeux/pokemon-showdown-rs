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
        target: Option<&crate::event::EventTarget>,
        source: Option<(usize, usize)>,
    ) -> crate::event::EventResult {
        use crate::event::EventResult;

        // Extract pokemon position from EventTarget for pokemon checks
        let target_pos = target.and_then(|t| t.as_pokemon());

        let effect_str = effect_id.as_str();

        // IMPORTANT: For Try, TryMove, PrepareHit and Hit events, check if effect is a MOVE first
        // This ensures that when single_event("Try", "noretreat", ...) is called,
        // it routes to the MOVE's onTry handler, not the volatile's handler
        // Same for TryMove, PrepareHit and Hit events
        // For two-turn moves like Meteor Beam, the move is also added as a volatile,
        // so we need to prioritize the MOVE handler for TryMove events
        // Note: run_event() now calls handlers directly based on effect_type, so this only affects single_event calls
        if event_id == "Try" || event_id == "TryMove" || event_id == "PrepareHit" || event_id == "Hit" {
            if let Some(_move_def) = self.dex.moves().get(effect_id.as_str()) {
                return self.handle_move_event(event_id, effect_id, target, source);
            }
        }

        // Check if effect is a condition (volatile, status, etc.) on the target Pokemon
        // This handles cases where a volatile needs to be checked before other effect types
        if let Some(target_pokemon_pos) = target_pos {
            if let Some(pokemon) = self.pokemon_at(target_pokemon_pos.0, target_pokemon_pos.1) {
                // Check if effect is in target's volatiles
                if pokemon.volatiles.contains_key(effect_id) {
                    // Set current_effect_state from the volatile's state (like JS does with this.effectState)
                    let volatile_state = pokemon.volatiles.get(effect_id).cloned();
                    let previous_effect_state = self.current_effect_state.take();
                    self.current_effect_state = volatile_state;

                    let result = self.handle_condition_event(event_id, effect_str, target);

                    // Restore previous effect state
                    self.current_effect_state = previous_effect_state;

                    return result;
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
            return self.handle_move_event(event_id, effect_id, target, source);
        }

        // Handle condition events (status, volatile, weather, terrain)
        if let Some(_condition) = self.dex.conditions().get_by_id(effect_id) {
            return self.handle_condition_event(event_id, effect_str, target);
        }

        EventResult::Continue
    }
}
