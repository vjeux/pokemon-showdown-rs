//! Stat types - StatID and StatsTable

use serde::{Deserialize, Serialize};

/// Stat IDs
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
/// JavaScript equivalent: StatID (sim/global-types.ts)
pub enum StatID {
    HP,
    Atk,
    Def,
    SpA,
    SpD,
    Spe,
}

impl StatID {
    pub fn all() -> &'static [StatID] {
        &[
            StatID::HP,
            StatID::Atk,
            StatID::Def,
            StatID::SpA,
            StatID::SpD,
            StatID::Spe,
        ]
    }

    pub fn except_hp() -> &'static [StatID] {
        &[
            StatID::Atk,
            StatID::Def,
            StatID::SpA,
            StatID::SpD,
            StatID::Spe,
        ]
    }

    pub fn parse(s: &str) -> Option<StatID> {
        match s.to_lowercase().as_str() {
            "hp" | "hitpoints" => Some(StatID::HP),
            "atk" | "attack" => Some(StatID::Atk),
            "def" | "defense" => Some(StatID::Def),
            "spa" | "spatk" | "specialattack" | "special" | "spc" => Some(StatID::SpA),
            "spd" | "spdef" | "specialdefense" => Some(StatID::SpD),
            "spe" | "speed" => Some(StatID::Spe),
            _ => None,
        }
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            StatID::HP => "hp",
            StatID::Atk => "atk",
            StatID::Def => "def",
            StatID::SpA => "spa",
            StatID::SpD => "spd",
            StatID::Spe => "spe",
        }
    }
}

/// Stats table (all 6 stats)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
/// JavaScript equivalent: StatsTable (sim/global-types.ts)
/// 6 fields in JavaScript
pub struct StatsTable {
    /// Hit Points
    /// JavaScript: hp: number
    pub hp: i32,
    /// Attack
    /// JavaScript: atk: number
    pub atk: i32,
    /// Defense
    /// JavaScript: def: number
    pub def: i32,
    /// Special Attack
    /// JavaScript: spa: number
    pub spa: i32,
    /// Special Defense
    /// JavaScript: spd: number
    pub spd: i32,
    /// Speed
    /// JavaScript: spe: number
    pub spe: i32,
}

impl StatsTable {
    pub fn new(hp: i32, atk: i32, def: i32, spa: i32, spd: i32, spe: i32) -> Self {
        Self {
            hp,
            atk,
            def,
            spa,
            spd,
            spe,
        }
    }

    pub fn get(&self, stat: StatID) -> i32 {
        match stat {
            StatID::HP => self.hp,
            StatID::Atk => self.atk,
            StatID::Def => self.def,
            StatID::SpA => self.spa,
            StatID::SpD => self.spd,
            StatID::Spe => self.spe,
        }
    }

    pub fn set(&mut self, stat: StatID, value: i32) {
        match stat {
            StatID::HP => self.hp = value,
            StatID::Atk => self.atk = value,
            StatID::Def => self.def = value,
            StatID::SpA => self.spa = value,
            StatID::SpD => self.spd = value,
            StatID::Spe => self.spe = value,
        }
    }
}
