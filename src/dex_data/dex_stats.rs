//! DexStats - stats helper with name lookups

use std::collections::HashMap;

use super::StatID;

/// Stats helper with name lookups
/// JavaScript equivalent: DexStats (sim/dex-data.ts)
/// 9 fields in JavaScript
pub struct DexStats {
    gen: u8,
}

impl DexStats {
    /// Create a new DexStats for a given generation
    pub fn new(gen: u8) -> Self {
        Self { gen }
    }

    /// Get short stat names
    pub fn short_names(&self) -> HashMap<StatID, &'static str> {
        let mut map = HashMap::new();
        if self.gen != 1 {
            map.insert(StatID::HP, "HP");
            map.insert(StatID::Atk, "Atk");
            map.insert(StatID::Def, "Def");
            map.insert(StatID::SpA, "SpA");
            map.insert(StatID::SpD, "SpD");
            map.insert(StatID::Spe, "Spe");
        } else {
            map.insert(StatID::HP, "HP");
            map.insert(StatID::Atk, "Atk");
            map.insert(StatID::Def, "Def");
            map.insert(StatID::SpA, "Spc");
            map.insert(StatID::SpD, "[SpD]");
            map.insert(StatID::Spe, "Spe");
        }
        map
    }

    /// Get medium stat names
    pub fn medium_names(&self) -> HashMap<StatID, &'static str> {
        let mut map = HashMap::new();
        if self.gen != 1 {
            map.insert(StatID::HP, "HP");
            map.insert(StatID::Atk, "Attack");
            map.insert(StatID::Def, "Defense");
            map.insert(StatID::SpA, "Sp. Atk");
            map.insert(StatID::SpD, "Sp. Def");
            map.insert(StatID::Spe, "Speed");
        } else {
            map.insert(StatID::HP, "HP");
            map.insert(StatID::Atk, "Attack");
            map.insert(StatID::Def, "Defense");
            map.insert(StatID::SpA, "Special");
            map.insert(StatID::SpD, "[Sp. Def]");
            map.insert(StatID::Spe, "Speed");
        }
        map
    }

    /// Get full stat names
    pub fn names(&self) -> HashMap<StatID, &'static str> {
        let mut map = HashMap::new();
        if self.gen != 1 {
            map.insert(StatID::HP, "HP");
            map.insert(StatID::Atk, "Attack");
            map.insert(StatID::Def, "Defense");
            map.insert(StatID::SpA, "Special Attack");
            map.insert(StatID::SpD, "Special Defense");
            map.insert(StatID::Spe, "Speed");
        } else {
            map.insert(StatID::HP, "HP");
            map.insert(StatID::Atk, "Attack");
            map.insert(StatID::Def, "Defense");
            map.insert(StatID::SpA, "Special");
            map.insert(StatID::SpD, "[Special Defense]");
            map.insert(StatID::Spe, "Speed");
        }
        map
    }

    /// Get stat ID from name
    pub fn get_id(&self, name: &str) -> Option<StatID> {
        // Special case: "Spd" means Speed, not Special Defense
        if name == "Spd" {
            return Some(StatID::Spe);
        }
        StatID::parse(name)
    }

    /// Get all stat IDs
    pub fn ids(&self) -> &'static [StatID] {
        StatID::all()
    }
}
