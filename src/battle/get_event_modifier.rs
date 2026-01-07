// NOTE: This method is NOT in JavaScript - Rust-specific implementation

use crate::*;

impl Battle {

    /// Get event modifier (4096 = 1.0x)
    /// Rust convenience method - JavaScript accesses this.event.modifier directly
    /// This method provides safe access to current event's modifier value
    /// Returns 4096 (1.0x) if no event is active
    pub fn get_event_modifier(&self) -> i32 {
        self.event
            .as_ref()
            .map(|e| e.modifier)
            .unwrap_or(4096)
    }
}
