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

        eprintln!("[HAS_CALLBACK] Checking effect_id={}, event_id={}", effect_str, event_id);

        // IMPORTANT: Check conditions (status, volatile, weather, terrain) FIRST
        // This is critical because some IDs like "stall" exist as both abilities AND conditions
        // When checking a volatile, we want to find the CONDITION, not the ability
        // In JavaScript, the volatile already has its callback attached, so there's no ambiguity
        let condition_check = self.dex.conditions().get_by_id(effect_id);
        if condition_check.is_some() {
            let result = self.condition_has_callback(effect_str, event_id);
            eprintln!("[HAS_CALLBACK] Found as condition, has_callback={}", result);
            return result;
        }

        // Check if this is a move-embedded condition
        // JavaScript's dex.conditions.getByID() checks moves for embedded conditions:
        // } else if (this.dex.data.Moves.hasOwnProperty(id) && (found = this.dex.data.Moves[id]).condition ...
        //     condition = new Condition({ name: found.name || id, ...found.condition });
        if let Some(move_data) = self.dex.moves().get(effect_str) {
            if move_data.condition.is_some() {
                // This is a move with an embedded condition (like King's Shield)
                // Treat it as a condition for callback purposes
                let result = self.condition_has_callback(effect_str, event_id);
                eprintln!("[HAS_CALLBACK] Found as move-embedded condition, has_callback={}", result);
                return result;
            }
        }

        // Check abilities
        if self.dex.abilities().get(effect_str).is_some() {
            return self.ability_has_callback(effect_str, event_id);
        }

        // Check items
        if self.dex.items().get(effect_str).is_some() {
            return self.item_has_callback(effect_str, event_id);
        }

        // Check moves
        if self.dex.moves().get(effect_str).is_some() {
            return self.move_has_callback(effect_str, event_id);
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

    /// Check if a a move has a callback for an event
    pub fn move_has_callback(&self, move_id: &str, event_id: &str) -> bool {
        // Look up the move in dex data and check its extra field for callback boolean
        if let Some(move_data) = self.dex.moves().get(move_id) {
            // Check the exact event_id first, then try with "on" prefix for backward compatibility
            let has_callback = move_data.extra.get(event_id)
                .and_then(|v| v.as_bool())
                .unwrap_or(false);

            if has_callback {
                true
            } else if !event_id.starts_with("on") {
                // Try with "on" prefix for backward compatibility
                let with_on = format!("on{}", event_id);
                move_data.extra.get(&with_on)
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

    /// Check if a condition has a callback for an event
    ///
    /// In JavaScript, this is done by checking if effect[callbackName] exists.
    /// In Rust, we check against the dispatcher implementations to see which
    /// conditions actually have which callbacks.
    ///
    /// This prevents false positives where we'd add handlers for callbacks
    /// that don't exist, which would cause incorrect speed_sort shuffling.
    pub fn condition_has_callback(&self, condition_id: &str, event_id: &str) -> bool {
        eprintln!("[CONDITION_HAS_CALLBACK] condition_id={}, event_id={}", condition_id, event_id);

        // Special case: conditions don't have onAnySwitchIn
        if event_id == "onAnySwitchIn" || event_id == "AnySwitchIn" {
            eprintln!("[CONDITION_HAS_CALLBACK] Returning false for AnySwitchIn");
            return false;
        }

        // Look up the condition in dex data and check its extra field for callback boolean
        let id = ID::from(condition_id);
        if let Some(condition_data) = self.dex.conditions().get_by_id(&id) {
            eprintln!("[CONDITION_HAS_CALLBACK] Found condition in dex");
            // Check the exact event_id first, then try with "on" prefix for backward compatibility
            let has_callback = condition_data.extra.get(event_id)
                .and_then(|v| v.as_bool())
                .unwrap_or(false);

            if has_callback {
                eprintln!("[CONDITION_HAS_CALLBACK] Found exact match: {}", event_id);
                true
            } else if !event_id.starts_with("on") {
                // Try with "on" prefix for backward compatibility
                let with_on = format!("on{}", event_id);
                let result = condition_data.extra.get(&with_on)
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false);
                eprintln!("[CONDITION_HAS_CALLBACK] Tried with 'on' prefix: {}={}", with_on, result);
                result
            } else {
                eprintln!("[CONDITION_HAS_CALLBACK] No match found in condition data, checking if move-embedded");
                // Before returning false, check if this is a move-embedded condition
                // Even though the condition exists in the dex, the callback might be in the move dispatcher
                if let Some(move_data) = self.dex.moves().get(condition_id) {
                    if move_data.condition.is_some() {
                        eprintln!("[CONDITION_HAS_CALLBACK] Found as move with embedded condition (from dex branch)");
                        // Check if this is a known event for move-embedded conditions
                        match event_id {
                            "onStart" | "onTryHit" | "onTryPrimaryHit" | "onHit" | "onEnd" |
                            "onSourceModifyDamage" | "onDisableMove" |
                            "onAnyInvulnerability" | "onInvulnerability" | "Invulnerability" => {
                                eprintln!("[CONDITION_HAS_CALLBACK] Returning true for known move-condition event (from dex branch)");
                                return true;
                            },
                            _ => {}
                        }
                    }
                }
                eprintln!("[CONDITION_HAS_CALLBACK] Returning false - not found");
                false
            }
        } else {
            eprintln!("[CONDITION_HAS_CALLBACK] Not found in conditions dex, checking move-embedded");
            // If not found in conditions dex, check if this is a move-embedded condition
            // Some moves like "kingsshield" create volatile conditions with their own callbacks
            // that are hardcoded in the move_callbacks dispatcher

            // Check if this is a move with an embedded condition
            if let Some(move_data) = self.dex.moves().get(condition_id) {
                if move_data.condition.is_some() {
                    eprintln!("[CONDITION_HAS_CALLBACK] Found as move with embedded condition");
                    // For move-embedded conditions, we need to check if the move_callbacks dispatcher
                    // has this callback. For now, we'll conservatively return true for known events
                    // that move conditions typically handle.
                    // The dispatchers will return Continue if the callback doesn't exist.
                    //
                    // NOTE: onBeforeTurn and onBeforeMove are for Pokemon volatiles, not pseudoweather
                    // Pseudoweather uses onFieldStart, onFieldRestart, onFieldResidual, onFieldEnd
                    match event_id {
                        "onStart" | "onTryHit" | "onTryPrimaryHit" | "onHit" | "onEnd" |
                        "onSourceModifyDamage" | "onDisableMove" |
                        "onAnyInvulnerability" | "onInvulnerability" | "Invulnerability" => {
                            eprintln!("[CONDITION_HAS_CALLBACK] Returning true for known move-condition event");
                            return true;
                        },
                        _ => {}
                    }
                }
            }

            eprintln!("[CONDITION_HAS_CALLBACK] Returning false - not found");
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
