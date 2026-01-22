//! Event Result

use crate::dex_data::ID;

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
    Secondaries(Vec<crate::dex::MoveSecondary>),
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
    /// - Boolean(false), Number(0), Null, Stop => false
    /// - Everything else => true
    pub fn is_truthy(&self) -> bool {
        match self {
            EventResult::Boolean(b) => *b,
            EventResult::Number(n) => *n != 0,
            EventResult::Null => false,
            EventResult::Stop => false,
            _ => true,
        }
    }

    /// Check if the result is strictly false (=== false in JavaScript)
    /// This is different from is_truthy() which uses falsy semantics.
    /// In JavaScript, `result === false` only matches Boolean(false), not Number(0) or other falsy values.
    pub fn is_strictly_false(&self) -> bool {
        matches!(self, EventResult::Boolean(false))
    }
}
