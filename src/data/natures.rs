//! Data-driven Nature Definitions
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! This module contains data-driven definitions for Pokemon natures,
//! following the Pokemon Showdown JS architecture.

use std::collections::HashMap;
use once_cell::sync::Lazy;
use crate::dex_data::ID;

/// Stat that can be modified by nature
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NatureStat {
    Atk,
    Def,
    SpA,
    SpD,
    Spe,
}

impl NatureStat {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "atk" | "attack" => Some(NatureStat::Atk),
            "def" | "defense" => Some(NatureStat::Def),
            "spa" | "specialattack" | "spatk" => Some(NatureStat::SpA),
            "spd" | "specialdefense" | "spdef" => Some(NatureStat::SpD),
            "spe" | "speed" => Some(NatureStat::Spe),
            _ => None,
        }
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            NatureStat::Atk => "atk",
            NatureStat::Def => "def",
            NatureStat::SpA => "spa",
            NatureStat::SpD => "spd",
            NatureStat::Spe => "spe",
        }
    }
}

/// Nature definition
#[derive(Debug, Clone, Copy)]
pub struct NatureDef {
    /// Nature name
    pub name: &'static str,
    /// Stat increased by 10% (None for neutral natures)
    pub plus: Option<NatureStat>,
    /// Stat decreased by 10% (None for neutral natures)
    pub minus: Option<NatureStat>,
}

impl NatureDef {
    /// Get the multiplier for a given stat
    pub fn get_multiplier(&self, stat: NatureStat) -> f64 {
        if self.plus == Some(stat) {
            1.1
        } else if self.minus == Some(stat) {
            0.9
        } else {
            1.0
        }
    }

    /// Check if this nature is neutral (no stat changes)
    pub fn is_neutral(&self) -> bool {
        self.plus.is_none() && self.minus.is_none()
    }

    /// Get stat modifier as integer (-1, 0, or +1)
    pub fn get_modifier(&self, stat: NatureStat) -> i8 {
        if self.plus == Some(stat) {
            1
        } else if self.minus == Some(stat) {
            -1
        } else {
            0
        }
    }
}

/// Static natures registry
pub static NATURES: Lazy<HashMap<ID, NatureDef>> = Lazy::new(|| {
    let mut natures = HashMap::new();

    // Neutral natures (no stat changes)
    natures.insert(ID::new("hardy"), NatureDef {
        name: "Hardy",
        plus: None,
        minus: None,
    });
    natures.insert(ID::new("docile"), NatureDef {
        name: "Docile",
        plus: None,
        minus: None,
    });
    natures.insert(ID::new("serious"), NatureDef {
        name: "Serious",
        plus: None,
        minus: None,
    });
    natures.insert(ID::new("bashful"), NatureDef {
        name: "Bashful",
        plus: None,
        minus: None,
    });
    natures.insert(ID::new("quirky"), NatureDef {
        name: "Quirky",
        plus: None,
        minus: None,
    });

    // +Atk natures
    natures.insert(ID::new("lonely"), NatureDef {
        name: "Lonely",
        plus: Some(NatureStat::Atk),
        minus: Some(NatureStat::Def),
    });
    natures.insert(ID::new("brave"), NatureDef {
        name: "Brave",
        plus: Some(NatureStat::Atk),
        minus: Some(NatureStat::Spe),
    });
    natures.insert(ID::new("adamant"), NatureDef {
        name: "Adamant",
        plus: Some(NatureStat::Atk),
        minus: Some(NatureStat::SpA),
    });
    natures.insert(ID::new("naughty"), NatureDef {
        name: "Naughty",
        plus: Some(NatureStat::Atk),
        minus: Some(NatureStat::SpD),
    });

    // +Def natures
    natures.insert(ID::new("bold"), NatureDef {
        name: "Bold",
        plus: Some(NatureStat::Def),
        minus: Some(NatureStat::Atk),
    });
    natures.insert(ID::new("relaxed"), NatureDef {
        name: "Relaxed",
        plus: Some(NatureStat::Def),
        minus: Some(NatureStat::Spe),
    });
    natures.insert(ID::new("impish"), NatureDef {
        name: "Impish",
        plus: Some(NatureStat::Def),
        minus: Some(NatureStat::SpA),
    });
    natures.insert(ID::new("lax"), NatureDef {
        name: "Lax",
        plus: Some(NatureStat::Def),
        minus: Some(NatureStat::SpD),
    });

    // +SpA natures
    natures.insert(ID::new("modest"), NatureDef {
        name: "Modest",
        plus: Some(NatureStat::SpA),
        minus: Some(NatureStat::Atk),
    });
    natures.insert(ID::new("mild"), NatureDef {
        name: "Mild",
        plus: Some(NatureStat::SpA),
        minus: Some(NatureStat::Def),
    });
    natures.insert(ID::new("quiet"), NatureDef {
        name: "Quiet",
        plus: Some(NatureStat::SpA),
        minus: Some(NatureStat::Spe),
    });
    natures.insert(ID::new("rash"), NatureDef {
        name: "Rash",
        plus: Some(NatureStat::SpA),
        minus: Some(NatureStat::SpD),
    });

    // +SpD natures
    natures.insert(ID::new("calm"), NatureDef {
        name: "Calm",
        plus: Some(NatureStat::SpD),
        minus: Some(NatureStat::Atk),
    });
    natures.insert(ID::new("gentle"), NatureDef {
        name: "Gentle",
        plus: Some(NatureStat::SpD),
        minus: Some(NatureStat::Def),
    });
    natures.insert(ID::new("sassy"), NatureDef {
        name: "Sassy",
        plus: Some(NatureStat::SpD),
        minus: Some(NatureStat::Spe),
    });
    natures.insert(ID::new("careful"), NatureDef {
        name: "Careful",
        plus: Some(NatureStat::SpD),
        minus: Some(NatureStat::SpA),
    });

    // +Spe natures
    natures.insert(ID::new("timid"), NatureDef {
        name: "Timid",
        plus: Some(NatureStat::Spe),
        minus: Some(NatureStat::Atk),
    });
    natures.insert(ID::new("hasty"), NatureDef {
        name: "Hasty",
        plus: Some(NatureStat::Spe),
        minus: Some(NatureStat::Def),
    });
    natures.insert(ID::new("jolly"), NatureDef {
        name: "Jolly",
        plus: Some(NatureStat::Spe),
        minus: Some(NatureStat::SpA),
    });
    natures.insert(ID::new("naive"), NatureDef {
        name: "Naive",
        plus: Some(NatureStat::Spe),
        minus: Some(NatureStat::SpD),
    });

    natures
});

/// Get a nature definition by ID
pub fn get_nature(id: &ID) -> Option<&'static NatureDef> {
    NATURES.get(id)
}

/// Get a nature by name (case-insensitive)
pub fn get_nature_by_name(name: &str) -> Option<&'static NatureDef> {
    get_nature(&ID::new(name))
}

/// Get stat multiplier for a nature and stat
pub fn nature_stat_multiplier(nature: &str, stat: &str) -> f64 {
    let nature_id = ID::new(nature);
    let stat_enum = NatureStat::from_str(stat);

    match (get_nature(&nature_id), stat_enum) {
        (Some(nature_def), Some(stat)) => nature_def.get_multiplier(stat),
        _ => 1.0,
    }
}

/// Get all natures that boost a specific stat
pub fn get_boosting_natures(stat: NatureStat) -> Vec<&'static NatureDef> {
    NATURES.values()
        .filter(|n| n.plus == Some(stat))
        .collect()
}

/// Get all natures that lower a specific stat
pub fn get_lowering_natures(stat: NatureStat) -> Vec<&'static NatureDef> {
    NATURES.values()
        .filter(|n| n.minus == Some(stat))
        .collect()
}

/// Get all neutral natures
pub fn get_neutral_natures() -> Vec<&'static NatureDef> {
    NATURES.values()
        .filter(|n| n.is_neutral())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adamant() {
        let adamant = get_nature_by_name("Adamant").unwrap();
        assert_eq!(adamant.name, "Adamant");
        assert_eq!(adamant.plus, Some(NatureStat::Atk));
        assert_eq!(adamant.minus, Some(NatureStat::SpA));
        assert_eq!(adamant.get_multiplier(NatureStat::Atk), 1.1);
        assert_eq!(adamant.get_multiplier(NatureStat::SpA), 0.9);
        assert_eq!(adamant.get_multiplier(NatureStat::Spe), 1.0);
    }

    #[test]
    fn test_timid() {
        let timid = get_nature_by_name("Timid").unwrap();
        assert_eq!(timid.plus, Some(NatureStat::Spe));
        assert_eq!(timid.minus, Some(NatureStat::Atk));
    }

    #[test]
    fn test_jolly() {
        let jolly = get_nature_by_name("Jolly").unwrap();
        assert_eq!(jolly.plus, Some(NatureStat::Spe));
        assert_eq!(jolly.minus, Some(NatureStat::SpA));
    }

    #[test]
    fn test_neutral_nature() {
        let hardy = get_nature_by_name("Hardy").unwrap();
        assert!(hardy.is_neutral());
        assert_eq!(hardy.get_multiplier(NatureStat::Atk), 1.0);
        assert_eq!(hardy.get_multiplier(NatureStat::Spe), 1.0);
    }

    #[test]
    fn test_nature_stat_multiplier() {
        assert_eq!(nature_stat_multiplier("adamant", "atk"), 1.1);
        assert_eq!(nature_stat_multiplier("adamant", "spa"), 0.9);
        assert_eq!(nature_stat_multiplier("adamant", "spe"), 1.0);
        assert_eq!(nature_stat_multiplier("hardy", "atk"), 1.0);
    }

    #[test]
    fn test_boosting_natures() {
        let speed_boosters = get_boosting_natures(NatureStat::Spe);
        assert_eq!(speed_boosters.len(), 4); // Timid, Hasty, Jolly, Naive
        assert!(speed_boosters.iter().any(|n| n.name == "Jolly"));
        assert!(speed_boosters.iter().any(|n| n.name == "Timid"));
    }

    #[test]
    fn test_lowering_natures() {
        let atk_lowering = get_lowering_natures(NatureStat::Atk);
        assert_eq!(atk_lowering.len(), 4); // Bold, Modest, Calm, Timid
    }

    #[test]
    fn test_neutral_natures() {
        let neutrals = get_neutral_natures();
        assert_eq!(neutrals.len(), 5); // Hardy, Docile, Serious, Bashful, Quirky
    }

    #[test]
    fn test_all_25_natures() {
        assert_eq!(NATURES.len(), 25);
    }
}
