//! Event Handler

use crate::dex_data::ID;
use super::{EffectType, EventType, HandlerPriority};

/// An event handler registered by an effect
/// TODO: Not in JavaScript - Rust-specific for managing event handlers
#[derive(Debug, Clone)]
pub struct EventHandler {
    /// The effect ID (ability name, move name, etc.)
    pub effect_id: ID,
    /// Type of effect
    pub effect_type: EffectType,
    /// Priority information
    pub priority: HandlerPriority,
    /// The event this handler responds to
    pub event: EventType,
    /// Source position (for Pokemon-specific effects)
    pub source_position: Option<usize>,
    /// Source side index (0 or 1)
    pub source_side: Option<usize>,
}
