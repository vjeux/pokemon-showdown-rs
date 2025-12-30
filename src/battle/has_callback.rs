use crate::*;

impl Battle {

    /// Check if an effect has a callback for a specific event
    /// This is a Rust helper to replicate JavaScript's getCallback() check
    /// without actually executing the callback
    ///
    /// Returns true if the effect has a handler for the event, false otherwise
    pub fn has_callback(&self, effect_id: &ID, event_id: &str) -> bool {
        let effect_str = effect_id.as_str();

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

        // Check conditions (status, volatile, weather, terrain)
        if crate::data::conditions::get_condition(effect_id).is_some() {
            return self.condition_has_callback(effect_str, event_id);
        }

        // Check species - species can have callbacks like onSwitchIn for form changes
        if self.dex.species().get(effect_str).is_some() {
            return self.species_has_callback(effect_str, event_id);
        }

        false
    }

    /// Check if an ability has a callback for an event
    fn ability_has_callback(&self, ability_id: &str, event_id: &str) -> bool {
        // Special case: onAnySwitchIn is only for abilities that trigger when ANY Pokemon switches in
        // Examples: Intimidate, Trace, Download, etc.
        if event_id == "onAnySwitchIn" {
            // Only specific abilities have onAnySwitchIn
            return matches!(ability_id, "intimidate" | "trace" | "download" | "frisk" | "forewarn" | "anticipation");
        }

        // For other events, conservatively return false by default
        // TODO: Implement proper callback checking by consulting ability data
        // For now, this prevents collecting non-existent handlers
        false
    }

    /// Check if an item has a callback for an event
    fn item_has_callback(&self, _item_id: &str, event_id: &str) -> bool {
        // Items don't have onAnySwitchIn
        if event_id == "onAnySwitchIn" {
            return false;
        }

        // For other events, conservatively return false by default
        // TODO: Implement proper callback checking by consulting item data
        false
    }

    /// Check if a move has a callback for an event
    fn move_has_callback(&self, move_id: &str, event_id: &str) -> bool {
        // Moves don't have onAnySwitchIn
        if event_id == "onAnySwitchIn" {
            return false;
        }

        // For BasePower event, check if move is in the dispatcher
        if event_id == "BasePower" {
            return matches!(
                move_id,
                "barbbarrage"
                    | "brine"
                    | "collisioncourse"
                    | "electrodrift"
                    | "expandingforce"
                    | "facade"
                    | "ficklebeam"
                    | "fusionbolt"
                    | "fusionflare"
                    | "gravapple"
                    | "knockoff"
                    | "lashout"
                    | "mistyexplosion"
                    | "psyblade"
                    | "retaliate"
                    | "solarbeam"
                    | "solarblade"
                    | "venoshock"
            );
        }

        // For other events, conservatively return false by default
        // TODO: Implement proper callback checking for other events
        false
    }

    /// Check if a condition has a callback for an event
    fn condition_has_callback(&self, _condition_id: &str, event_id: &str) -> bool {
        // Conditions don't have onAnySwitchIn
        if event_id == "onAnySwitchIn" {
            return false;
        }

        // For other events, conservatively return false by default
        false
    }

    /// Check if a species has a callback for an event
    fn species_has_callback(&self, _species_id: &str, event_id: &str) -> bool {
        // Species don't have onAnySwitchIn
        if event_id == "onAnySwitchIn" {
            return false;
        }

        // For other events, conservatively return false by default
        false
    }
}
