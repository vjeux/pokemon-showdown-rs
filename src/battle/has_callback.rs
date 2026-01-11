// NOTE: This method is NOT in JavaScript - Rust-specific implementation

use crate::*;

impl Battle {

    // ============================================================================
    // SPECIALIZED CALLBACK CHECKERS
    // Each function knows exactly what type of effect it's checking, avoiding
    // the ambiguity of the old generic has_callback() that guessed based on dex lookups.
    // ============================================================================

    /// Check if a status condition has a callback for an event
    /// Status conditions: slp, psn, tox, brn, par, frz
    pub fn has_status_callback(&self, status_id: &ID, event_id: &str) -> bool {
        if status_id.is_empty() {
            return false;
        }
        eprintln!("[HAS_STATUS_CALLBACK] status_id={}, event_id={}", status_id.as_str(), event_id);
        self.condition_has_callback(status_id.as_str(), event_id)
    }

    /// Check if a volatile condition has a callback for an event
    /// Volatiles: confusion, substitute, taunt, encore, etc.
    /// Also handles ability-embedded conditions (flashfire, etc.)
    pub fn has_volatile_callback(&self, volatile_id: &ID, event_id: &str) -> bool {
        if volatile_id.is_empty() {
            return false;
        }
        let volatile_str = volatile_id.as_str();
        eprintln!("[HAS_VOLATILE_CALLBACK] volatile_id={}, event_id={}", volatile_str, event_id);

        // First check conditions dex
        if self.condition_has_callback(volatile_str, event_id) {
            return true;
        }

        // Check ability-embedded conditions
        // Some abilities define a `condition` block with their own callbacks
        // When a volatile is added with the ability's ID (e.g., "flashfire"),
        // its callbacks are in ability.condition, not in conditions.json
        if let Some(ability_data) = self.dex.abilities().get(volatile_str) {
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

        false
    }

    /// Check if a slot condition has a callback for an event
    /// Slot conditions: healingwish, lunardance, wish, etc.
    pub fn has_slot_condition_callback(&self, slot_cond_id: &ID, event_id: &str) -> bool {
        if slot_cond_id.is_empty() {
            return false;
        }
        eprintln!("[HAS_SLOT_CONDITION_CALLBACK] slot_cond_id={}, event_id={}", slot_cond_id.as_str(), event_id);
        self.condition_has_callback(slot_cond_id.as_str(), event_id)
    }

    /// Check if a side condition has a callback for an event
    /// Side conditions: spikes, stealthrock, reflect, lightscreen, etc.
    pub fn has_side_condition_callback(&self, side_cond_id: &ID, event_id: &str) -> bool {
        if side_cond_id.is_empty() {
            return false;
        }
        let side_cond_str = side_cond_id.as_str();
        eprintln!("[HAS_SIDE_CONDITION_CALLBACK] side_cond_id={}, event_id={}", side_cond_str, event_id);

        // Check conditions dex first
        if self.condition_has_callback(side_cond_str, event_id) {
            return true;
        }

        // Side conditions can also be move-embedded (like gmaxvolcalith)
        // For these, we also need to check the move's condition callbacks
        if let Some(move_data) = self.dex.moves().get(side_cond_str) {
            if let Some(ref condition_data) = move_data.condition {
                if condition_data.extra.contains_key(event_id) {
                    return true;
                }
                // Try with "on" prefix
                if !event_id.starts_with("on") {
                    let with_on = format!("on{}", event_id);
                    if condition_data.extra.contains_key(&with_on) {
                        return true;
                    }
                }
            }
        }

        false
    }

    /// Check if a weather condition has a callback for an event
    /// Weather: sunnyday, raindance, sandstorm, hail, snow
    pub fn has_weather_callback(&self, weather_id: &ID, event_id: &str) -> bool {
        if weather_id.is_empty() {
            return false;
        }
        eprintln!("[HAS_WEATHER_CALLBACK] weather_id={}, event_id={}", weather_id.as_str(), event_id);
        self.condition_has_callback(weather_id.as_str(), event_id)
    }

    /// Check if a terrain condition has a callback for an event
    /// Terrain: electricterrain, grassyterrain, mistyterrain, psychicterrain
    pub fn has_terrain_callback(&self, terrain_id: &ID, event_id: &str) -> bool {
        if terrain_id.is_empty() {
            return false;
        }
        eprintln!("[HAS_TERRAIN_CALLBACK] terrain_id={}, event_id={}", terrain_id.as_str(), event_id);
        self.condition_has_callback(terrain_id.as_str(), event_id)
    }

    /// Check if a pseudo-weather condition has a callback for an event
    /// Pseudo-weather: trickroom, gravity, magicroom, wonderroom
    pub fn has_pseudo_weather_callback(&self, pw_id: &ID, event_id: &str) -> bool {
        if pw_id.is_empty() {
            return false;
        }
        eprintln!("[HAS_PSEUDO_WEATHER_CALLBACK] pw_id={}, event_id={}", pw_id.as_str(), event_id);
        self.condition_has_callback(pw_id.as_str(), event_id)
    }

    /// Check if an item has a callback for an event (ID version)
    /// Wrapper for item_has_callback that takes &ID for consistency
    pub fn has_item_id_callback(&self, item_id: &ID, event_id: &str) -> bool {
        if item_id.is_empty() {
            return false;
        }
        eprintln!("[HAS_ITEM_ID_CALLBACK] item_id={}, event_id={}", item_id.as_str(), event_id);
        self.item_has_callback(item_id.as_str(), event_id)
    }

    /// Check if a species has a callback for an event (ID version)
    /// Wrapper for species_has_callback that takes &ID for consistency
    pub fn has_species_id_callback(&self, species_id: &ID, event_id: &str) -> bool {
        if species_id.is_empty() {
            return false;
        }
        eprintln!("[HAS_SPECIES_ID_CALLBACK] species_id={}, event_id={}", species_id.as_str(), event_id);
        self.species_has_callback(species_id.as_str(), event_id)
    }

    /// Check if a move has a callback for an event (ID version)
    /// Wrapper for move_has_callback that takes &ID for consistency
    // TODO: Verify move parameter type matches JavaScript's ActiveMove usage
    pub fn has_move_id_callback(&self, move_id: &ID, event_id: &str) -> bool {
        if move_id.is_empty() {
            return false;
        }
        eprintln!("[HAS_MOVE_ID_CALLBACK] move_id={}, event_id={}", move_id.as_str(), event_id);
        self.move_has_callback(move_id.as_str(), event_id)
    }

    /// Check if an ability has a callback for an event (ID version)
    /// Wrapper for ability_has_callback that takes &ID for consistency
    pub fn has_ability_id_callback(&self, ability_id: &ID, event_id: &str) -> bool {
        if ability_id.is_empty() {
            return false;
        }
        eprintln!("[HAS_ABILITY_ID_CALLBACK] ability_id={}, event_id={}", ability_id.as_str(), event_id);
        self.ability_has_callback(ability_id.as_str(), event_id)
    }

    /// Check if an effect has a callback based on its effect type
    /// This is for cases where the effect type is known at runtime (e.g., from handler.effect_type)
    pub fn has_callback_for_effect_type(&self, effect_id: &ID, event_id: &str, effect_type: &super::EffectType) -> bool {
        match effect_type {
            super::EffectType::Status => self.has_status_callback(effect_id, event_id),
            super::EffectType::Condition => self.has_volatile_callback(effect_id, event_id),
            super::EffectType::Ability => self.has_ability_id_callback(effect_id, event_id),
            super::EffectType::Item => self.has_item_id_callback(effect_id, event_id),
            super::EffectType::Move => self.has_move_id_callback(effect_id, event_id),
            super::EffectType::MoveSelf => self.move_has_self_callback(effect_id.as_str(), event_id),
            super::EffectType::Weather => self.has_weather_callback(effect_id, event_id),
            super::EffectType::Terrain => self.has_terrain_callback(effect_id, event_id),
            super::EffectType::SideCondition => self.has_side_condition_callback(effect_id, event_id),
            super::EffectType::SlotCondition => self.has_slot_condition_callback(effect_id, event_id),
            // These types don't have callbacks or are not expected in this context
            super::EffectType::ZMove
            | super::EffectType::FieldCondition
            | super::EffectType::Format
            | super::EffectType::Rule
            | super::EffectType::Ruleset => false,
        }
    }

    // ============================================================================
    // INTERNAL HELPER FUNCTIONS
    // These are the underlying implementations used by the specialized functions.
    // ============================================================================
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

        // Look up the ability in dex data and check its extra field for callback
        // IMPORTANT: In JavaScript, callback !== undefined means any defined value
        // For abilities like Battle Armor with "onCriticalHit": false, the value exists
        // so there IS a callback (it just returns false to prevent crits)
        if let Some(ability_data) = self.dex.abilities().get(ability_id) {
            // Check if the event_id key exists in extra - if the key exists, there's a callback
            // The value (true/false) is the callback's return value, not whether the callback exists
            let has_callback = ability_data.extra.contains_key(event_id);

            if has_callback {
                eprintln!("[ABILITY_HAS_CALLBACK] FOUND callback for ability_id={}, event_id={}", ability_id, event_id);
                true
            } else if !event_id.starts_with("on") {
                // Try with "on" prefix for backward compatibility
                let with_on = format!("on{}", event_id);
                ability_data.extra.contains_key(&with_on)
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

        // Look up the item in dex data and check its extra field for callback
        // In JavaScript, callbacks can be:
        // - true (boolean indicating callback exists)
        // - false (static value that returns false, e.g., onNegateImmunity: false)
        // - a function (actual callback)
        // - a static value like -0.1 (for onFractionalPriority)
        // In Rust, we check for boolean (true OR false) OR any number (static return value)
        // The KEY existing is what matters - even if the value is false, the callback exists!
        if let Some(item_data) = self.dex.items().get(item_id) {
            // Check the exact event_id first, then try with "on" prefix for backward compatibility
            // IMPORTANT: Check if the key EXISTS and is a bool/number, not just if it's true
            let has_callback = item_data.extra.get(event_id)
                .map(|v| v.is_boolean() || v.as_f64().is_some() || v.as_i64().is_some())
                .unwrap_or(false);

            if has_callback {
                true
            } else if !event_id.starts_with("on") {
                // Try with "on" prefix for backward compatibility
                let with_on = format!("on{}", event_id);
                item_data.extra.get(&with_on)
                    .map(|v| v.is_boolean() || v.as_f64().is_some() || v.as_i64().is_some())
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
    // TODO: Verify move parameter type matches JavaScript's ActiveMove usage
    pub fn move_has_callback(&self, move_id: &str, event_id: &str) -> bool {
        // Look up the move in dex data and check its extra field for callback boolean
        if let Some(move_data) = self.dex.moves().get(move_id) {
            // Check the move's direct callbacks (not self callbacks)
            // IMPORTANT: Check if the key EXISTS and is a boolean, not just if it's true
            // In JavaScript, callback !== undefined means any defined value (true OR false)
            // indicates a handler exists. The value is the callback's return value.
            let has_callback = move_data.extra.get(event_id)
                .map(|v| v.is_boolean())
                .unwrap_or(false);

            if has_callback {
                return true;
            }

            // Try with "on" prefix for backward compatibility
            if !event_id.starts_with("on") {
                let with_on = format!("on{}", event_id);
                let has_callback_with_on = move_data.extra.get(&with_on)
                    .map(|v| v.is_boolean())
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
    // TODO: Verify move parameter type matches JavaScript's ActiveMove usage
    pub fn move_has_self_callback(&self, move_id: &str, event_id: &str) -> bool {
        eprintln!("[MOVE_HAS_SELF_CALLBACK] move_id={}, event_id={}", move_id, event_id);
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

            eprintln!("[MOVE_HAS_SELF_CALLBACK] Checking for key: {}", self_key);
            if let Some(has_self_callback) = move_data.extra.get(&self_key).and_then(|v| v.as_bool()) {
                eprintln!("[MOVE_HAS_SELF_CALLBACK] Found! value={}", has_self_callback);
                if has_self_callback {
                    return true;
                }
            } else {
                eprintln!("[MOVE_HAS_SELF_CALLBACK] Not found in extra fields");
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
            eprintln!("[CONDITION_HAS_CALLBACK] condition_data.extra keys: {:?}", condition_data.extra.keys().collect::<Vec<_>>());
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
                eprintln!("[CONDITION_HAS_CALLBACK] Found match: {} (has_key={}, has_priority_key={})", event_id, has_key, has_priority_key);
                true
            } else if !event_id.starts_with("on") {
                // Try with "on" prefix for backward compatibility
                let with_on = format!("on{}", event_id);
                let priority_with_on = format!("on{}Priority", event_id);
                let has_key_with_on = condition_data.extra.contains_key(&with_on);
                let has_priority_with_on = condition_data.extra.contains_key(&priority_with_on);
                eprintln!("[CONDITION_HAS_CALLBACK] Tried with 'on' prefix: {}={}, {}Priority={}", with_on, has_key_with_on, with_on, has_priority_with_on);
                has_key_with_on || has_priority_with_on
            } else {
                eprintln!("[CONDITION_HAS_CALLBACK] No match found in condition data, checking if move-embedded");
                // NOTE: onSideResidual and onResidual are DIFFERENT callbacks:
                // - onSideResidual: called once per side, no Pokemon target
                // - onResidual: called once per Pokemon on the side
                // Do NOT treat onResidual as a fallback for onSideResidual.
                // The Pokemon-targeted onResidual handlers are found via
                // find_side_event_handlers with custom_holder in field_event.

                // NOTE: onResidual and onFieldResidual are DIFFERENT callbacks:
                // - onFieldResidual: called once per turn for the field (no target)
                // - onResidual: called once per active Pokemon (target = Pokemon)
                // These should NOT be conflated. If a terrain has onResidual but not
                // onFieldResidual, it should NOT match onFieldResidual requests.
                // The per-Pokemon onResidual is handled via findFieldEventHandlers(callbackName, customHolder).

                // Before returning false, check if this is a move-embedded condition
                // Even though the condition exists in the dex, the callback might be in the move dispatcher
                if let Some(move_data) = self.dex.moves().get(condition_id) {
                    if let Some(ref condition_data) = move_data.condition {
                        eprintln!("[CONDITION_HAS_CALLBACK] Found as move with embedded condition (from dex branch), checking condition callbacks");
                        eprintln!("[CONDITION_HAS_CALLBACK] condition_data.extra keys: {:?}, looking for event_id={}", condition_data.extra.keys().collect::<Vec<_>>(), event_id);
                        // Check if the key exists in the condition data (not just if it's true)
                        // This was populated by scripts/update-move-callbacks.js
                        if condition_data.extra.contains_key(event_id) {
                            eprintln!("[CONDITION_HAS_CALLBACK] Found {} in condition.extra, returning true", event_id);
                            return true;
                        }

                        // Try with "on" prefix for backward compatibility
                        // Move-embedded conditions like shadowforce have "onInvulnerability" not "Invulnerability"
                        if !event_id.starts_with("on") {
                            let with_on = format!("on{}", event_id);
                            if condition_data.extra.contains_key(&with_on) {
                                eprintln!("[CONDITION_HAS_CALLBACK] Found {} in condition.extra, returning true", with_on);
                                return true;
                            }
                        }

                        // NOTE: Do NOT fallback from onSideResidual to onResidual for embedded conditions.
                        // They are different callbacks (per-side vs per-Pokemon).
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
                if let Some(ref condition_data) = move_data.condition {
                    eprintln!("[CONDITION_HAS_CALLBACK] Found as move with embedded condition, checking condition callbacks");
                    // Check if the key exists in the condition data (not just if it's true)
                    // This was populated by scripts/update-move-callbacks.js
                    if condition_data.extra.contains_key(event_id) {
                        eprintln!("[CONDITION_HAS_CALLBACK] Found {} in condition.extra, returning true", event_id);
                        return true;
                    }

                    // Try with "on" prefix for backward compatibility
                    // Move-embedded conditions like shadowforce have "onInvulnerability" not "Invulnerability"
                    if !event_id.starts_with("on") {
                        let with_on = format!("on{}", event_id);
                        if condition_data.extra.contains_key(&with_on) {
                            eprintln!("[CONDITION_HAS_CALLBACK] Found {} in condition.extra, returning true", with_on);
                            return true;
                        }
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
