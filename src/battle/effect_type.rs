//! Effect Type

use serde::{Deserialize, Serialize};

/// Effect type - matches JavaScript effectType
/// Used to determine event handler priority ordering
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
/// JavaScript equivalent: EffectType (sim/global-types.ts)
pub enum EffectType {
    ZMove,
    #[default]
    Condition,
    SlotCondition,
    SideCondition,
    FieldCondition,
    Weather,
    Terrain,
    Format,
    Rule,
    Ruleset,
    Ability,
    Item,
    Move,
    /// Represents moveData.self in JavaScript - the self-targeting callbacks
    MoveSelf,
    Status,
    /// Represents a Pokemon species (used as effect source in some callbacks)
    Species,
}
