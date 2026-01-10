//! Nature data and DexNatures

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::{BoostID, ID, StatID};

/// Nature data
#[derive(Debug, Clone, Serialize, Deserialize)]
/// JavaScript equivalent: Nature (sim/global-types.ts)
pub struct Nature {
    /// JavaScript: id: ID
    pub id: ID,
    /// JavaScript: name: string
    pub name: String,
    /// JavaScript: plus?: StatIDExceptHP
    pub plus: Option<StatID>,
    /// JavaScript: minus?: StatIDExceptHP
    pub minus: Option<StatID>,
    /// JavaScript: gen: number
    pub gen: u8,
}

impl Nature {
    pub fn new(name: &str, plus: Option<StatID>, minus: Option<StatID>) -> Self {
        Self {
            id: ID::new(name),
            name: name.to_string(),
            plus,
            minus,
            gen: 3,
        }
    }

    /// Get the stat modifier for a given stat (1.0, 1.1, or 0.9)
    pub fn stat_modifier(&self, stat: StatID) -> f64 {
        if stat == StatID::HP {
            return 1.0;
        }
        let stat_boost = match stat {
            StatID::Atk => BoostID::Atk,
            StatID::Def => BoostID::Def,
            StatID::SpA => BoostID::SpA,
            StatID::SpD => BoostID::SpD,
            StatID::Spe => BoostID::Spe,
            _ => return 1.0,
        };
        let plus_match = self.plus.map(|s| match s {
            StatID::Atk => BoostID::Atk,
            StatID::Def => BoostID::Def,
            StatID::SpA => BoostID::SpA,
            StatID::SpD => BoostID::SpD,
            StatID::Spe => BoostID::Spe,
            _ => BoostID::Atk,
        });
        let minus_match = self.minus.map(|s| match s {
            StatID::Atk => BoostID::Atk,
            StatID::Def => BoostID::Def,
            StatID::SpA => BoostID::SpA,
            StatID::SpD => BoostID::SpD,
            StatID::Spe => BoostID::Spe,
            _ => BoostID::Atk,
        });

        if plus_match == Some(stat_boost) {
            1.1
        } else if minus_match == Some(stat_boost) {
            0.9
        } else {
            1.0
        }
    }
}

impl std::fmt::Display for Nature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

/// Natures collection with caching
/// JavaScript equivalent: DexNatures (sim/dex-data.ts)
/// Fields: dex, allCache
pub struct DexNatures {
    gen: u8,
    nature_cache: HashMap<String, Nature>,
    all_cache: Option<Vec<Nature>>,
}

impl DexNatures {
    /// Create a new DexNatures for a given generation
    pub fn new(gen: u8) -> Self {
        Self {
            gen,
            nature_cache: HashMap::new(),
            all_cache: None,
        }
    }

    /// Get a nature by name or ID
    pub fn get(&mut self, name: &str) -> Nature {
        self.get_by_id(&super::to_id(name))
    }

    /// Get a nature by ID
    pub fn get_by_id(&mut self, id: &str) -> Nature {
        if id.is_empty() {
            return Nature {
                id: ID::empty(),
                name: String::new(),
                plus: None,
                minus: None,
                gen: 0,
            };
        }

        if let Some(nature) = self.nature_cache.get(id) {
            return nature.clone();
        }

        // Look up in nature data
        if let Some(nature) = get_nature_data(id) {
            let n = nature.clone();
            if n.gen > self.gen {
                // Mark as future if not available in this gen
            }
            self.nature_cache.insert(id.to_string(), n.clone());
            return n;
        }

        // Return non-existent nature
        Nature {
            id: ID::new(id),
            name: id.to_string(),
            plus: None,
            minus: None,
            gen: 0,
        }
    }

    /// Get all natures
    pub fn all(&mut self) -> Vec<Nature> {
        if let Some(ref all) = self.all_cache {
            return all.clone();
        }

        let natures = get_all_natures();
        self.all_cache = Some(natures.clone());
        natures
    }
}

/// Built-in nature data
fn get_nature_data(id: &str) -> Option<Nature> {
    let (plus, minus) = match id {
        "adamant" => (Some(StatID::Atk), Some(StatID::SpA)),
        "bashful" => (None, None),
        "bold" => (Some(StatID::Def), Some(StatID::Atk)),
        "brave" => (Some(StatID::Atk), Some(StatID::Spe)),
        "calm" => (Some(StatID::SpD), Some(StatID::Atk)),
        "careful" => (Some(StatID::SpD), Some(StatID::SpA)),
        "docile" => (None, None),
        "gentle" => (Some(StatID::SpD), Some(StatID::Def)),
        "hardy" => (None, None),
        "hasty" => (Some(StatID::Spe), Some(StatID::Def)),
        "impish" => (Some(StatID::Def), Some(StatID::SpA)),
        "jolly" => (Some(StatID::Spe), Some(StatID::SpA)),
        "lax" => (Some(StatID::Def), Some(StatID::SpD)),
        "lonely" => (Some(StatID::Atk), Some(StatID::Def)),
        "mild" => (Some(StatID::SpA), Some(StatID::Def)),
        "modest" => (Some(StatID::SpA), Some(StatID::Atk)),
        "naive" => (Some(StatID::Spe), Some(StatID::SpD)),
        "naughty" => (Some(StatID::Atk), Some(StatID::SpD)),
        "quiet" => (Some(StatID::SpA), Some(StatID::Spe)),
        "quirky" => (None, None),
        "rash" => (Some(StatID::SpA), Some(StatID::SpD)),
        "relaxed" => (Some(StatID::Def), Some(StatID::Spe)),
        "sassy" => (Some(StatID::SpD), Some(StatID::Spe)),
        "serious" => (None, None),
        "timid" => (Some(StatID::Spe), Some(StatID::Atk)),
        _ => return None,
    };

    let name = match id {
        "adamant" => "Adamant",
        "bashful" => "Bashful",
        "bold" => "Bold",
        "brave" => "Brave",
        "calm" => "Calm",
        "careful" => "Careful",
        "docile" => "Docile",
        "gentle" => "Gentle",
        "hardy" => "Hardy",
        "hasty" => "Hasty",
        "impish" => "Impish",
        "jolly" => "Jolly",
        "lax" => "Lax",
        "lonely" => "Lonely",
        "mild" => "Mild",
        "modest" => "Modest",
        "naive" => "Naive",
        "naughty" => "Naughty",
        "quiet" => "Quiet",
        "quirky" => "Quirky",
        "rash" => "Rash",
        "relaxed" => "Relaxed",
        "sassy" => "Sassy",
        "serious" => "Serious",
        "timid" => "Timid",
        _ => return None,
    };

    Some(Nature {
        id: ID::new(id),
        name: name.to_string(),
        plus,
        minus,
        gen: 3,
    })
}

/// Get all natures
fn get_all_natures() -> Vec<Nature> {
    let nature_ids = [
        "adamant", "bashful", "bold", "brave", "calm", "careful", "docile", "gentle", "hardy",
        "hasty", "impish", "jolly", "lax", "lonely", "mild", "modest", "naive", "naughty", "quiet",
        "quirky", "rash", "relaxed", "sassy", "serious", "timid",
    ];
    nature_ids
        .iter()
        .filter_map(|id| get_nature_data(id))
        .collect()
}
