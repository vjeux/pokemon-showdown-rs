use crate::*;

impl Battle {

    /// Run event and return boolean
    /// Rust convenience method - JavaScript runEvent() returns various types (undefined, null, number, etc.)
    /// This method simplifies the API by returning true if event succeeded (returned Some value), false otherwise
    /// Used when callers only need to know if the event succeeded, not the actual relay value
    pub fn run_event_bool(
        &mut self,
        event_id: &str,
        target: Option<(usize, usize)>,
        source: Option<(usize, usize)>,
        source_effect: Option<&ID>,
    ) -> bool {
        self.run_event(event_id, target, source, source_effect, Some(1))
            .is_some()
    }
}
