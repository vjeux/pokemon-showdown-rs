//! Event Info

use serde::{Deserialize, Serialize};
use super::Effect;

/// Event information passed through the event system
#[derive(Debug, Clone, Serialize, Deserialize)]
/// JavaScript equivalent: Event object (sim/battle.ts)
/// JavaScript stores this in battle.event during event processing
/// 10+ fields in JavaScript (context object with dynamic properties)
pub struct EventInfo {
    /// Event ID/name
    /// JavaScript: event.id (implicit, based on which event is running)
    pub id: String,
    /// Target of the event (side_idx, poke_idx) or None for field/battle
    /// JavaScript: event.target (Pokemon reference)
    /// TODO: Rust uses indices instead of Pokemon reference due to ownership
    pub target: Option<(usize, usize)>,
    /// Source of the event
    /// JavaScript: event.source (Pokemon reference)
    /// TODO: Rust uses indices instead of Pokemon reference due to ownership
    pub source: Option<(usize, usize)>,
    /// Effect that caused the event
    /// JavaScript: event.effect (Effect object)
    pub effect: Option<Effect>,
    /// Modifier accumulated during event processing (4096 = 1.0x)
    /// JavaScript: event.modifier (number)
    pub modifier: i32,
    /// Relay variable for event callbacks
    /// JavaScript: relayVar parameter in event callbacks
    /// Can hold various types via EventResult variants:
    /// - Number(i32) for numeric values (damage, crit ratio, etc.)
    /// - Float(f64) for fractional values (priority, etc.)
    /// - Boost(BoostsTable) for stat modifications
    /// - String for type strings (immunity, etc.)
    /// - Secondaries(Vec<SecondaryEffect>) for move secondary effects
    /// - Boolean for true/false checks
    /// - etc.
    pub relay_var: Option<crate::event::EventResult>,
    /// Type parameter for Effectiveness events
    /// JavaScript: type parameter in runEvent('Effectiveness', this, type, move, typeMod)
    /// Used to pass the defender type being checked for type effectiveness
    pub type_param: Option<String>,
}

impl EventInfo {
    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_string(),
            target: None,
            source: None,
            effect: None,
            modifier: 4096,
            relay_var: None,
            type_param: None,
        }
    }
}

impl Default for EventInfo {
    fn default() -> Self {
        Self {
            id: String::new(),
            target: None,
            source: None,
            effect: None,
            modifier: 4096,
            relay_var: None,
            type_param: None,
        }
    }
}
