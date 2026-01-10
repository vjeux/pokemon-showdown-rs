//! Pokemon team set (from team builder)

use serde::{Deserialize, Serialize};

use crate::dex_data::{Gender, StatsTable};

/// Pokemon team set (from team builder)
#[derive(Debug, Clone, Serialize, Deserialize)]
/// JavaScript equivalent: PokemonSet (sim/pokemon.ts)
/// 16 fields in JavaScript
pub struct PokemonSet {
    /// Pokemon name (nickname)
    /// JavaScript: name: string
    pub name: String,
    /// Species name
    /// JavaScript: species: string
    pub species: String,
    /// Held item
    /// JavaScript: item: string
    pub item: String,
    /// Ability name
    /// JavaScript: ability: string
    pub ability: String,
    /// Move names
    /// JavaScript: moves: string[]
    pub moves: Vec<String>,
    /// Nature name
    /// JavaScript: nature: string
    pub nature: String,
    /// Gender
    /// JavaScript: gender: 'M' | 'F' | 'N' | ''
    pub gender: Gender,
    /// Effort Values
    /// JavaScript: evs: StatsTable
    pub evs: StatsTable,
    /// Individual Values
    /// JavaScript: ivs: StatsTable
    pub ivs: StatsTable,
    /// Level (1-100)
    /// JavaScript: level: number
    pub level: u8,
    /// Is shiny?
    /// JavaScript: shiny: boolean
    pub shiny: bool,
    /// Happiness/Friendship
    /// JavaScript: happiness: number
    pub happiness: u8,
    /// Pokeball type
    /// JavaScript: pokeball: string
    pub pokeball: String,
    /// Hidden Power type override
    /// JavaScript: hpType?: string
    pub hptype: Option<String>,
    /// Dynamax level (0-10, Gen 8+)
    /// JavaScript: dynamaxLevel: number
    pub dynamax_level: u8,
    /// Can Gigantamax?
    /// JavaScript: gigantamax: boolean
    pub gigantamax: bool,
    /// Tera type (Gen 9+)
    /// JavaScript: teraType?: string
    pub tera_type: Option<String>,
}

impl Default for PokemonSet {
    fn default() -> Self {
        Self {
            name: String::new(),
            species: String::new(),
            item: String::new(),
            ability: String::new(),
            moves: Vec::new(),
            nature: "Hardy".to_string(),
            gender: Gender::None,
            evs: StatsTable::default(),
            ivs: StatsTable::new(31, 31, 31, 31, 31, 31),
            level: 100,
            shiny: false,
            happiness: 255,
            pokeball: String::new(),
            hptype: None,
            dynamax_level: 10,
            gigantamax: false,
            tera_type: None,
        }
    }
}
