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
    pub base_power: u32,
    pub accuracy: u32,
    pub pp: u32,
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

impl MoveDef {
    /// Create a placeholder MoveDef
    pub fn placeholder() -> Self {
        Self {
            id: ID::new(""),
            name: String::new(),
            num: 0,
            move_type: String::new(),
            category: MoveCategory::Status,
            base_power: 0,
            accuracy: 100,
            pp: 0,
            priority: 0,
            target: MoveTargetType::Normal,
            flags: HashMap::new(),
            force_stab: false,
            crit_ratio: 0,
            ignore_immunity_types: Vec::new(),
            type_changer_boosted: false,
            is_z: false,
            secondaries: Vec::new(),
            ignores_evasion: false,
            tracks_target: false,
            ignore_immunity: false,
            self_volatile: None,
            self_drops: None,
            self_boosts: None,
            volatile_status: None,
            recoil: None,
            prankster_boosted: false,
            multi_hit: None,
            is_max: false,
            infiltrates: false,
            ignores_ability: false,
            has_sheer_force: false,
            has_crash_damage: false,
            ohko: false,
        }
    }
}
