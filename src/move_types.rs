//! Move type definitions for compatibility with generated ability callbacks
//! 
//! These types are used by auto-generated ability callback code.
//! The actual move data comes from JSON via the Dex system.

use crate::dex_data::ID;
use std::collections::HashMap;

/// Move category enumeration
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MoveCategory {
    Physical,
    Special,
    Status,
}

/// Move target type enumeration  
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MoveTargetType {
    Normal,
    Self_,
    AllySide,
    AllyTeam,
    FoeSide,
    All,
    RandomNormal,
    Scripted,
    Any,
    AdjacentAlly,
    AdjacentFoe,
    AdjacentAllyOrSelf,
}

/// Move definition structure (compatibility type for callbacks)
/// Maps to MoveData from the Dex system
#[derive(Debug, Clone)]
pub struct MoveDef {
    pub id: ID,
    pub name: String,
    pub num: i32,
    pub move_type: String,
    pub category: MoveCategory,
    pub base_power: i32,
    pub accuracy: i32,
    pub pp: i32,
    pub priority: i8,
    pub target: MoveTargetType,
    pub flags: HashMap<String, bool>,
    pub force_stab: bool,
    pub crit_ratio: i32,

    // Additional stub fields for auto-generated callbacks
    pub ignore_immunity_types: Vec<String>,
    pub type_changer_boosted: bool,
    pub is_z: bool,
    pub secondaries: Vec<String>,  // Simplified from complex secondary type
    pub ignores_evasion: bool,
    pub tracks_target: bool,
    pub ignore_immunity: bool,
    pub self_volatile: Option<String>,
    pub self_drops: Option<HashMap<String, i32>>,
    pub self_boosts: Option<HashMap<String, i32>>,
    pub volatile_status: Option<String>,
    pub recoil: Option<f64>,
    pub prankster_boosted: bool,
    pub multi_hit: Option<String>,  // Simplified
    pub is_max: bool,
    pub infiltrates: bool,
    pub ignores_ability: bool,
    pub has_sheer_force: bool,
    pub has_crash_damage: bool,
    pub ohko: bool,
}
