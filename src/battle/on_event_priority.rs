use crate::*;
use crate::battle::CustomEventHandler;
use crate::battle::EventContext;

impl Battle {

    /// Register a custom event handler with priority (for testing)
    /// JavaScript: onEvent(eventid: string, target: Format, priority: number, callback)
    ///
    /// # Arguments
    /// * `event_id` - Event name (e.g., "Hit", "ModifyDamage")
    /// * `priority` - Priority value (higher = called earlier)
    /// * `callback` - Function to call when event fires
    pub fn on_event_priority<F>(&mut self, event_id: &str, priority: i32, callback: F)
    where
        F: Fn(&EventContext) -> Option<i32> + Send + Sync + 'static,
    {
        if event_id.is_empty() {
            panic!("Event handlers must have an event to listen to");
        }

        let callback_name = format!("on{}", event_id);

        let handler = CustomEventHandler {
            callback: Box::new(callback),
            priority,
            order: false,
            sub_order: 0,
        };

        self.events.entry(callback_name).or_default().push(handler);
    }
}
