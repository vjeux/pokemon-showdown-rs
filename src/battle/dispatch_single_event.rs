use crate::*;

impl Battle {

    /// Dispatch a single event to the appropriate handler
    /// Rust helper method - JavaScript's singleEvent() calls handler functions directly
    /// This method routes events to specialized handlers based on effect type
    /// Routes to: handle_ability_event, handle_item_event, handle_move_event, handle_condition_event
    pub fn dispatch_single_event(
        &mut self,
        event_id: &str,
        effect_id: &ID,
        target: Option<(usize, usize)>,
        _source: Option<(usize, usize)>,
    ) -> crate::event::EventResult {
        use crate::event::EventResult;

        eprintln!("[DISPATCH_SINGLE_EVENT] event_id={}, effect_id={}, target={:?}", event_id, effect_id, target);

        let effect_str = effect_id.as_str();

        // Handle ability events
        if self.dex.abilities().get(effect_id.as_str()).is_some() {
            eprintln!("[DISPATCH_SINGLE_EVENT] Calling handle_ability_event");
            return self.handle_ability_event(event_id, effect_id, target);
        }

        // Handle item events
        if self.dex.items().get(effect_id.as_str()).is_some() {
            eprintln!("[DISPATCH_SINGLE_EVENT] Calling handle_item_event for item: {}", effect_id);
            return self.handle_item_event(event_id, effect_id, target);
        }

        // Handle move events
        if let Some(_move_def) = self.dex.moves().get(effect_id.as_str()) {
            eprintln!("[DISPATCH_SINGLE_EVENT] Calling handle_move_event");
            return self.handle_move_event(event_id, effect_str, target);
        }

        // Handle condition events (status, volatile, weather, terrain)
        if let Some(_condition) = crate::data::conditions::get_condition(effect_id) {
            eprintln!("[DISPATCH_SINGLE_EVENT] Calling handle_condition_event");
            return self.handle_condition_event(event_id, effect_str, target);
        }

        eprintln!("[DISPATCH_SINGLE_EVENT] No handler found, returning Continue");
        EventResult::Continue
    }
}
