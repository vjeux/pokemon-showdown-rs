//! Battle action enums

use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// Ignore immunity setting for moves
/// JavaScript: ignoreImmunity?: boolean | { [k: string]: boolean }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IgnoreImmunity {
    /// Ignore all type immunities (true)
    All,
    /// Ignore specific type immunities ({ Type: true, ... })
    Specific(HashMap<String, bool>),
}

/// Fixed damage value for moves
/// JavaScript: damage?: number | 'level' | false | null
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Damage {
    /// Fixed numeric damage
    Fixed(i32),
    /// Damage equals user's level (Seismic Toss, Night Shade)
    Level,
}

/// Damage value (can be number, false, or undefined-like None)
/// JavaScript: number | false | undefined
#[derive(Debug, Clone)]
pub enum DamageValue {
    Damage(i32),
    Failed,
    HitSubstitute,
}

/// Switch copy flag type
/// JavaScript uses string literals 'copyvolatile' | 'shedtail' | true
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SwitchCopyFlag {
    None,
    CopyVolatile,
    ShedTail,
}

/// Target info for spread moves (can be the Pokemon or null/false for failed)
/// JavaScript: Pokemon | null | false
#[derive(Debug, Clone)]
pub enum SpreadMoveTarget {
    Target((usize, usize)),
    None,
    Failed,
}
