//! Boost types - BoostID and BoostsTable

use serde::{Deserialize, Serialize};

/// Boost IDs (stats that can be boosted, plus accuracy/evasion)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
/// JavaScript equivalent: BoostID (sim/global-types.ts)
pub enum BoostID {
    Atk,
    Def,
    SpA,
    SpD,
    Spe,
    Accuracy,
    Evasion,
}

impl BoostID {
    pub fn all() -> &'static [BoostID] {
        &[
            BoostID::Atk,
            BoostID::Def,
            BoostID::SpA,
            BoostID::SpD,
            BoostID::Spe,
            BoostID::Accuracy,
            BoostID::Evasion,
        ]
    }

    pub fn stats_only() -> &'static [BoostID] {
        &[
            BoostID::Atk,
            BoostID::Def,
            BoostID::SpA,
            BoostID::SpD,
            BoostID::Spe,
        ]
    }
}

/// Boosts table
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
/// JavaScript equivalent: BoostsTable (sim/global-types.ts)
/// 7 fields in JavaScript
pub struct BoostsTable {
    /// Attack boost (-6 to +6)
    /// JavaScript: atk: number
    pub atk: i8,
    /// Defense boost (-6 to +6)
    /// JavaScript: def: number
    pub def: i8,
    /// Special Attack boost (-6 to +6)
    /// JavaScript: spa: number
    pub spa: i8,
    /// Special Defense boost (-6 to +6)
    /// JavaScript: spd: number
    pub spd: i8,
    /// Speed boost (-6 to +6)
    /// JavaScript: spe: number
    pub spe: i8,
    /// Accuracy boost (-6 to +6)
    /// JavaScript: accuracy: number
    pub accuracy: i8,
    /// Evasion boost (-6 to +6)
    /// JavaScript: evasion: number
    pub evasion: i8,
}

impl BoostsTable {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get(&self, boost: BoostID) -> i8 {
        match boost {
            BoostID::Atk => self.atk,
            BoostID::Def => self.def,
            BoostID::SpA => self.spa,
            BoostID::SpD => self.spd,
            BoostID::Spe => self.spe,
            BoostID::Accuracy => self.accuracy,
            BoostID::Evasion => self.evasion,
        }
    }

    pub fn set(&mut self, boost: BoostID, value: i8) {
        let clamped = value.clamp(-6, 6);
        match boost {
            BoostID::Atk => self.atk = clamped,
            BoostID::Def => self.def = clamped,
            BoostID::SpA => self.spa = clamped,
            BoostID::SpD => self.spd = clamped,
            BoostID::Spe => self.spe = clamped,
            BoostID::Accuracy => self.accuracy = clamped,
            BoostID::Evasion => self.evasion = clamped,
        }
    }

    pub fn boost(&mut self, boost: BoostID, amount: i8) -> i8 {
        let old = self.get(boost);
        let new = (old + amount).clamp(-6, 6);
        self.set(boost, new);
        new - old // Return actual change
    }

    pub fn clear(&mut self) {
        *self = Self::default();
    }
}
