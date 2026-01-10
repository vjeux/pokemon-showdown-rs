//! Effect Type

use serde::{Deserialize, Serialize};

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
