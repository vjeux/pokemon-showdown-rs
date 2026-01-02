// NOTE: This method is NOT in JavaScript - Rust-specific implementation

use crate::*;

impl Battle {

    /// Set event modifier (for chainModify pattern, 4096 = 1.0x)
    /// Rust convenience method - JavaScript sets this.event.modifier directly
    /// This method chains modifiers by multiplying: modifier = (current * new + 2048) >> 12
    /// Used by chainModify() to accumulate multiple damage/stat modifiers
    pub fn set_event_modifier(&mut self, modifier: i32) {
        if let Some(ref mut event) = self.current_event {
            // Chain modifiers by multiplying in 4096 basis points
            let current = event.modifier as i64;
            let new = modifier as i64;
            event.modifier = ((current * new + 2048) >> 12) as i32;
        }
    }
}
