//! Event Listener

use crate::event_system::EffectState;
use super::Effect;

/// Event listener - matches JavaScript EventListener interface
/// JavaScript: interface EventListener extends EventListenerWithoutPriority
#[derive(Clone)]
/// JavaScript equivalent: EventListener (sim/battle.ts)
pub struct EventListener {
    /// Effect that owns this handler
    /// JavaScript: effect: Effect
    pub effect: Effect,
    /// Target Pokemon (optional)
    /// JavaScript: target?: Pokemon
    pub target: Option<(usize, usize)>,
    /// Index for multi-target events
    /// JavaScript: index?: number
    pub index: Option<usize>,
    /// Callback name for static dispatch
    /// JavaScript: callback?: Function - in JS this stores the function, in Rust we store the name
    /// This is the event variant name (e.g., "Damage", "AllyDamage", "FoeDamage")
    pub callback_name: String,
    /// Effect state
    /// JavaScript: state: EffectState | null
    pub state: Option<EffectState>,
    /// Effect holder (Pokemon/Side/Field/Battle)
    /// JavaScript: effectHolder: Pokemon | Side | Field | Battle
    pub effect_holder: Option<(usize, usize)>,
    /// Order value (false = first in JS, represented as Option<i32>)
    /// JavaScript: order: number | false
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
