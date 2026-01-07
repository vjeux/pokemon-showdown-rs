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

        // IMPORTANT: For Try, TryMove, PrepareHit, Hit and ModifyType events, check if effect is a MOVE first
        // This ensures that when single_event("Try", "noretreat", ...) is called,
        // it routes to the MOVE's onTry handler, not the volatile's handler
        // Same for TryMove, PrepareHit, Hit and ModifyType events
        // For two-turn moves like Meteor Beam, the move is also added as a volatile,
        // so we need to prioritize the MOVE handler for TryMove events
        // For moves like Ivy Cudgel that change type based on the user's species,
        // we need to prioritize the MOVE handler for ModifyType events
        // Note: run_event() now calls handlers directly based on effect_type, so this only affects single_event calls
        if event_id == "Try" || event_id == "TryMove" || event_id == "PrepareHit" || event_id == "Hit" || event_id == "ModifyType" {
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

                    // CRITICAL FIX: Save modified current_effect_state back to Pokemon's volatile
                    // In JavaScript, this.effectState is a REFERENCE to this.volatiles[status.id]
                    // In Rust, we cloned it above, so we must copy it back after the callback modifies it
                    // This fixes bugs where callbacks set effectState.trueDuration, effectState.move, etc.
                    if let Some(modified_state) = self.current_effect_state.take() {
                        if let Some(pokemon_mut) = self.pokemon_at_mut(target_pokemon_pos.0, target_pokemon_pos.1) {
                            if let Some(volatile) = pokemon_mut.volatiles.get_mut(effect_id) {
                                // Copy the modified state back
                                *volatile = modified_state;
                            }
                        }
                    }

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
        // EXCEPTION: For side condition events (SideResidual, SideStart, SideEnd, SideRestart),
        // check if the move has an embedded condition and route to condition handler
        // Example: gmaxvolcalith is a move with condition.onResidual
        if let Some(move_def) = self.dex.moves().get(effect_id.as_str()) {
            // Check if this is a side condition event and the move has an embedded condition
            if (event_id == "SideResidual" || event_id == "SideStart" || event_id == "SideEnd" || event_id == "SideRestart")
                && move_def.condition.is_some() {
                // Route to condition handler for the embedded condition
                return self.handle_condition_event(event_id, effect_str, target);
            }
            // Normal move events
            return self.handle_move_event(event_id, effect_id, target, source);
        }

        // Handle condition events (status, volatile, weather, terrain)
        if let Some(_condition) = self.dex.conditions().get_by_id(effect_id) {
            return self.handle_condition_event(event_id, effect_str, target);
        }

        EventResult::Continue
    }
}
