//! Effect Trait

use crate::dex_data::ID;
use crate::event::EventResult;
use super::EffectType;

/// Event callback signature
/// Takes (battle, args) and returns an EventResult
pub type EventCallback = fn(&mut crate::battle::Battle) -> EventResult;

/// Represents an effect that can have event handlers
pub trait Effect {
    /// Get the effect ID
    fn id(&self) -> &ID;

    /// Get the effect type
    fn effect_type(&self) -> EffectType;

    /// Check if this effect has a breakable flag (for Mold Breaker suppression)
    fn is_breakable(&self) -> bool {
        false
    }

    /// Get event callback for a given event ID
    fn get_event_callback(&self, _event_id: &str) -> Option<EventCallback> {
        None
    }
}
