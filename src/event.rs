//! Event System
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Implements the runEvent infrastructure for effect dispatch.
//! Effects (Abilities, Items, Moves, Status, Volatiles) can register handlers
//! for events, and the battle dispatches to them.

use crate::dex_data::ID;
use serde::{Deserialize, Serialize};

/// Event types that can be dispatched
/// TODO: Not in JavaScript - Rust-specific enum for event type constants
/// JavaScript uses string literals for event names
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EventType {
    // Move execution events
    BeforeMove,
    TryMove,
    TryHit,
    TryImmunity,
    PrepareHit,
    Damage,
    DamagingHit,
    AfterHit,
    AfterMoveSecondarySelf,
    AfterSubDamage,

    // Stat modification events
    ModifyAtk,
    ModifyDef,
    ModifySpA,
    ModifySpD,
    ModifySpe,
    ModifyAccuracy,
    ModifyCritRatio,
    ModifyDamage,
    ModifyMove,
    BasePower,
    ModifyType,
    ModifyPriority,

    // Status events
    SetStatus,
    TrySetStatus,
    TryAddVolatile,
    AfterSetStatus,

    // Lifecycle events
    Start,
    End,
    Restart,
    Residual,
    SwitchIn,
    SwitchOut,
    BeforeSwitchIn,
    BeforeSwitchOut,

    // Immunity events
    Immunity,
    NegateImmunity,

    // Override events
    OverrideAction,
    DisableMove,
    LockMove,
    BeforeTurn,

    // Boost events
    Boost,
    TryBoost,
    AfterBoost,

    // Trapping
    TrapPokemon,
    MaybeTrapPokemon,

    // Type effectiveness
    Effectiveness,
    ModifyTypeEffectiveness,

    // Healing/damage
    TryHeal,
    AfterHeal,
    DrainDamage,

    // Flinch
    Flinch,

    // Critical hit
    CriticalHit,

    // PP
    DeductPP,

    // Hits
    ModifyHitCount,

    // Other
    Update,
    WeatherChange,
    TerrainChange,
    SourceModifyDamage,
    SourceBasePowerPriority,
    SourceModifyAccuracy,
    ModifyWeight,
    Invulnerability,
    HitField,
}

/// Effect types that can have handlers
/// TODO: Not in JavaScript - Rust-specific enum for effect type constants
/// JavaScript uses string literals for effect types ('Ability', 'Item', 'Move', etc.)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EffectType {
    Move,
    Ability,
    Item,
    Status,
    Volatile,
    Weather,
    Terrain,
    SideCondition,
    PseudoWeather,
    Format,
}

/// Result from an event handler
/// TODO: Not in JavaScript - Rust-specific enum for event handler return values
/// JavaScript event handlers return various types directly (undefined, null, number, string, boolean, etc.)
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub enum EventResult {
    /// Continue to next handler, no modification
    #[default]
    Continue,
    /// Stop event processing, event succeeded
    Stop,
    /// Stop event processing, event didn't fail (passed failure checks)
    NotFail,
    /// Return null - prevents default behavior (equivalent to TypeScript's `return null`)
    Null,
    /// Return a number value
    Number(i32),
    /// Return a float value (for fractional priorities, etc.)
    Float(f64),
    /// Return a string value
    String(String),
    /// Return a boolean value
    Boolean(bool),
    /// Return a position (side, slot)
    Position((usize, usize)),
    /// Return an ID
    ID(ID),
    /// Return a list of types
    Types(Vec<String>),
    /// Hit substitute - indicates damage was blocked by substitute (equivalent to TypeScript's HIT_SUBSTITUTE)
    HitSubstitute,
    /// Return a boost table (stat stages)
    Boost(crate::dex_data::BoostsTable),
    /// Return secondary effects
    Secondaries(Vec<crate::battle_actions::SecondaryEffect>),
}

impl EventResult {
    /// Check if the result indicates not failing
    pub fn is_not_fail(&self) -> bool {
        matches!(self, EventResult::NotFail)
    }

    /// Check if the result indicates stopping
    pub fn should_stop(&self) -> bool {
        matches!(self, EventResult::Stop | EventResult::NotFail)
    }

    /// Get the number value if present
    pub fn number(&self) -> Option<i32> {
        match self {
            EventResult::Number(n) => Some(*n),
            _ => None,
        }
    }

    /// Get the string value if present
    pub fn string(&self) -> Option<&str> {
        match self {
            EventResult::String(s) => Some(s.as_str()),
            _ => None,
        }
    }

    /// Get the boolean value if present
    pub fn boolean(&self) -> Option<bool> {
        match self {
            EventResult::Boolean(b) => Some(*b),
            _ => None,
        }
    }

    /// Get the position value if present
    pub fn position(&self) -> Option<(usize, usize)> {
        match self {
            EventResult::Position(pos) => Some(*pos),
            _ => None,
        }
    }

    /// Get the ID value if present
    pub fn id(&self) -> Option<&ID> {
        match self {
            EventResult::ID(id) => Some(id),
            _ => None,
        }
    }

    /// Get the types value if present
    pub fn types(&self) -> Option<&Vec<String>> {
        match self {
            EventResult::Types(types) => Some(types),
            _ => None,
        }
    }

    /// Get the boost table if present
    pub fn boost(&self) -> Option<crate::dex_data::BoostsTable> {
        match self {
            EventResult::Boost(b) => Some(*b),
            _ => None,
        }
    }

    /// Check if the result is truthy (for boolean-like event handling)
    /// Matches JavaScript truthiness semantics:
    /// - Boolean(false), Number(0), Null => false
    /// - Everything else => true
    pub fn is_truthy(&self) -> bool {
        match self {
            EventResult::Boolean(b) => *b,
            EventResult::Number(n) => *n != 0,
            EventResult::Null => false,
            _ => true,
        }
    }
}

/// Priority ordering for event handlers
/// TODO: Not in JavaScript - Rust-specific for managing handler priority ordering
#[derive(Debug, Clone, Default)]
pub struct HandlerPriority {
    /// Order value (false = highest priority in JS, we use Option<i32>)
    /// In JS: false = first, true = last, numbers in between
    pub order: Option<i32>,
    /// Priority value (higher = earlier)
    pub priority: i32,
    /// Sub-order for same priority
    pub sub_order: i32,
    /// Speed of the Pokemon (for speed ties)
    pub speed: i32,
}

impl HandlerPriority {
    /// Create with just priority
    pub fn with_priority(priority: i32) -> Self {
        Self {
            priority,
            ..Default::default()
        }
    }
}

/// Compare two handler priorities for sorting
/// Returns true if a should come before b
pub fn compare_priorities(
    a: &HandlerPriority,
    b: &HandlerPriority,
    trick_room: bool,
) -> std::cmp::Ordering {
    use std::cmp::Ordering;

    // First compare order (None = highest priority, like JS false)
    match (a.order, b.order) {
        (None, Some(_)) => return Ordering::Less,
        (Some(_), None) => return Ordering::Greater,
        (Some(ao), Some(bo)) => {
            if ao != bo {
                return ao.cmp(&bo);
            }
        }
        (None, None) => {}
    }

    // Then priority (higher = earlier)
    if a.priority != b.priority {
        return b.priority.cmp(&a.priority);
    }

    // Then sub_order
    if a.sub_order != b.sub_order {
        return a.sub_order.cmp(&b.sub_order);
    }

    // Finally speed (higher = earlier, unless Trick Room)
    if trick_room {
        a.speed.cmp(&b.speed)
    } else {
        b.speed.cmp(&a.speed)
    }
}

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

/// Flags that can be set on abilities
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
/// JavaScript equivalent: AbilityFlags (sim/dex-abilities.ts)
/// 8 fields in JavaScript
pub struct AbilityFlags {
    /// Can be suppressed by Mold Breaker etc.
    /// JavaScript: breakable?: 1
    pub breakable: bool,
    /// Cannot be copied by Role Play, etc.
    /// JavaScript: noCopy?: 1
    pub no_copy: bool,
    /// Cannot be suppressed
    /// JavaScript: cannotSuppress?: 1
    pub cannot_suppress: bool,
    /// Cannot be traced
    /// JavaScript: noTrace?: 1
    pub no_trace: bool,
    /// Cannot be skill swapped
    /// JavaScript: noSkillSwap?: 1
    pub no_skill_swap: bool,
    /// Fails Entrainment
    /// JavaScript: failEntrain?: 1
    pub fail_entrainment: bool,
}

/// Flags that can be set on moves
/// JavaScript equivalent: MoveFlags (sim/dex-moves.ts)
/// 27 fields in JavaScript
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MoveFlags {
    /// Makes contact
    pub contact: bool,
    /// Can be protected against
    pub protect: bool,
    /// Can be reflected by Magic Bounce
    pub reflectable: bool,
    /// Can be copied by Mirror Move
    pub mirror: bool,
    /// Uses sound
    pub sound: bool,
    /// Affected by Gravity
    pub gravity: bool,
    /// Is a punch move (Iron Fist)
    pub punch: bool,
    /// Is a biting move (Strong Jaw)
    pub bite: bool,
    /// Is a bullet/ball move (Bulletproof)
    pub bullet: bool,
    /// Is a slicing move
    pub slicing: bool,
    /// Is a wind move
    pub wind: bool,
    /// Is a powder move
    pub powder: bool,
    /// Bypasses Substitute
    pub bypasssub: bool,
    /// Cannot be Encored
    pub failencore: bool,
    /// High crit ratio
    pub high_crit_ratio: bool,
    /// Cannot be copied by Mimic/Sketch
    pub no_mimic: bool,
    /// Cannot be copied by Metronome
    pub no_metronome: bool,
    /// Defrosts user
    pub defrost: bool,
    /// Uses recharge
    pub recharge: bool,
    /// Is a heal move
    pub heal: bool,
    /// Is a dance move
    pub dance: bool,
    /// Can target distant Pokemon
    pub distance: bool,
    /// Is a pulse move (Mega Launcher)
    pub pulse: bool,
    /// Is a charge move (turn 1)
    pub charge: bool,
    // Stolen by Snatch
    pub snatch: bool,
    // Cannot be used in Sky Battles
    pub nonsky: bool,
}

impl MoveFlags {
    /// Check if a move makes contact
    pub fn makes_contact(&self) -> bool {
        self.contact
    }
}

/// Condition data - for status, volatiles, side conditions, etc.
/// This mirrors the JS "condition" object pattern
/// JavaScript equivalent: ConditionData (sim/dex-conditions.ts)
/// 13 fields in JavaScript
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ConditionData {
    /// Unique ID
    pub id: ID,
    /// Display name
    pub name: String,
    /// Number of turns the condition lasts (None = indefinite)
    pub duration: Option<i32>,
    /// Can this condition be passed by Baton Pass?
    pub no_copy: bool,
    /// Counter used by some conditions (e.g., Perish Song)
    pub counter_max: Option<i32>,
    /// Status this condition applies (e.g., "slp" for Rest)
    pub status: Option<String>,
    /// Residual order (when to run onResidual)
    pub residual_order: Option<i32>,
    /// Volatile status to add
    pub volatile_status: Option<String>,
    /// Side condition to add
    pub side_condition: Option<String>,
    /// Slot condition to add
    pub slot_condition: Option<String>,
    /// Weather to set
    pub weather: Option<String>,
    /// Terrain to set
    pub terrain: Option<String>,
    /// Pseudo-weather to set
    pub pseudo_weather: Option<String>,
}

impl ConditionData {
    pub fn new(id: &str) -> Self {
        Self {
            id: ID::new(id),
            name: id.to_string(),
            ..Default::default()
        }
    }

    pub fn with_duration(mut self, duration: i32) -> Self {
        self.duration = Some(duration);
        self
    }
}

/// Categories of type immunity
/// TODO: Not in JavaScript - Rust-specific enum for categorizing immunity types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ImmunityType {
    /// Type-based immunity (e.g., Ground moves on Flying types)
    Type(String),
    /// Status immunity (e.g., Electric can't be paralyzed)
    Status(String),
    /// Ability-granted immunity (e.g., Levitate)
    Ability(String),
    /// Weather immunity (e.g., Sand immunity for Rock/Ground/Steel)
    Weather(String),
    /// Terrain immunity (e.g., Psychic Terrain blocks priority)
    Terrain(String),
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_priority_comparison() {
        let high = HandlerPriority {
            priority: 10,
            ..Default::default()
        };
        let low = HandlerPriority {
            priority: 5,
            ..Default::default()
        };

        assert_eq!(
            compare_priorities(&high, &low, false),
            std::cmp::Ordering::Less
        );
    }

    #[test]
    fn test_event_result() {
        let result = EventResult::Number(150);
        assert_eq!(result.number(), Some(150));
        assert!(!result.is_not_fail());

        let string_result = EventResult::String("test".to_string());
        assert_eq!(string_result.string(), Some("test"));

        let bool_result = EventResult::Boolean(true);
        assert_eq!(bool_result.boolean(), Some(true));

        let not_fail = EventResult::NotFail;
        assert!(not_fail.is_not_fail());
        assert!(not_fail.should_stop());
    }
}
