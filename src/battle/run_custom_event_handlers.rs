use crate::*;
use crate::battle::EventContext;

impl Battle {

    /// Call custom event handlers for a given event
    /// Returns the last non-None value returned by a handler, if any
    ///
    /// This version is SAFE - no unsafe code needed because callbacks
    /// receive EventContext instead of &mut Battle, breaking the circular reference
    pub fn run_custom_event_handlers(&mut self, event_name: &str) -> Option<i32> {
        let callback_name = format!("on{}", event_name);

        // Check if there are any custom handlers for this event
        if !self.events.contains_key(&callback_name) {
            return None;
        }

        // Get sorted indices by priority (higher priority first)
        let sorted_indices: Vec<usize> = {
            let handlers = self.events.get(&callback_name).unwrap();
            let mut indices: Vec<usize> = (0..handlers.len()).collect();
            indices.sort_by(|&a, &b| {
                let pa = handlers[a].priority;
                let pb = handlers[b].priority;
                pb.cmp(&pa).then_with(|| a.cmp(&b)) // Descending priority, stable sort
            });
            indices
        };

        // Create EventContext from current state
        // We extract this before iterating to avoid borrow checker issues
        let event_context = if let Some(ref event_info) = self.current_event {
            EventContext::from_event_info(event_name, event_info, None)
        } else {
            EventContext::minimal(event_name)
        };

        let mut last_result = None;

        // SAFE: No unsafe code needed!
        // We can borrow self.events immutably and call callbacks safely
        // because callbacks don't receive &mut Battle anymore
        let handlers = self.events.get(&callback_name).unwrap();

        for &index in &sorted_indices {
            if index >= handlers.len() {
                continue;
            }

            // Call the callback with EventContext
            // This is completely safe - no circular borrowing!
            if let Some(result) = (handlers[index].callback)(&event_context) {
                last_result = Some(result);
            }
        }

        last_result
    }
}
