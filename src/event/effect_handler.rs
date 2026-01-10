//! Effect Handler Trait

use crate::dex_data::ID;
use super::{ConditionData, EffectType, EventHandler, EventType, HandlerPriority};
use super::handler_priority::compare_priorities;

/// Effect handler trait - implemented by abilities, moves, items, conditions
/// This is the core of the data-driven event system
pub trait EffectHandler: Send + Sync {
    /// Get the effect ID
    fn id(&self) -> &ID;

    /// Get the effect type
    fn effect_type(&self) -> EffectType;

    /// Get condition data if this effect creates a condition
    fn condition_data(&self) -> Option<&ConditionData> {
        None
    }

    /// Get handler priority for an event
    fn priority_for(&self, _event: &EventType) -> Option<HandlerPriority> {
        None
    }

    /// Check if this handler responds to an event
    fn handles_event(&self, event: &EventType) -> bool {
        self.priority_for(event).is_some()
    }
}

/// Collect handlers for an event from various sources
pub fn collect_handlers(
    handlers: &mut Vec<EventHandler>,
    effect_id: ID,
    effect_type: EffectType,
    event: EventType,
    priority: HandlerPriority,
    source_side: Option<usize>,
    source_position: Option<usize>,
) {
    handlers.push(EventHandler {
        effect_id,
        effect_type,
        priority,
        event,
        source_side,
        source_position,
    });
}

/// Sort handlers by priority
pub fn sort_handlers(handlers: &mut [EventHandler], trick_room: bool) {
    handlers.sort_by(|a, b| compare_priorities(&a.priority, &b.priority, trick_room));
}
