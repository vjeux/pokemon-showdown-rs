// NOTE: This method extends run_event to support type relay variables

use crate::*;
use crate::battle::EventInfo;

impl Battle {

    /// Run event with type string parameter
    /// Used for events that need a type parameter but return numeric/boolean results
    /// Examples: Immunity event - runEvent('Immunity', pokemon, null, null, 'sandstorm')
    ///
    /// JavaScript equivalent: this.runEvent('Immunity', target, source, effect, typeString)
    pub fn run_event_with_type(
        &mut self,
        event_id: &str,
        target: Option<(usize, usize)>,
        source: Option<(usize, usize)>,
        source_effect: Option<&ID>,
        type_string: &str,
    ) -> Option<i32> {
        use crate::event::EventResult;

        // Check stack depth
        if self.event_depth >= 8 {
            self.add("message", &["STACK LIMIT EXCEEDED".into()]);
            return Some(1);
        }

        // Save parent event context
        let parent_event = self.current_event.take();
        self.event_depth += 1;

        // Set up current event with type relay variable
        self.current_event = Some(EventInfo {
            id: event_id.to_string(),
            target,
            source,
            effect: source_effect.cloned(),
            modifier: 4096,
            relay_var: Some(EventResult::String(type_string.to_string())),
        });

        let mut result = Some(1); // Default to true

        // Find and run all handlers for this event
        let handlers = self.find_event_handlers(event_id, target, source);

        for handler in handlers {
            let event_result =
                self.dispatch_single_event(&handler.event_name, &handler.effect_id, handler.effect_holder, source);

            match event_result {
                EventResult::Continue => {
                    // Continue with current result
                }
                EventResult::Boolean(false) => {
                    // Event returned false, set result to 0 and stop
                    result = Some(0);
                    break;
                }
                EventResult::Boolean(true) => {
                    // Event returned true, keep result as 1
                    result = Some(1);
                }
                EventResult::Number(n) => {
                    // Event returned a number
                    result = Some(n);
                    if n == 0 {
                        // Falsy result, stop processing
                        break;
                    }
                }
                EventResult::Null => {
                    // Event returned null, set to None (suppresses message) and stop
                    result = None;
                    break;
                }
                EventResult::Stop => {
                    result = Some(0);
                    break;
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
