use crate::*;

impl Battle {

    /// Run event and return boolean
    /// Rust convenience method - JavaScript runEvent() returns various types (undefined, null, number, etc.)
    /// This method checks the truthiness of the return value like JavaScript does:
    /// - falsy values (0, null/None) return false
    /// - truthy values (non-zero numbers) return true
    /// Used when callers only need to know if the event succeeded, not the actual relay value
    pub fn run_event_bool(
        &mut self,
        event_id: &str,
        target: Option<(usize, usize)>,
        source: Option<(usize, usize)>,
        source_effect: Option<&ID>,
    ) -> bool {
        let result = self.run_event(event_id, target, source, source_effect, Some(1));
        // In JavaScript, runEvent returns a value that is checked for truthiness:
        // - 0, null, undefined, false are falsy → return false
        // - non-zero numbers, true are truthy → return true
        match result {
            Some(0) => false,  // Represents false or empty value (EventResult::Boolean(false))
            None => false,     // Represents null (EventResult::NotFail)
            Some(_) => true,   // Any other number (including 1) is truthy
        }
    }
}
