// NOTE: This method is NOT in JavaScript - Rust-specific implementation

use crate::*;

impl Battle {

    /// Call custom event handlers for a given event
    /// Returns the last non-None value returned by a handler, if any
    ///
    /// Callbacks receive EventInfo for event context
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

        // Use EventInfo from current state, or create minimal one
        let event_info = if let Some(ref event) = self.event {
            event.clone()
        } else {
            crate::battle::EventInfo::new(event_name)
        };

        let mut last_result = None;

        // Get handlers immutably and call callbacks
        let handlers = self.events.get(&callback_name).unwrap();

        for &index in &sorted_indices {
            if index >= handlers.len() {
                continue;
            }

            // Call the callback with EventInfo
            if let Some(result) = (handlers[index].callback)(&event_info) {
                last_result = Some(result);
            }
        }

        last_result
    }
}
