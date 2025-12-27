//! Data-driven Species Definitions
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! This module contains data-driven definitions for Pokemon species,
//! following the Pokemon Showdown JS architecture.

use std::collections::HashMap;
use once_cell::sync::Lazy;
use crate::dex_data::ID;

/// Base stats for a species
#[derive(Debug, Clone, Copy, Default)]
pub struct BaseStats {
    pub hp: i32,
    pub atk: i32,
    pub def: i32,
    pub spa: i32,
    pub spd: i32,
    pub spe: i32,
}

impl BaseStats {
    pub const fn new(hp: i32, atk: i32, def: i32, spa: i32, spd: i32, spe: i32) -> Self {
        Self { hp, atk, def, spa, spd, spe }
    }

    /// Get base stat total (BST)
    pub fn total(&self) -> i32 {
        self.hp + self.atk + self.def + self.spa + self.spd + self.spe
    }
}

/// Gender ratio for a species
#[derive(Debug, Clone, Copy)]
pub enum GenderRatio {
    /// Always male
    MaleOnly,
    /// Always female
    FemaleOnly,
    /// Genderless (e.g., legendaries, magnemite)
    Genderless,
    /// Custom ratio (male percentage, e.g., 87.5 for starters)
    Ratio { male: f64 },
}

impl Default for GenderRatio {
    fn default() -> Self {
        GenderRatio::Ratio { male: 50.0 }
    }
}

/// Species definition
#[derive(Debug, Clone)]
pub struct SpeciesDef {
    /// National dex number
    pub num: i32,
    /// Display name
    pub name: &'static str,
    /// Types (1-2)
    pub types: &'static [&'static str],
    /// Base stats
    pub base_stats: BaseStats,
    /// Abilities: (primary, secondary, hidden)
    pub abilities: (&'static str, Option<&'static str>, Option<&'static str>),
    /// Height in meters
    pub height: f64,
    /// Weight in kg
    pub weight: f64,
    /// Gender ratio
    pub gender_ratio: GenderRatio,
    /// Evolution chain info
    pub prevo: Option<&'static str>,
    pub evos: &'static [&'static str],
    /// Forme info
    pub base_species: Option<&'static str>,
    pub forme: Option<&'static str>,
    /// Egg groups
    pub egg_groups: &'static [&'static str],
    /// Tier (for formats)
    pub tier: &'static str,
}

impl SpeciesDef {
    /// Check if species has a specific type
    pub fn has_type(&self, type_name: &str) -> bool {
        let lower = type_name.to_lowercase();
        self.types.iter().any(|t| t.to_lowercase() == lower)
    }

    /// Check if species has a specific ability
    pub fn has_ability(&self, ability: &str) -> bool {
        let lower = ability.to_lowercase();
        let ability_id = lower.chars().filter(|c| c.is_alphanumeric()).collect::<String>();

        let check = |a: &str| -> bool {
            a.to_lowercase().chars().filter(|c| c.is_alphanumeric()).collect::<String>() == ability_id
        };

        check(self.abilities.0)
            || self.abilities.1.map_or(false, check)
            || self.abilities.2.map_or(false, check)
    }

    /// Get primary type
    pub fn primary_type(&self) -> &'static str {
        self.types.first().copied().unwrap_or("Normal")
    }

    /// Get secondary type (if any)
    pub fn secondary_type(&self) -> Option<&'static str> {
        if self.types.len() > 1 {
            Some(self.types[1])
        } else {
            None
        }
    }

    /// Check if this is a legendary/mythical
    pub fn is_legendary(&self) -> bool {
        // Legendaries by dex number ranges
        matches!(self.num,
            144..=146 | 150..=151 |  // Gen 1: Birds, Mewtwo, Mew
            243..=251 |              // Gen 2: Beasts, Lugia, Ho-Oh, Celebi
            377..=386 |              // Gen 3: Regis, Latis, Weather trio, Jirachi, Deoxys
            480..=494 |              // Gen 4: Lake trio, Creation trio, Heatran, etc
            638..=649 |              // Gen 5: Swords, Tao trio, Forces, Meloetta, Genesect
            716..=721 |              // Gen 6: XY legends
            785..=809 |              // Gen 7: Tapus, UBs, etc
            888..=898 |              // Gen 8: Zacian, Zamazenta, Eternatus, Calyrex, etc
            1001..=1025              // Gen 9: Treasures, Paradox legends, etc
        )
    }
}

/// Static species registry
pub static SPECIES: Lazy<HashMap<ID, SpeciesDef>> = Lazy::new(|| {
    let mut species = HashMap::new();

    // Gen 1 Starters
    species.insert(ID::new("bulbasaur"), SpeciesDef {
        num: 1,
        name: "Bulbasaur",
        types: &["Grass", "Poison"],
        base_stats: BaseStats::new(45, 49, 49, 65, 65, 45),
        abilities: ("Overgrow", None, Some("Chlorophyll")),
        height: 0.7,
        weight: 6.9,
        gender_ratio: GenderRatio::Ratio { male: 87.5 },
        prevo: None,
        evos: &["Ivysaur"],
        base_species: None,
        forme: None,
        egg_groups: &["Monster", "Grass"],
        tier: "LC",
    });

    species.insert(ID::new("ivysaur"), SpeciesDef {
        num: 2,
        name: "Ivysaur",
        types: &["Grass", "Poison"],
        base_stats: BaseStats::new(60, 62, 63, 80, 80, 60),
        abilities: ("Overgrow", None, Some("Chlorophyll")),
        height: 1.0,
        weight: 13.0,
        gender_ratio: GenderRatio::Ratio { male: 87.5 },
        prevo: Some("Bulbasaur"),
        evos: &["Venusaur"],
        base_species: None,
        forme: None,
        egg_groups: &["Monster", "Grass"],
        tier: "NFE",
    });

    species.insert(ID::new("venusaur"), SpeciesDef {
        num: 3,
        name: "Venusaur",
        types: &["Grass", "Poison"],
        base_stats: BaseStats::new(80, 82, 83, 100, 100, 80),
        abilities: ("Overgrow", None, Some("Chlorophyll")),
        height: 2.0,
        weight: 100.0,
        gender_ratio: GenderRatio::Ratio { male: 87.5 },
        prevo: Some("Ivysaur"),
        evos: &[],
        base_species: None,
        forme: None,
        egg_groups: &["Monster", "Grass"],
        tier: "UU",
    });

    species.insert(ID::new("charmander"), SpeciesDef {
        num: 4,
        name: "Charmander",
        types: &["Fire"],
        base_stats: BaseStats::new(39, 52, 43, 60, 50, 65),
        abilities: ("Blaze", None, Some("Solar Power")),
        height: 0.6,
        weight: 8.5,
        gender_ratio: GenderRatio::Ratio { male: 87.5 },
        prevo: None,
        evos: &["Charmeleon"],
        base_species: None,
        forme: None,
        egg_groups: &["Monster", "Dragon"],
        tier: "LC",
    });

    species.insert(ID::new("charmeleon"), SpeciesDef {
        num: 5,
        name: "Charmeleon",
        types: &["Fire"],
        base_stats: BaseStats::new(58, 64, 58, 80, 65, 80),
        abilities: ("Blaze", None, Some("Solar Power")),
        height: 1.1,
        weight: 19.0,
        gender_ratio: GenderRatio::Ratio { male: 87.5 },
        prevo: Some("Charmander"),
        evos: &["Charizard"],
        base_species: None,
        forme: None,
        egg_groups: &["Monster", "Dragon"],
        tier: "NFE",
    });

    species.insert(ID::new("charizard"), SpeciesDef {
        num: 6,
        name: "Charizard",
        types: &["Fire", "Flying"],
        base_stats: BaseStats::new(78, 84, 78, 109, 85, 100),
        abilities: ("Blaze", None, Some("Solar Power")),
        height: 1.7,
        weight: 90.5,
        gender_ratio: GenderRatio::Ratio { male: 87.5 },
        prevo: Some("Charmeleon"),
        evos: &[],
        base_species: None,
        forme: None,
        egg_groups: &["Monster", "Dragon"],
        tier: "UU",
    });

    species.insert(ID::new("squirtle"), SpeciesDef {
        num: 7,
        name: "Squirtle",
        types: &["Water"],
        base_stats: BaseStats::new(44, 48, 65, 50, 64, 43),
        abilities: ("Torrent", None, Some("Rain Dish")),
        height: 0.5,
        weight: 9.0,
        gender_ratio: GenderRatio::Ratio { male: 87.5 },
        prevo: None,
        evos: &["Wartortle"],
        base_species: None,
        forme: None,
        egg_groups: &["Monster", "Water 1"],
        tier: "LC",
    });

    species.insert(ID::new("wartortle"), SpeciesDef {
        num: 8,
        name: "Wartortle",
        types: &["Water"],
        base_stats: BaseStats::new(59, 63, 80, 65, 80, 58),
        abilities: ("Torrent", None, Some("Rain Dish")),
        height: 1.0,
        weight: 22.5,
        gender_ratio: GenderRatio::Ratio { male: 87.5 },
        prevo: Some("Squirtle"),
        evos: &["Blastoise"],
        base_species: None,
        forme: None,
        egg_groups: &["Monster", "Water 1"],
        tier: "NFE",
    });

    species.insert(ID::new("blastoise"), SpeciesDef {
        num: 9,
        name: "Blastoise",
        types: &["Water"],
        base_stats: BaseStats::new(79, 83, 100, 85, 105, 78),
        abilities: ("Torrent", None, Some("Rain Dish")),
        height: 1.6,
        weight: 85.5,
        gender_ratio: GenderRatio::Ratio { male: 87.5 },
        prevo: Some("Wartortle"),
        evos: &[],
        base_species: None,
        forme: None,
        egg_groups: &["Monster", "Water 1"],
        tier: "UU",
    });

    // Pikachu line
    species.insert(ID::new("pichu"), SpeciesDef {
        num: 172,
        name: "Pichu",
        types: &["Electric"],
        base_stats: BaseStats::new(20, 40, 15, 35, 35, 60),
        abilities: ("Static", None, Some("Lightning Rod")),
        height: 0.3,
        weight: 2.0,
        gender_ratio: GenderRatio::Ratio { male: 50.0 },
        prevo: None,
        evos: &["Pikachu"],
        base_species: None,
        forme: None,
        egg_groups: &["Undiscovered"],
        tier: "LC",
    });

    species.insert(ID::new("pikachu"), SpeciesDef {
        num: 25,
        name: "Pikachu",
        types: &["Electric"],
        base_stats: BaseStats::new(35, 55, 40, 50, 50, 90),
        abilities: ("Static", None, Some("Lightning Rod")),
        height: 0.4,
        weight: 6.0,
        gender_ratio: GenderRatio::Ratio { male: 50.0 },
        prevo: Some("Pichu"),
        evos: &["Raichu"],
        base_species: None,
        forme: None,
        egg_groups: &["Field", "Fairy"],
        tier: "PU",
    });

    species.insert(ID::new("raichu"), SpeciesDef {
        num: 26,
        name: "Raichu",
        types: &["Electric"],
        base_stats: BaseStats::new(60, 90, 55, 90, 80, 110),
        abilities: ("Static", None, Some("Lightning Rod")),
        height: 0.8,
        weight: 30.0,
        gender_ratio: GenderRatio::Ratio { male: 50.0 },
        prevo: Some("Pikachu"),
        evos: &[],
        base_species: None,
        forme: None,
        egg_groups: &["Field", "Fairy"],
        tier: "PU",
    });

    // Common competitive Pokemon
    species.insert(ID::new("dragonite"), SpeciesDef {
        num: 149,
        name: "Dragonite",
        types: &["Dragon", "Flying"],
        base_stats: BaseStats::new(91, 134, 95, 100, 100, 80),
        abilities: ("Inner Focus", None, Some("Multiscale")),
        height: 2.2,
        weight: 210.0,
        gender_ratio: GenderRatio::Ratio { male: 50.0 },
        prevo: Some("Dragonair"),
        evos: &[],
        base_species: None,
        forme: None,
        egg_groups: &["Water 1", "Dragon"],
        tier: "OU",
    });

    species.insert(ID::new("tyranitar"), SpeciesDef {
        num: 248,
        name: "Tyranitar",
        types: &["Rock", "Dark"],
        base_stats: BaseStats::new(100, 134, 110, 95, 100, 61),
        abilities: ("Sand Stream", None, Some("Unnerve")),
        height: 2.0,
        weight: 202.0,
        gender_ratio: GenderRatio::Ratio { male: 50.0 },
        prevo: Some("Pupitar"),
        evos: &[],
        base_species: None,
        forme: None,
        egg_groups: &["Monster"],
        tier: "OU",
    });

    species.insert(ID::new("garchomp"), SpeciesDef {
        num: 445,
        name: "Garchomp",
        types: &["Dragon", "Ground"],
        base_stats: BaseStats::new(108, 130, 95, 80, 85, 102),
        abilities: ("Sand Veil", None, Some("Rough Skin")),
        height: 1.9,
        weight: 95.0,
        gender_ratio: GenderRatio::Ratio { male: 50.0 },
        prevo: Some("Gabite"),
        evos: &[],
        base_species: None,
        forme: None,
        egg_groups: &["Monster", "Dragon"],
        tier: "OU",
    });

    species.insert(ID::new("ferrothorn"), SpeciesDef {
        num: 598,
        name: "Ferrothorn",
        types: &["Grass", "Steel"],
        base_stats: BaseStats::new(74, 94, 131, 54, 116, 20),
        abilities: ("Iron Barbs", None, Some("Anticipation")),
        height: 1.0,
        weight: 110.0,
        gender_ratio: GenderRatio::Ratio { male: 50.0 },
        prevo: Some("Ferroseed"),
        evos: &[],
        base_species: None,
        forme: None,
        egg_groups: &["Grass", "Mineral"],
        tier: "OU",
    });

    species.insert(ID::new("landorustherian"), SpeciesDef {
        num: 645,
        name: "Landorus-Therian",
        types: &["Ground", "Flying"],
        base_stats: BaseStats::new(89, 145, 90, 105, 80, 91),
        abilities: ("Intimidate", None, None),
        height: 1.3,
        weight: 68.0,
        gender_ratio: GenderRatio::MaleOnly,
        prevo: None,
        evos: &[],
        base_species: Some("Landorus"),
        forme: Some("Therian"),
        egg_groups: &["Undiscovered"],
        tier: "OU",
    });

    species.insert(ID::new("toxapex"), SpeciesDef {
        num: 748,
        name: "Toxapex",
        types: &["Poison", "Water"],
        base_stats: BaseStats::new(50, 63, 152, 53, 142, 35),
        abilities: ("Merciless", Some("Limber"), Some("Regenerator")),
        height: 0.7,
        weight: 14.5,
        gender_ratio: GenderRatio::Ratio { male: 50.0 },
        prevo: Some("Mareanie"),
        evos: &[],
        base_species: None,
        forme: None,
        egg_groups: &["Water 1"],
        tier: "OU",
    });

    species.insert(ID::new("dragapult"), SpeciesDef {
        num: 887,
        name: "Dragapult",
        types: &["Dragon", "Ghost"],
        base_stats: BaseStats::new(88, 120, 75, 100, 75, 142),
        abilities: ("Clear Body", Some("Infiltrator"), Some("Cursed Body")),
        height: 3.0,
        weight: 50.0,
        gender_ratio: GenderRatio::Ratio { male: 50.0 },
        prevo: Some("Drakloak"),
        evos: &[],
        base_species: None,
        forme: None,
        egg_groups: &["Amorphous", "Dragon"],
        tier: "OU",
    });

    // Legendaries
    species.insert(ID::new("mewtwo"), SpeciesDef {
        num: 150,
        name: "Mewtwo",
        types: &["Psychic"],
        base_stats: BaseStats::new(106, 110, 90, 154, 90, 130),
        abilities: ("Pressure", None, Some("Unnerve")),
        height: 2.0,
        weight: 122.0,
        gender_ratio: GenderRatio::Genderless,
        prevo: None,
        evos: &[],
        base_species: None,
        forme: None,
        egg_groups: &["Undiscovered"],
        tier: "Uber",
    });

    species.insert(ID::new("kyogre"), SpeciesDef {
        num: 382,
        name: "Kyogre",
        types: &["Water"],
        base_stats: BaseStats::new(100, 100, 90, 150, 140, 90),
        abilities: ("Drizzle", None, None),
        height: 4.5,
        weight: 352.0,
        gender_ratio: GenderRatio::Genderless,
        prevo: None,
        evos: &[],
        base_species: None,
        forme: None,
        egg_groups: &["Undiscovered"],
        tier: "Uber",
    });

    species.insert(ID::new("groudon"), SpeciesDef {
        num: 383,
        name: "Groudon",
        types: &["Ground"],
        base_stats: BaseStats::new(100, 150, 140, 100, 90, 90),
        abilities: ("Drought", None, None),
        height: 3.5,
        weight: 950.0,
        gender_ratio: GenderRatio::Genderless,
        prevo: None,
        evos: &[],
        base_species: None,
        forme: None,
        egg_groups: &["Undiscovered"],
        tier: "Uber",
    });

    species.insert(ID::new("rayquaza"), SpeciesDef {
        num: 384,
        name: "Rayquaza",
        types: &["Dragon", "Flying"],
        base_stats: BaseStats::new(105, 150, 90, 150, 90, 95),
        abilities: ("Air Lock", None, None),
        height: 7.0,
        weight: 206.5,
        gender_ratio: GenderRatio::Genderless,
        prevo: None,
        evos: &[],
        base_species: None,
        forme: None,
        egg_groups: &["Undiscovered"],
        tier: "AG",
    });

    // Common Pokemon
    species.insert(ID::new("alakazam"), SpeciesDef {
        num: 65,
        name: "Alakazam",
        types: &["Psychic"],
        base_stats: BaseStats::new(55, 50, 45, 135, 95, 120),
        abilities: ("Synchronize", Some("Inner Focus"), Some("Magic Guard")),
        height: 1.5,
        weight: 48.0,
        gender_ratio: GenderRatio::Ratio { male: 75.0 },
        prevo: Some("Kadabra"),
        evos: &[],
        base_species: None,
        forme: None,
        egg_groups: &["Human-Like"],
        tier: "UU",
    });

    species.insert(ID::new("machamp"), SpeciesDef {
        num: 68,
        name: "Machamp",
        types: &["Fighting"],
        base_stats: BaseStats::new(90, 130, 80, 65, 85, 55),
        abilities: ("Guts", Some("No Guard"), Some("Steadfast")),
        height: 1.6,
        weight: 130.0,
        gender_ratio: GenderRatio::Ratio { male: 75.0 },
        prevo: Some("Machoke"),
        evos: &[],
        base_species: None,
        forme: None,
        egg_groups: &["Human-Like"],
        tier: "RU",
    });

    species.insert(ID::new("gengar"), SpeciesDef {
        num: 94,
        name: "Gengar",
        types: &["Ghost", "Poison"],
        base_stats: BaseStats::new(60, 65, 60, 130, 75, 110),
        abilities: ("Cursed Body", None, None),
        height: 1.5,
        weight: 40.5,
        gender_ratio: GenderRatio::Ratio { male: 50.0 },
        prevo: Some("Haunter"),
        evos: &[],
        base_species: None,
        forme: None,
        egg_groups: &["Amorphous"],
        tier: "UU",
    });

    species.insert(ID::new("gyarados"), SpeciesDef {
        num: 130,
        name: "Gyarados",
        types: &["Water", "Flying"],
        base_stats: BaseStats::new(95, 125, 79, 60, 100, 81),
        abilities: ("Intimidate", None, Some("Moxie")),
        height: 6.5,
        weight: 235.0,
        gender_ratio: GenderRatio::Ratio { male: 50.0 },
        prevo: Some("Magikarp"),
        evos: &[],
        base_species: None,
        forme: None,
        egg_groups: &["Water 2", "Dragon"],
        tier: "UU",
    });

    species.insert(ID::new("snorlax"), SpeciesDef {
        num: 143,
        name: "Snorlax",
        types: &["Normal"],
        base_stats: BaseStats::new(160, 110, 65, 65, 110, 30),
        abilities: ("Immunity", Some("Thick Fat"), Some("Gluttony")),
        height: 2.1,
        weight: 460.0,
        gender_ratio: GenderRatio::Ratio { male: 87.5 },
        prevo: Some("Munchlax"),
        evos: &[],
        base_species: None,
        forme: None,
        egg_groups: &["Monster"],
        tier: "RU",
    });

    species.insert(ID::new("scizor"), SpeciesDef {
        num: 212,
        name: "Scizor",
        types: &["Bug", "Steel"],
        base_stats: BaseStats::new(70, 130, 100, 55, 80, 65),
        abilities: ("Swarm", Some("Technician"), Some("Light Metal")),
        height: 1.8,
        weight: 118.0,
        gender_ratio: GenderRatio::Ratio { male: 50.0 },
        prevo: Some("Scyther"),
        evos: &[],
        base_species: None,
        forme: None,
        egg_groups: &["Bug"],
        tier: "UU",
    });

    species.insert(ID::new("blaziken"), SpeciesDef {
        num: 257,
        name: "Blaziken",
        types: &["Fire", "Fighting"],
        base_stats: BaseStats::new(80, 120, 70, 110, 70, 80),
        abilities: ("Blaze", None, Some("Speed Boost")),
        height: 1.9,
        weight: 52.0,
        gender_ratio: GenderRatio::Ratio { male: 87.5 },
        prevo: Some("Combusken"),
        evos: &[],
        base_species: None,
        forme: None,
        egg_groups: &["Field"],
        tier: "Uber",
    });

    species.insert(ID::new("salamence"), SpeciesDef {
        num: 373,
        name: "Salamence",
        types: &["Dragon", "Flying"],
        base_stats: BaseStats::new(95, 135, 80, 110, 80, 100),
        abilities: ("Intimidate", None, Some("Moxie")),
        height: 1.5,
        weight: 102.6,
        gender_ratio: GenderRatio::Ratio { male: 50.0 },
        prevo: Some("Shelgon"),
        evos: &[],
        base_species: None,
        forme: None,
        egg_groups: &["Dragon"],
        tier: "UU",
    });

    species.insert(ID::new("metagross"), SpeciesDef {
        num: 376,
        name: "Metagross",
        types: &["Steel", "Psychic"],
        base_stats: BaseStats::new(80, 135, 130, 95, 90, 70),
        abilities: ("Clear Body", None, Some("Light Metal")),
        height: 1.6,
        weight: 550.0,
        gender_ratio: GenderRatio::Genderless,
        prevo: Some("Metang"),
        evos: &[],
        base_species: None,
        forme: None,
        egg_groups: &["Mineral"],
        tier: "UU",
    });

    species.insert(ID::new("lucario"), SpeciesDef {
        num: 448,
        name: "Lucario",
        types: &["Fighting", "Steel"],
        base_stats: BaseStats::new(70, 110, 70, 115, 70, 90),
        abilities: ("Steadfast", Some("Inner Focus"), Some("Justified")),
        height: 1.2,
        weight: 54.0,
        gender_ratio: GenderRatio::Ratio { male: 87.5 },
        prevo: Some("Riolu"),
        evos: &[],
        base_species: None,
        forme: None,
        egg_groups: &["Field", "Human-Like"],
        tier: "RU",
    });

    species.insert(ID::new("rotomwash"), SpeciesDef {
        num: 479,
        name: "Rotom-Wash",
        types: &["Electric", "Water"],
        base_stats: BaseStats::new(50, 65, 107, 105, 107, 86),
        abilities: ("Levitate", None, None),
        height: 0.3,
        weight: 0.3,
        gender_ratio: GenderRatio::Genderless,
        prevo: None,
        evos: &[],
        base_species: Some("Rotom"),
        forme: Some("Wash"),
        egg_groups: &["Undiscovered"],
        tier: "OU",
    });

    species.insert(ID::new("heatran"), SpeciesDef {
        num: 485,
        name: "Heatran",
        types: &["Fire", "Steel"],
        base_stats: BaseStats::new(91, 90, 106, 130, 106, 77),
        abilities: ("Flash Fire", None, Some("Flame Body")),
        height: 1.7,
        weight: 430.0,
        gender_ratio: GenderRatio::Ratio { male: 50.0 },
        prevo: None,
        evos: &[],
        base_species: None,
        forme: None,
        egg_groups: &["Undiscovered"],
        tier: "OU",
    });

    species.insert(ID::new("excadrill"), SpeciesDef {
        num: 530,
        name: "Excadrill",
        types: &["Ground", "Steel"],
        base_stats: BaseStats::new(110, 135, 60, 50, 65, 88),
        abilities: ("Sand Rush", Some("Sand Force"), Some("Mold Breaker")),
        height: 0.7,
        weight: 40.4,
        gender_ratio: GenderRatio::Ratio { male: 50.0 },
        prevo: Some("Drilbur"),
        evos: &[],
        base_species: None,
        forme: None,
        egg_groups: &["Field"],
        tier: "OU",
    });

    species.insert(ID::new("volcarona"), SpeciesDef {
        num: 637,
        name: "Volcarona",
        types: &["Bug", "Fire"],
        base_stats: BaseStats::new(85, 60, 65, 135, 105, 100),
        abilities: ("Flame Body", None, Some("Swarm")),
        height: 1.6,
        weight: 46.0,
        gender_ratio: GenderRatio::Ratio { male: 50.0 },
        prevo: Some("Larvesta"),
        evos: &[],
        base_species: None,
        forme: None,
        egg_groups: &["Bug"],
        tier: "OU",
    });

    species.insert(ID::new("clefable"), SpeciesDef {
        num: 36,
        name: "Clefable",
        types: &["Fairy"],
        base_stats: BaseStats::new(95, 70, 73, 95, 90, 60),
        abilities: ("Cute Charm", Some("Magic Guard"), Some("Unaware")),
        height: 1.3,
        weight: 40.0,
        gender_ratio: GenderRatio::Ratio { male: 25.0 },
        prevo: Some("Clefairy"),
        evos: &[],
        base_species: None,
        forme: None,
        egg_groups: &["Fairy"],
        tier: "OU",
    });

    species.insert(ID::new("weavile"), SpeciesDef {
        num: 461,
        name: "Weavile",
        types: &["Dark", "Ice"],
        base_stats: BaseStats::new(70, 120, 65, 45, 85, 125),
        abilities: ("Pressure", None, Some("Pickpocket")),
        height: 1.1,
        weight: 34.0,
        gender_ratio: GenderRatio::Ratio { male: 50.0 },
        prevo: Some("Sneasel"),
        evos: &[],
        base_species: None,
        forme: None,
        egg_groups: &["Field"],
        tier: "OU",
    });

    species.insert(ID::new("magnezone"), SpeciesDef {
        num: 462,
        name: "Magnezone",
        types: &["Electric", "Steel"],
        base_stats: BaseStats::new(70, 70, 115, 130, 90, 60),
        abilities: ("Magnet Pull", Some("Sturdy"), Some("Analytic")),
        height: 1.2,
        weight: 180.0,
        gender_ratio: GenderRatio::Genderless,
        prevo: Some("Magneton"),
        evos: &[],
        base_species: None,
        forme: None,
        egg_groups: &["Mineral"],
        tier: "UU",
    });

    species.insert(ID::new("gliscor"), SpeciesDef {
        num: 472,
        name: "Gliscor",
        types: &["Ground", "Flying"],
        base_stats: BaseStats::new(75, 95, 125, 45, 75, 95),
        abilities: ("Hyper Cutter", Some("Sand Veil"), Some("Poison Heal")),
        height: 2.0,
        weight: 42.5,
        gender_ratio: GenderRatio::Ratio { male: 50.0 },
        prevo: Some("Gligar"),
        evos: &[],
        base_species: None,
        forme: None,
        egg_groups: &["Bug"],
        tier: "OU",
    });

    species.insert(ID::new("ditto"), SpeciesDef {
        num: 132,
        name: "Ditto",
        types: &["Normal"],
        base_stats: BaseStats::new(48, 48, 48, 48, 48, 48),
        abilities: ("Limber", None, Some("Imposter")),
        height: 0.3,
        weight: 4.0,
        gender_ratio: GenderRatio::Genderless,
        prevo: None,
        evos: &[],
        base_species: None,
        forme: None,
        egg_groups: &["Ditto"],
        tier: "ZU",
    });

    species.insert(ID::new("skarmory"), SpeciesDef {
        num: 227,
        name: "Skarmory",
        types: &["Steel", "Flying"],
        base_stats: BaseStats::new(65, 80, 140, 40, 70, 70),
        abilities: ("Keen Eye", Some("Sturdy"), Some("Weak Armor")),
        height: 1.7,
        weight: 50.5,
        gender_ratio: GenderRatio::Ratio { male: 50.0 },
        prevo: None,
        evos: &[],
        base_species: None,
        forme: None,
        egg_groups: &["Flying"],
        tier: "UU",
    });

    species.insert(ID::new("blissey"), SpeciesDef {
        num: 242,
        name: "Blissey",
        types: &["Normal"],
        base_stats: BaseStats::new(255, 10, 10, 75, 135, 55),
        abilities: ("Natural Cure", Some("Serene Grace"), Some("Healer")),
        height: 1.5,
        weight: 46.8,
        gender_ratio: GenderRatio::FemaleOnly,
        prevo: Some("Chansey"),
        evos: &[],
        base_species: None,
        forme: None,
        egg_groups: &["Fairy"],
        tier: "OU",
    });

    species
});

/// Get a species definition by ID
pub fn get_species(id: &ID) -> Option<&'static SpeciesDef> {
    SPECIES.get(id)
}

/// Get a species by name (case-insensitive)
pub fn get_species_by_name(name: &str) -> Option<&'static SpeciesDef> {
    get_species(&ID::new(name))
}

/// Check if a species exists
pub fn species_exists(name: &str) -> bool {
    get_species(&ID::new(name)).is_some()
}

/// Get all species of a specific type
pub fn get_species_by_type(type_name: &str) -> Vec<&'static SpeciesDef> {
    SPECIES.values()
        .filter(|s| s.has_type(type_name))
        .collect()
}

/// Get all species in a specific tier
pub fn get_species_by_tier(tier: &str) -> Vec<&'static SpeciesDef> {
    let tier_lower = tier.to_lowercase();
    SPECIES.values()
        .filter(|s| s.tier.to_lowercase() == tier_lower)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pikachu() {
        let pika = get_species_by_name("Pikachu").unwrap();
        assert_eq!(pika.num, 25);
        assert_eq!(pika.name, "Pikachu");
        assert!(pika.has_type("Electric"));
        assert!(!pika.has_type("Normal"));
        assert!(pika.has_ability("Static"));
        assert!(pika.has_ability("Lightning Rod"));
        assert_eq!(pika.base_stats.spe, 90);
    }

    #[test]
    fn test_charizard_dual_type() {
        let zard = get_species_by_name("Charizard").unwrap();
        assert_eq!(zard.types.len(), 2);
        assert!(zard.has_type("Fire"));
        assert!(zard.has_type("Flying"));
        assert_eq!(zard.primary_type(), "Fire");
        assert_eq!(zard.secondary_type(), Some("Flying"));
    }

    #[test]
    fn test_base_stats() {
        let ttar = get_species_by_name("Tyranitar").unwrap();
        assert_eq!(ttar.base_stats.hp, 100);
        assert_eq!(ttar.base_stats.atk, 134);
        assert_eq!(ttar.base_stats.total(), 600);
    }

    #[test]
    fn test_legendary() {
        let mewtwo = get_species_by_name("Mewtwo").unwrap();
        assert!(mewtwo.is_legendary());

        let pikachu = get_species_by_name("Pikachu").unwrap();
        assert!(!pikachu.is_legendary());
    }

    #[test]
    fn test_gender_ratio() {
        let blissey = get_species_by_name("Blissey").unwrap();
        assert!(matches!(blissey.gender_ratio, GenderRatio::FemaleOnly));

        let mewtwo = get_species_by_name("Mewtwo").unwrap();
        assert!(matches!(mewtwo.gender_ratio, GenderRatio::Genderless));

        let starter = get_species_by_name("Bulbasaur").unwrap();
        if let GenderRatio::Ratio { male } = starter.gender_ratio {
            assert_eq!(male, 87.5);
        } else {
            panic!("Expected ratio");
        }
    }

    #[test]
    fn test_formes() {
        let lando_t = get_species_by_name("Landorus-Therian").unwrap();
        assert_eq!(lando_t.base_species, Some("Landorus"));
        assert_eq!(lando_t.forme, Some("Therian"));
        assert!(lando_t.has_ability("Intimidate"));
    }

    #[test]
    fn test_species_by_type() {
        let fire_types = get_species_by_type("Fire");
        assert!(fire_types.iter().any(|s| s.name == "Charizard"));
        assert!(fire_types.iter().any(|s| s.name == "Blaziken"));
        assert!(!fire_types.iter().any(|s| s.name == "Pikachu"));
    }

    #[test]
    fn test_species_by_tier() {
        let ou_mons = get_species_by_tier("OU");
        assert!(ou_mons.iter().any(|s| s.name == "Garchomp"));
        assert!(ou_mons.iter().any(|s| s.name == "Dragonite"));
    }
}
