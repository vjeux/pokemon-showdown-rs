//! Event Handler

use crate::dex_data::ID;
use super::EffectType;

/// Event handler with priority information
/// TODO: Not in JavaScript - Rust-specific struct for organizing event handlers
/// JavaScript manages handler ordering via inline sorting in runEvent
#[derive(Debug, Clone)]
pub struct EventHandler {
    /// Effect that owns this handler
    pub effect_id: ID,
    /// Type of effect
    pub effect_type: EffectType,
    /// Priority value (for move events)
    pub priority: i32,
    /// Sub-order for same priority
    pub sub_order: i32,
    /// Speed stat (for speed-based sorting)
    pub speed: i32,
    /// Pokemon that holds this effect
    pub effect_holder: Option<(usize, usize)>,
    /// Order value (for special sorting)
    pub order: i32,
}

impl EventHandler {
    pub fn new(effect_id: ID, effect_type: EffectType) -> Self {
        Self {
            effect_id,
            effect_type,
            priority: 0,
            sub_order: 0,
            speed: 0,
            effect_holder: None,
            order: 0,
        }
    }
}
