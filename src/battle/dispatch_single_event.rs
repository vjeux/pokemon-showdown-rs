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
        source: Option<(usize, usize)>,
    ) -> crate::event::EventResult {
        use crate::event::EventResult;

        if event_id.contains("Invulnerability") {
            eprintln!("[DISPATCH_SINGLE_EVENT] event_id={}, effect_id={}, target={:?}",
                event_id, effect_id.as_str(), target);
        }

        let effect_str = effect_id.as_str();

        // IMPORTANT: For Try, PrepareHit and Hit events, check if effect is a MOVE first
        // This ensures that when single_event("Try", "noretreat", ...) is called,
        // it routes to the MOVE's onTry handler, not the volatile's handler
        // Same for PrepareHit and Hit events
        // Note: run_event() now calls handlers directly based on effect_type, so this only affects single_event calls
        if event_id == "Try" || event_id == "PrepareHit" || event_id == "Hit" {
            if let Some(_move_def) = self.dex.moves().get(effect_id.as_str()) {
                return self.handle_move_event(event_id, effect_id, target, source);
            }
        }

        // Check if effect is a condition (volatile, status, etc.) on the target Pokemon
        // This handles cases where a volatile needs to be checked before other effect types
        if let Some(target_pos) = target {
            if let Some(pokemon) = self.pokemon_at(target_pos.0, target_pos.1) {
                // Check if effect is in target's volatiles
                if pokemon.volatiles.contains_key(effect_id) {
                    eprintln!("[DISPATCH_SINGLE_EVENT] Found effect '{}' in target volatiles, routing to handle_condition_event", effect_str);

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
                    eprintln!("[DISPATCH_SINGLE_EVENT] Found effect '{}' as target status, routing to handle_condition_event", effect_str);
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
            eprintln!("[DISPATCH_SINGLE_EVENT] Dispatching to handle_item_event for {}", effect_id.as_str());
            return self.handle_item_event(event_id, effect_id, target);
        } else {
            eprintln!("[DISPATCH_SINGLE_EVENT] {} is NOT an item in dex", effect_id.as_str());
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
