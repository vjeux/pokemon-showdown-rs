// NOTE: This method is NOT in JavaScript - Rust-specific implementation
//
// BACKGROUND: JavaScript vs Rust event dispatch
// ==============================================
//
// In JavaScript, singleEvent() directly looks up callbacks on the effect object:
//
//     // battle.ts line 624
//     const callback = customCallback || (effect as any)[`on${eventid}`];
//     if (callback === undefined) return relayVar;
//
// The magic happens in dex.conditions.get() which extracts embedded conditions:
//
//     // dex-conditions.ts lines 687-692
//     else if (
//         (this.dex.data.Moves.hasOwnProperty(id) && (found = this.dex.data.Moves[id]).condition) ||
//         (this.dex.data.Abilities.hasOwnProperty(id) && (found = this.dex.data.Abilities[id]).condition) ||
//         (this.dex.data.Items.hasOwnProperty(id) && (found = this.dex.data.Items[id]).condition)
//     ) {
//         condition = new Condition({ name: found.name || id, ...found.condition });
//     }
//
// So when JavaScript calls dex.conditions.get("wish"):
// 1. It finds "wish" is a Move with a .condition property
// 2. It extracts move.condition and returns it as a Condition object
// 3. The Condition object has onStart, onResidual, onEnd directly on it
// 4. singleEvent just does effect['on' + eventid] to get the callback
//
// In Rust, we don't have this extraction step. Instead, we route based on
// effect type and look up the embedded condition in the handler. Both
// approaches achieve the same result.

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

        // If we're in a Move context, prioritize the move handler over volatiles with the same name
        // This handles cases like "noretreat" which exists both as a move and as a volatile
        if let Some(ref effect) = self.effect {
            if effect.effect_type == crate::battle::EffectType::Move {
                if let Some(_move_def) = self.dex.moves().get(effect_id.as_str()) {
                    return self.handle_move_event(event_id, effect_id, target, source);
                }
            }
        }

        // Check if effect is a condition (volatile, status, etc.) on the target Pokemon
        // This handles cases where a volatile needs to be checked before other effect types
        if let Some(target_pokemon_pos) = target_pos {
            if let Some(pokemon) = self.pokemon_at(target_pokemon_pos.0, target_pokemon_pos.1) {
                // Check if effect is in target's volatiles
                if pokemon.volatiles.contains_key(effect_id) {
                    // The context is already set up by single_event with effect_holder = target_pos
                    // Callbacks should use with_effect_state to access/modify the volatile's state
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
        //
        // EXCEPTION: For side/slot condition events, check if the move has an embedded
        // condition and route to condition handler. This mimics JavaScript's behavior
        // where dex.conditions.get("wish") extracts the move's .condition property:
        //
        //     // dex-conditions.ts lines 687-692
        //     else if (
        //         (this.dex.data.Moves.hasOwnProperty(id) && (found = this.dex.data.Moves[id]).condition) ||
        //         ...
        //     ) {
        //         condition = new Condition({ name: found.name || id, ...found.condition });
        //     }
        //
        // Examples:
        // - "wish" is a move with condition.onStart, condition.onResidual, condition.onEnd
        // - "gmaxvolcalith" is a move with condition.onSideResidual
        //
        if let Some(move_def) = self.dex.moves().get(effect_id.as_str()) {
            // Check if we're in a side/slot condition context and the move has an embedded condition
            // The effect type tells us the context - no need to check specific event IDs
            if let Some(ref effect) = self.effect {
                if (effect.effect_type == crate::battle::EffectType::SideCondition
                    || effect.effect_type == crate::battle::EffectType::SlotCondition)
                    && move_def.condition.is_some()
                {
                    // Route to condition handler for the embedded condition
                    return self.handle_condition_event(event_id, effect_str, target);
                }
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
