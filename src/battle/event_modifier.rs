use crate::*;

impl Battle {

    /// Get the current event's modifier (4096 = 1.0x)
    /// Rust accessor method - JavaScript directly accesses this.event.modifier field
    /// Used in event handlers to get the current relay variable modifier
    pub fn event_modifier(&self) -> i32 {
        self.current_event
            .as_ref()
            .map(|e| e.modifier)
            .unwrap_or(4096)
    }
}
