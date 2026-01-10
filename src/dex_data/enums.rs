//! Effect and game type enums

use serde::{Deserialize, Serialize};

/// Effect type enumeration
/// JavaScript equivalent: EffectType (sim/global-types.ts)
/// JavaScript uses string literals for effect types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum EffectType {
    #[default]
    Condition,
    Pokemon,
    Move,
    Item,
    Ability,
    Format,
    Nature,
    Ruleset,
    Weather,
    Status,
    Terrain,
    Rule,
    ValidatorRule,
}

/// Nonstandard classification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
/// JavaScript equivalent: Nonstandard (sim/global-types.ts)
pub enum Nonstandard {
    Past,
    Future,
    Unobtainable,
    CAP,
    LGPE,
    Custom,
    Gigantamax,
}

/// Game type
/// JavaScript equivalent: GameType (sim/global-types.ts)
/// JavaScript: 'singles' | 'doubles' | 'triples' | 'rotation' | 'multi' | 'freeforall'
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum GameType {
    #[default]
    Singles,
    Doubles,
    Triples,
    Rotation,
    Multi,
    FreeForAll,
}

/// Side ID
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
/// JavaScript equivalent: SideID (sim/global-types.ts)
pub enum SideID {
    P1,
    P2,
    P3,
    P4,
}

impl SideID {
    pub fn parse(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "p1" => Some(SideID::P1),
            "p2" => Some(SideID::P2),
            "p3" => Some(SideID::P3),
            "p4" => Some(SideID::P4),
            _ => None,
        }
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            SideID::P1 => "p1",
            SideID::P2 => "p2",
            SideID::P3 => "p3",
            SideID::P4 => "p4",
        }
    }

    pub fn index(&self) -> usize {
        match self {
            SideID::P1 => 0,
            SideID::P2 => 1,
            SideID::P3 => 2,
            SideID::P4 => 3,
        }
    }
}

/// Move target type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
/// JavaScript equivalent: MoveTarget (sim/global-types.ts)
pub enum MoveTarget {
    #[default]
    Normal,
    Self_,
    AdjacentAlly,
    AdjacentAllyOrSelf,
    AdjacentFoe,
    AllAdjacent,
    AllAdjacentFoes,
    Allies,
    AllySide,
    AllyTeam,
    Any,
    FoeSide,
    RandomNormal,
    Scripted,
    All,
}
