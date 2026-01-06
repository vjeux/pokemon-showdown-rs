// NOTE: This method is NOT in JavaScript - Rust-specific implementation

use crate::*;
use crate::battle::EventInfo;

impl Battle {

    /// Run event and return String
    /// Used for events that return type strings, like 'Drive' event for Techno Blast
    /// Equivalent to TypeScript: this.runEvent('Drive', pokemon, null, move, 'Normal')
    pub fn run_event_string(
        &mut self,
        event_id: &str,
        target: Option<(usize, usize)>,
        source: Option<(usize, usize)>,
        source_effect: Option<&ID>,
        default_value: String,
    ) -> String {
        use crate::event::EventResult;

        // Check stack depth
        if self.event_depth >= 8 {
            self.add("message", &["STACK LIMIT EXCEEDED".into()]);
            return default_value;
        }

        // Save parent event context
        let parent_event = self.current_event.take();
        self.event_depth += 1;

        // Set up current event
        self.current_event = Some(EventInfo {
            id: event_id.to_string(),
            target,
            source,
            effect: source_effect.cloned(),
            modifier: 4096,
            relay_var: Some(EventResult::String(default_value.clone())),
        });

        let mut result = default_value;

        // Find and run all handlers for this event
        let handlers = self.find_event_handlers(event_id, target, source);

        for handler in handlers {
            let event_result =
                self.dispatch_single_event(&handler.event_name, &handler.effect_id, handler.effect_holder, source);

            match event_result {
                EventResult::Boolean(false) => {
                    // Event returned false, stop processing
                    break;
                }
                EventResult::Stop => {
                    break;
                }
                EventResult::String(s) => {
                    // For string events, replace the result with the returned value
                    result = s;
                }
                _ => {}
            }
        }

        // Restore parent context
        self.event_depth -= 1;
        self.current_event = parent_event;

        result
    }
}
