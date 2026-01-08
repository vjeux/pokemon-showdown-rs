// NOTE: This method is NOT in JavaScript - Rust-specific implementation

use crate::*;

impl Battle {

    /// Check if an effect has a callback for a specific event
    /// This is a Rust helper to replicate JavaScript's getCallback() check
    /// without actually executing the callback
    ///
    /// Returns true if the effect has a handler for the event, false otherwise
    pub fn has_callback(&self, effect_id: &ID, event_id: &str) -> bool {
        let effect_str = effect_id.as_str();

        // IMPORTANT: Check conditions (status, volatile, weather, terrain) FIRST
        // This is critical because some IDs like "stall" exist as both abilities AND conditions
        // When checking a volatile, we want to find the CONDITION, not the ability
        // In JavaScript, the volatile already has its callback attached, so there's no ambiguity
        //
        // HOWEVER: If the condition is a move-embedded condition (like gmaxvolcalith),
        // we need to check BOTH condition callbacks AND move callbacks!
        // Example: gmaxvolcalith has condition.onSideStart AND self.onHit
        let condition_check = self.dex.conditions().get_by_id(effect_id);
        let is_also_move = self.dex.moves().get(effect_str).is_some();

        if condition_check.is_some() {
            let has_condition_callback = self.condition_has_callback(effect_str, event_id);

            // If it's ONLY a condition (not also a move), return the condition result
            if !is_also_move {
                return has_condition_callback;
            }

            // If it's BOTH a condition and a move, check move callbacks too
            // Fall through to check move callbacks below, but remember condition result
            let has_move_callback = self.move_has_callback(effect_str, event_id);
            let has_self_callback = self.move_has_self_callback(effect_str, event_id);
            return has_condition_callback || has_move_callback || has_self_callback;
        }

        // Check if this is a move-embedded condition
        // JavaScript's dex.conditions.getByID() checks moves for embedded conditions:
        // } else if (this.dex.data.Moves.hasOwnProperty(id) && (found = this.dex.data.Moves[id]).condition ...
        //     condition = new Condition({ name: found.name || id, ...found.condition });
        // IMPORTANT: Don't return early! A move with an embedded condition can have BOTH
        // condition callbacks (onSideStart, onResidual, etc.) AND move callbacks (onHit, etc.)
        // Example: gmaxvolcalith has condition.onSideStart AND self.onHit
        let has_embedded_condition = if let Some(move_data) = self.dex.moves().get(effect_str) {
            if move_data.condition.is_some() {
                // This is a move with an embedded condition (like King's Shield, gmaxvolcalith)
                // Check if the condition has the callback
                self.condition_has_callback(effect_str, event_id)
            } else {
                false
            }
        } else {
            false
        };

        // Check ability-embedded conditions
        // Some abilities define a `condition` block with their own callbacks
        // When a volatile is added with the ability's ID (e.g., "flashfire"),
        // its callbacks are in ability.condition, not in conditions.json
        if let Some(ability_data) = self.dex.abilities().get(effect_str) {
            if let Some(condition_value) = ability_data.extra.get("condition") {
                if let Some(condition) = condition_value.as_object() {
                    // Check if the condition has this callback
                    let has_key = condition.contains_key(event_id);
                    let priority_key = format!("{}Priority", event_id);
                    let has_priority_key = condition.contains_key(&priority_key);

                    if has_key || has_priority_key {
                        return true;
                    }

                    // Try with "on" prefix
                    if !event_id.starts_with("on") {
                        let with_on = format!("on{}", event_id);
                        let priority_with_on = format!("on{}Priority", event_id);
                        if condition.contains_key(&with_on) || condition.contains_key(&priority_with_on) {
                            return true;
                        }
                    }
                }
            }
        }

        // Check abilities (for ability-specific callbacks like onTryHit, onEnd)
        if self.dex.abilities().get(effect_str).is_some() {
            return self.ability_has_callback(effect_str, event_id);
        }

        // Check moves BEFORE items (both regular and self callbacks)
        // This is important because some IDs like "metronome" exist as both move AND item
        // When checking callbacks during move execution, we want the move callbacks
        if self.dex.moves().get(effect_str).is_some() {
            // A move has a callback if:
            // 1. The embedded condition has the callback (already checked above)
            // 2. The move itself has the callback
            // 3. The move's self effect has the callback
            let has_move_callback = self.move_has_callback(effect_str, event_id);
            let has_self_callback = self.move_has_self_callback(effect_str, event_id);
            return has_embedded_condition || has_move_callback || has_self_callback;
        }

        // Check items
        if self.dex.items().get(effect_str).is_some() {
            return self.item_has_callback(effect_str, event_id);
        }

        // Check species - species can have callbacks like onSwitchIn for form changes
        if self.dex.species().get(effect_str).is_some() {
            return self.species_has_callback(effect_str, event_id);
        }

        false
    }

    /// Check if a an ability has a callback for an event
    pub fn ability_has_callback(&self, ability_id: &str, event_id: &str) -> bool {
        // Gen 5+ special case: abilities use onStart during SwitchIn events
        // This matches JavaScript getCallback() behavior
        // JavaScript: if (callback === undefined && target instanceof Pokemon && this.gen >= 5 && callbackName === 'onSwitchIn' &&
        //             !(effect as any).onAnySwitchIn && (['Ability', 'Item'].includes(effect.effectType) ...)) {
        //             callback = (effect as any).onStart;
        // }
        if self.gen >= 5 && event_id == "onSwitchIn" {
            // Check if ability has onAnySwitchIn - if yes, use normal SwitchIn logic
            // This recursive call is safe because event_id != "onSwitchIn" so won't trigger special case
            let has_any_switch_in = self.ability_has_callback(ability_id, "onAnySwitchIn");

            // If ability doesn't have onAnySwitchIn, check for onStart instead
            // This recursive call is safe because event_id != "onSwitchIn" so won't trigger special case
            if !has_any_switch_in && self.ability_has_callback(ability_id, "onStart") {
                return true;
            }
        }

        // Look up the ability in dex data and check its extra field for callback boolean
        if let Some(ability_data) = self.dex.abilities().get(ability_id) {
            // Check the exact event_id first, then try with "on" prefix for backward compatibility
            let has_callback = ability_data.extra.get(event_id)
                .and_then(|v| v.as_bool())
                .unwrap_or(false);

            if has_callback {
                true
            } else if !event_id.starts_with("on") {
                // Try with "on" prefix for backward compatibility
                let with_on = format!("on{}", event_id);
                ability_data.extra.get(&with_on)
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false)
            } else {
                false
            }
        } else {
            // If not found in dex, return false
            false
        }
    }

    /// Check if a an item has a callback for an event
    pub fn item_has_callback(&self, item_id: &str, event_id: &str) -> bool {
        // Gen 5+ special case: items use onStart during SwitchIn events
        // This matches JavaScript getCallback() behavior (same logic as abilities)
        // Items don't have onAnySwitchIn callbacks, so always check onStart if gen >= 5
        if self.gen >= 5 && event_id == "onSwitchIn" {
            // This recursive call is safe because event_id != "onSwitchIn" so won't trigger special case
            if self.item_has_callback(item_id, "onStart") {
                return true;
            }
        }

        // Look up the item in dex data and check its extra field for callback boolean
        if let Some(item_data) = self.dex.items().get(item_id) {
            // Check the exact event_id first, then try with "on" prefix for backward compatibility
            let has_callback = item_data.extra.get(event_id)
                .and_then(|v| v.as_bool())
                .unwrap_or(false);

            if has_callback {
                true
            } else if !event_id.starts_with("on") {
                // Try with "on" prefix for backward compatibility
                let with_on = format!("on{}", event_id);
                item_data.extra.get(&with_on)
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false)
            } else {
                false
            }
        } else {
            // If not found in dex, return false
            false
        }
    }

    /// Check if a move has a callback for an event
    pub fn move_has_callback(&self, move_id: &str, event_id: &str) -> bool {
        // Check if we have a dispatch function for this move's callback
        // This is important because the dex data might not have the callback flag set
        // but we have the callback implemented in Rust
        if event_id == "Hit" || event_id == "onHit" {
            if crate::data::move_callbacks::has_on_hit(move_id) {
                return true;
            }
        }

        // Look up the move in dex data and check its extra field for callback boolean
        if let Some(move_data) = self.dex.moves().get(move_id) {
            // Check the move's direct callbacks (not self callbacks)
            let has_callback = move_data.extra.get(event_id)
                .and_then(|v| v.as_bool())
                .unwrap_or(false);

            if has_callback {
                return true;
            }

            // Try with "on" prefix for backward compatibility
            if !event_id.starts_with("on") {
                let with_on = format!("on{}", event_id);
                let has_callback_with_on = move_data.extra.get(&with_on)
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false);
                if has_callback_with_on {
                    return true;
                }
            }

            false
        } else {
            // If not found in dex, return false
            false
        }
    }

    /// Check if a move has a SELF callback for an event
    /// Self callbacks are in the self: { } object and target the move user, not the target
    pub fn move_has_self_callback(&self, move_id: &str, event_id: &str) -> bool {
        // Look up the move in dex data and check its extra field for callback boolean
        if let Some(move_data) = self.dex.moves().get(move_id) {
            // Check move.self callbacks (like gmaxmalodor)
            // In JavaScript, these are callbacks in the self: { ... } object
            // They're triggered on the move user, not the target
            // The JSON is flattened, so we check for "self.onHit" style keys
            let self_key = if event_id.starts_with("on") {
                format!("self.{}", event_id)
            } else {
                format!("self.on{}", event_id)
            };

            if let Some(has_self_callback) = move_data.extra.get(&self_key).and_then(|v| v.as_bool()) {
                if has_self_callback {
                    return true;
                }
            }

            // Also try without "on" prefix if event_id doesn't have it
            if !event_id.starts_with("on") {
                let self_key_no_on = format!("self.{}", event_id);
                if let Some(has_self_callback) = move_data.extra.get(&self_key_no_on).and_then(|v| v.as_bool()) {
                    if has_self_callback {
                        return true;
                    }
                }
            }

            false
        } else {
            // If not found in dex, return false
            false
        }
    }

    /// Check if a condition has a callback for an event
    ///
    /// In JavaScript, this is done by checking if effect[callbackName] exists.
    /// In Rust, we check against the dispatcher implementations to see which
    /// conditions actually have which callbacks.
    ///
    /// This prevents false positives where we'd add handlers for callbacks
    /// that don't exist, which would cause incorrect speed_sort shuffling.
    pub fn condition_has_callback(&self, condition_id: &str, event_id: &str) -> bool {
        // Special case: conditions don't have onAnySwitchIn
        if event_id == "onAnySwitchIn" || event_id == "AnySwitchIn" {
            return false;
        }

        // Look up the condition in dex data and check its extra field for callback boolean
        let id = ID::from(condition_id);
        if let Some(condition_data) = self.dex.conditions().get_by_id(&id) {
            // IMPORTANT: In JavaScript, conditions can have callbacks that are static values (not functions)
            // Example: phantomforce has onInvulnerability: false
            // These static values create handlers that return the static value
            // So we need to check if the KEY exists, not just if the value is true
            //
            // Check the exact event_id first, then try with "on" prefix for backward compatibility
            // ALSO check for Priority variant: if "onModifySpAPriority" exists, then "onModifySpA" callback exists
            let has_key = condition_data.extra.contains_key(event_id);
            let priority_key = format!("{}Priority", event_id);
            let has_priority_key = condition_data.extra.contains_key(&priority_key);

            if has_key || has_priority_key {
                true
            } else if !event_id.starts_with("on") {
                // Try with "on" prefix for backward compatibility
                let with_on = format!("on{}", event_id);
                let priority_with_on = format!("on{}Priority", event_id);
                let has_key_with_on = condition_data.extra.contains_key(&with_on);
                let has_priority_with_on = condition_data.extra.contains_key(&priority_with_on);
                has_key_with_on || has_priority_with_on
            } else {
                // NOTE: onSideResidual and onResidual are DIFFERENT callbacks:
                // - onSideResidual: called once per side, no Pokemon target
                // - onResidual: called once per Pokemon on the side
                // Do NOT treat onResidual as a fallback for onSideResidual.
                // The Pokemon-targeted onResidual handlers are found via
                // find_side_event_handlers with custom_holder in field_event.

                // Special case: Some field conditions use "onResidual" instead of "onFieldResidual"
                // Example: grassyterrain has condition.onResidual that should match onFieldResidual requests
                // JavaScript allows this because the callback signature matches (both take target)
                if event_id == "onFieldResidual" {
                    if condition_data.extra.contains_key("onResidual") {
                        return true;
                    }
                }

                // Before returning false, check if this is a move-embedded condition
                // Even though the condition exists in the dex, the callback might be in the move dispatcher
                if let Some(move_data) = self.dex.moves().get(condition_id) {
                    if let Some(ref condition_data) = move_data.condition {
                        // Check if the key exists in the condition data (not just if it's true)
                        // This was populated by scripts/update-move-callbacks.js
                        if condition_data.extra.contains_key(event_id) {
                            return true;
                        }

                        // NOTE: Do NOT fallback from onSideResidual to onResidual for embedded conditions.
                        // They are different callbacks (per-side vs per-Pokemon).
                    }
                }
                false
            }
        } else {
            // If not found in conditions dex, check if this is a move-embedded condition
            // Some moves like "kingsshield" create volatile conditions with their own callbacks
            // that are hardcoded in the move_callbacks dispatcher

            // Check if this is a move with an embedded condition
            if let Some(move_data) = self.dex.moves().get(condition_id) {
                if let Some(ref condition_data) = move_data.condition {
                    // Check if the key exists in the condition data (not just if it's true)
                    // This was populated by scripts/update-move-callbacks.js
                    if condition_data.extra.contains_key(event_id) {
                        return true;
                    }
                }
            }

            false
        }
    }

    /// Check if a species has a callback for an event
    pub fn species_has_callback(&self, _species_id: &str, event_id: &str) -> bool {
        // Species don't have onAnySwitchIn
        if event_id == "onAnySwitchIn" {
            return false;
        }

        // For other events, conservatively return false by default
        false
    }
}
