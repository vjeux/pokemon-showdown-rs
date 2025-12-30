use crate::*;
use crate::battle::EventInfo;

impl Battle {

    /// Run event with float relay variable (for fractional priorities, etc.)
    /// Similar to run_event but handles f64 values instead of i32
    pub fn run_event_float(
        &mut self,
        event_id: &str,
        target: Option<(usize, usize)>,
        source: Option<(usize, usize)>,
        source_effect: Option<&ID>,
        relay_var: Option<f64>,
    ) -> Option<f64> {
        use crate::event::EventResult;

        // Check stack depth
        if self.event_depth >= 8 {
            self.add_log("message", &["STACK LIMIT EXCEEDED"]);
            return None;
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
            relay_var: None,
            relay_var_float: relay_var,
            relay_var_boost: None,
            relay_var_secondaries: None,
            relay_var_type: None,
        });

        let mut result = relay_var;

        // Find and run all handlers for this event
        let handlers = self.find_event_handlers(event_id, target, source);

        for (effect_id, holder_target) in handlers {
            let event_result =
                self.dispatch_single_event(event_id, &effect_id, holder_target, source);

            match event_result {
                EventResult::Boolean(false) => {
                    result = None;
                    break;
                }
                EventResult::Stop => {
                    break;
                }
                EventResult::Float(f) => {
                    // For float events, replace the result with the returned value
                    result = Some(f);
                }
                _ => {}
            }
        }

        // Run custom event handlers (registered via onEvent in tests)
        if let Some(custom_result) = self.run_custom_event_handlers(event_id) {
            result = Some(custom_result as f64);
        }

        // Restore parent context
        self.event_depth -= 1;
        self.current_event = parent_event;

        result
    }
}
