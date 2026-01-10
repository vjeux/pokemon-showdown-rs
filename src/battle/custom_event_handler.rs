//! Custom Event Handler

use crate::dex_data::ID;
use super::{EffectType, EventInfo};

/// Type alias for event callback functions
pub type EventCallback = Box<dyn Fn(&EventInfo) -> Option<i32> + Send + Sync>;

/// Custom event handler registered via onEvent (for testing)
/// JavaScript: { callback, target, priority, order, subOrder }
/// JavaScript equivalent: CustomEventHandler (sim/battle.ts)
/// 5 fields in JavaScript
pub struct CustomEventHandler {
    /// The callback function - receives EventInfo for event context
    /// JavaScript: callback: (this: Battle, ...args: any[]) => any
    pub callback: EventCallback,
    /// Target effect ID
    /// JavaScript: target (Effect object ID)
    pub target_id: ID,
    /// Target effect type
    /// JavaScript: target.effectType
    pub target_type: EffectType,
    /// Priority for event ordering (higher = earlier)
    /// JavaScript: priority: number
    pub priority: i32,
    /// Order value
    /// JavaScript: order: boolean
    pub order: bool,
    /// Sub-order for same priority
    /// JavaScript: subOrder: number
    pub sub_order: i32,
}

impl std::fmt::Debug for CustomEventHandler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CustomEventHandler")
            .field("target_id", &self.target_id)
            .field("target_type", &self.target_type)
            .field("priority", &self.priority)
            .field("order", &self.order)
            .field("sub_order", &self.sub_order)
            .field("callback", &"<closure>")
            .finish()
    }
}
