//! Battle::run_event_side - Run event targeting a Side
//!
//! JavaScript equivalent: runEvent with Side as target parameter
//!
//! This method fires an event on a Side, allowing abilities and other effects to react.

use crate::*;
use crate::battle::EventInfo;

impl Battle {
    /// Run an event targeting a Side
    ///
    /// JavaScript equivalent:
    /// ```javascript
    /// battle.runEvent('SideConditionStart', this, source, status)
    /// ```
    ///
    /// where `this` is the Side object.
    ///
    /// This is similar to run_event() but targets a Side instead of a Pokemon.
    /// It allows abilities to react to side condition changes (e.g., Screen Cleaner clearing screens).
    ///
    /// Parameters:
    /// - event_id: Event name (e.g., "SideConditionStart")
    /// - side_idx: Index of the side being targeted
    /// - source: Optional source Pokemon position
    /// - effect: Optional effect that triggered this event
    ///
    /// Returns: Option<i32> with event result (None = no handlers changed the result)
    pub fn run_event_side(
        &mut self,
        event_id: &str,
        side_idx: usize,
        source: Option<(usize, usize)>,
        effect: Option<&ID>,
    ) -> Option<i32> {
        // For now, this is a simplified implementation
        // Full implementation would need to:
        // 1. Find all ability/item/condition handlers for this event
        // 2. Sort them by priority
        // 3. Run each handler
        // 4. Combine results

        // The main use case for SideConditionStart is abilities like Screen Cleaner, Wind Power, Wind Rider
        // that react when a side condition is added

        // Store event context for nested events
        let _parent_event = self.current_event.clone();
        let mut event_info = EventInfo::new(event_id);
        event_info.source = source;
        event_info.effect = effect.cloned();
        self.current_event = Some(event_info);

        // Find and run ability handlers
        // For each active Pokemon on all sides:
        for side_index in 0..self.sides.len() {
            let active_count = self.sides[side_index].active.len();
            for pokemon_index in 0..active_count {
                let (ability_id, fainted) = {
                    match self.pokemon_at(side_index, pokemon_index) {
                        Some(p) => (p.ability.clone(), p.fainted),
                        None => continue,
                    }
                };

                if fainted {
                    continue;
                }

                // Check if this ability has a handler for this event
                if self.has_callback(&ability_id, event_id) {
                    // Dispatch to ability callback based on event type
                    if event_id == "SideConditionStart" {
                        // Extract side_condition_id from effect parameter
                        let side_condition_id = effect.map(|id| id.as_str()).unwrap_or("");

                        use crate::data::ability_callbacks;
                        ability_callbacks::dispatch_on_side_condition_start(
                            self,
                            ability_id.as_str(),
                            (side_index, pokemon_index),
                            side_condition_id,
                            source,
                        );
                    }
                    // Add more event types here as needed (SideStart, SideEnd, etc.)
                }
            }
        }

        self.current_event = _parent_event;

        // For now, return None (no handler modified the result)
        // Full implementation would track and return modified results
        None
    }
}
