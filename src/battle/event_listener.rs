//! Event Listener

use crate::dex_data::ID;
use crate::event_system::EffectState;
use super::EffectType;

/// Event listener - matches JavaScript EventListener interface
/// JavaScript: interface EventListener extends EventListenerWithoutPriority
#[derive(Clone)]
/// JavaScript equivalent: EventListener (sim/battle.ts)
/// 10 fields in JavaScript
pub struct EventListener {
    /// Event name/variant (e.g., "Damage", "AllyDamage", "SourceDamage")
    /// Not in JavaScript - Rust-specific field to track event variant
    pub event_name: String,
    /// Effect that owns this handler
    /// JavaScript: effect: BasicEffect (via effectId)
    pub effect_id: ID,
    /// Type of effect (Ability, Item, Move, Status, etc.)
    /// JavaScript: effectType: EffectType
    pub effect_type: EffectType,
    /// Target Pokemon (optional)
    /// JavaScript: target?: Pokemon
    /// TODO: Rust uses (side_idx, poke_idx) tuple instead of Pokemon reference due to ownership
    pub target: Option<(usize, usize)>,
    /// Index for multi-target events
    /// JavaScript: index?: number
    pub index: Option<usize>,
    /// Effect state
    /// JavaScript: state?: EffectState
    pub state: Option<EffectState>,
    /// Effect holder (Pokemon/Side/Field/Battle)
    /// JavaScript: effectHolder?: Pokemon
    /// TODO: Rust uses (side_idx, poke_idx) tuple instead of Pokemon reference due to ownership
    pub effect_holder: Option<(usize, usize)>,
    /// Order value (false = first in JS, represented as Option<i32>)
    /// JavaScript: order?: false | number
    /// TODO: Rust cannot represent the union type (false | number), uses Option<i32>
    pub order: Option<i32>,
    /// Priority value (higher = earlier)
    /// JavaScript: priority: number
    pub priority: i32,
    /// Sub-order for same priority
    /// JavaScript: subOrder: number
    pub sub_order: i32,
    /// Effect order (for hazards and abilities with same priority)
    /// JavaScript: effectOrder?: number
    pub effect_order: Option<i32>,
    /// Speed stat (for speed-based sorting)
    /// JavaScript: speed?: number
    pub speed: Option<f64>,
}
