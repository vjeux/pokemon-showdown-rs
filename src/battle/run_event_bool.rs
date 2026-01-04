// NOTE: This method is NOT in JavaScript - Rust-specific implementation

use crate::*;
use crate::event::EventResult;

impl Battle {

    /// Run event and return boolean
    /// Rust convenience method - JavaScript runEvent() returns various types (undefined, null, number, etc.)
    /// This method checks the truthiness of the return value like JavaScript does:
    /// - falsy values (0, false, null/None) return false
    /// - truthy values (non-zero numbers, true) return true
    /// Used when callers only need to know if the event succeeded, not the actual relay value
    pub fn run_event_bool(
        &mut self,
        event_id: &str,
        target: Option<(usize, usize)>,
        source: Option<(usize, usize)>,
        source_effect: Option<&ID>,
    ) -> bool {
        let result = self.run_event(event_id, target, source, source_effect, EventResult::Number(1), false, false);
        // In JavaScript, runEvent returns a value that is checked for truthiness:
        // - 0, null, undefined, false are falsy → return false
        // - non-zero numbers, true are truthy → return true
        match result {
            EventResult::Boolean(b) => b,
            EventResult::Number(n) => n != 0,
            EventResult::Null => false,
            EventResult::Continue => true,  // undefined is truthy in this context
            EventResult::NotFail => true,
            EventResult::Stop => true,
            _ => true,
        }
    }
}
