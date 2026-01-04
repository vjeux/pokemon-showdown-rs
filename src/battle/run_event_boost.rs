// NOTE: This method extends run_event to support boost modification
//
// Used for events that modify Pokemon boosts (stat stages)
// Example: ModifyBoost event - runEvent('ModifyBoost', pokemon, null, null, boosts)

use crate::*;
use crate::battle::EventInfo;
use crate::dex_data::BoostsTable;

impl Battle {

    /// Run event with boost modification support
    /// Used for events that need to modify a BoostsTable
    /// Examples: ModifyBoost, TryBoost events
    ///
    /// JavaScript equivalent: boosts = this.runEvent('ModifyBoost', target, source, effect, boosts)
    ///
    /// The boosts are passed via relay_var_boost and callbacks can modify them in place.
    /// The modified boosts are returned.
    pub fn run_event_boost(
        &mut self,
        event_id: &str,
        target: Option<(usize, usize)>,
        source: Option<(usize, usize)>,
        source_effect: Option<&ID>,
        boosts: BoostsTable,
    ) -> BoostsTable {
        use crate::event::EventResult;

        // Check stack depth
        if self.event_depth >= 8 {
            self.add("message", &["STACK LIMIT EXCEEDED".into()]);
            return boosts;
        }

        // Save parent event context
        let parent_event = self.current_event.take();
        self.event_depth += 1;

        // Set up current event with boost relay variable
        self.current_event = Some(EventInfo {
            id: event_id.to_string(),
            target,
            source,
            effect: source_effect.cloned(),
            modifier: 4096,
            relay_var: None,
            relay_var_float: None,
            relay_var_boost: Some(boosts),
            relay_var_secondaries: None,
            relay_var_type: None,
        });

        // Find and run all handlers for this event
        let handlers = self.find_event_handlers(event_id, target, source);

        for handler in handlers {
            let event_result =
                self.dispatch_single_event(&handler.event_name, &handler.effect_id, handler.effect_holder, source);

            match event_result {
                EventResult::Stop | EventResult::Null => {
                    // Stop processing early
                    break;
                }
                _ => {
                    // Continue with next handler
                }
            }
        }

        // Extract the modified boosts from the event
        let modified_boosts = self.current_event.as_ref()
            .and_then(|e| e.relay_var_boost)
            .unwrap_or(boosts);

        // Restore parent context
        self.event_depth -= 1;
        self.current_event = parent_event;

        modified_boosts
    }
}
