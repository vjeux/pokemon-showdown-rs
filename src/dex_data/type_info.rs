//! Type info and DexTypes

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::{EffectType, ID, Nonstandard};

/// Type info for type chart
#[derive(Debug, Clone, Serialize, Deserialize)]
/// JavaScript equivalent: TypeInfo (sim/global-types.ts)
pub struct TypeInfo {
    /// JavaScript: id: ID
    pub id: ID,
    /// JavaScript: name: string
    pub name: String,
    /// JavaScript: effectType: EffectType
    pub effect_type: EffectType,
    /// JavaScript: exists: boolean
    pub exists: bool,
    /// JavaScript: gen: number
    pub gen: u8,
    /// JavaScript: isNonstandard?: Nonstandard
    pub is_nonstandard: Option<Nonstandard>,
    /// Type chart: 0 = normal, 1 = weakness, 2 = resistance, 3 = immunity
    /// JavaScript: damageTaken: { [type: string]: number }
    pub damage_taken: HashMap<String, u8>,
}

impl TypeInfo {
    pub fn new(name: &str) -> Self {
        Self {
            id: ID::new(name),
            name: name.to_string(),
            effect_type: EffectType::Condition,
            exists: true,
            gen: 1,
            is_nonstandard: None,
            damage_taken: HashMap::new(),
        }
    }

    /// Get the damage multiplier when attacked by a move of the given type
    pub fn damage_multiplier(&self, attacking_type: &str) -> f64 {
        match self.damage_taken.get(attacking_type) {
            Some(1) => 2.0, // Weakness
            Some(2) => 0.5, // Resistance
            Some(3) => 0.0, // Immunity
            _ => 1.0,       // Normal
        }
    }
}

impl std::fmt::Display for TypeInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

/// Types collection with caching
/// JavaScript equivalent: DexTypes (sim/dex-data.ts)
/// Fields: dex, allCache, namesCache
pub struct DexTypes {
    gen: u8,
    type_cache: HashMap<String, TypeInfo>,
    all_cache: Option<Vec<TypeInfo>>,
    names_cache: Option<Vec<String>>,
}

impl DexTypes {
    /// Create a new DexTypes for a given generation
    pub fn new(gen: u8) -> Self {
        Self {
            gen,
            type_cache: HashMap::new(),
            all_cache: None,
            names_cache: None,
        }
    }

    /// Get a type by name
    pub fn get(&mut self, name: &str) -> TypeInfo {
        self.get_by_id(&super::to_id(name))
    }

    /// Get a type by ID
    pub fn get_by_id(&mut self, id: &str) -> TypeInfo {
        if id.is_empty() {
            return TypeInfo {
                id: ID::empty(),
                name: String::new(),
                effect_type: EffectType::Condition,
                exists: false,
                gen: 0,
                is_nonstandard: None,
                damage_taken: HashMap::new(),
            };
        }

        if let Some(type_info) = self.type_cache.get(id) {
            return type_info.clone();
        }

        // Capitalize the name
        let type_name = capitalize_first(id);

        if let Some(type_info) = get_type_data(id, &type_name, self.gen) {
            self.type_cache.insert(id.to_string(), type_info.clone());
            return type_info;
        }

        // Return non-existent type
        TypeInfo {
            id: ID::new(id),
            name: type_name,
            effect_type: EffectType::Condition,
            exists: false,
            gen: 0,
            is_nonstandard: None,
            damage_taken: HashMap::new(),
        }
    }

    /// Get all type names (excluding nonstandard)
    pub fn names(&mut self) -> Vec<String> {
        if let Some(ref names) = self.names_cache {
            return names.clone();
        }

        let names: Vec<String> = self
            .all()
            .into_iter()
            .filter(|t| t.is_nonstandard.is_none())
            .map(|t| t.name)
            .collect();
        self.names_cache = Some(names.clone());
        names
    }

    /// Check if a name is a valid type name
    pub fn is_name(&self, name: &str) -> bool {
        if name.is_empty() {
            return false;
        }
        let id = name.to_lowercase();
        let type_name = capitalize_first(&id);
        name == type_name && is_valid_type(&id)
    }

    /// Get all types
    pub fn all(&mut self) -> Vec<TypeInfo> {
        if let Some(ref all) = self.all_cache {
            return all.clone();
        }

        let types = get_all_types(self.gen);
        self.all_cache = Some(types.clone());
        types
    }
}

pub fn capitalize_first(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

fn is_valid_type(id: &str) -> bool {
    matches!(
        id,
        "normal"
            | "fire"
            | "water"
            | "electric"
            | "grass"
            | "ice"
            | "fighting"
            | "poison"
            | "ground"
            | "flying"
            | "psychic"
            | "bug"
            | "rock"
            | "ghost"
            | "dragon"
            | "dark"
            | "steel"
            | "fairy"
    )
}

fn get_type_data(id: &str, name: &str, gen: u8) -> Option<TypeInfo> {
    // Build damage_taken based on type chart
    let damage_taken = get_type_chart(id)?;

    // Check if type exists in this gen
    let (type_gen, is_nonstandard) = match id {
        "dark" | "steel" => (
            2,
            if gen < 2 {
                Some(Nonstandard::Future)
            } else {
                None
            },
        ),
        "fairy" => (
            6,
            if gen < 6 {
                Some(Nonstandard::Future)
            } else {
                None
            },
        ),
        _ => (1, None),
    };

    Some(TypeInfo {
        id: ID::new(id),
        name: name.to_string(),
        effect_type: EffectType::Condition,
        exists: true,
        gen: type_gen,
        is_nonstandard,
        damage_taken,
    })
}

fn get_type_chart(id: &str) -> Option<HashMap<String, u8>> {
    // 0 = normal, 1 = weakness, 2 = resistance, 3 = immunity
    let mut chart = HashMap::new();

    match id {
        "normal" => {
            chart.insert("fighting".to_string(), 1);
            chart.insert("ghost".to_string(), 3);
        }
        "fire" => {
            chart.insert("fire".to_string(), 2);
            chart.insert("water".to_string(), 1);
            chart.insert("grass".to_string(), 2);
            chart.insert("ice".to_string(), 2);
            chart.insert("ground".to_string(), 1);
            chart.insert("bug".to_string(), 2);
            chart.insert("rock".to_string(), 1);
            chart.insert("steel".to_string(), 2);
            chart.insert("fairy".to_string(), 2);
        }
        "water" => {
            chart.insert("fire".to_string(), 2);
            chart.insert("water".to_string(), 2);
            chart.insert("electric".to_string(), 1);
            chart.insert("grass".to_string(), 1);
            chart.insert("ice".to_string(), 2);
            chart.insert("steel".to_string(), 2);
        }
        "electric" => {
            chart.insert("electric".to_string(), 2);
            chart.insert("ground".to_string(), 1);
            chart.insert("flying".to_string(), 2);
            chart.insert("steel".to_string(), 2);
        }
        "grass" => {
            chart.insert("fire".to_string(), 1);
            chart.insert("water".to_string(), 2);
            chart.insert("electric".to_string(), 2);
            chart.insert("grass".to_string(), 2);
            chart.insert("ice".to_string(), 1);
            chart.insert("poison".to_string(), 1);
            chart.insert("ground".to_string(), 2);
            chart.insert("flying".to_string(), 1);
            chart.insert("bug".to_string(), 1);
        }
        "ice" => {
            chart.insert("fire".to_string(), 1);
            chart.insert("ice".to_string(), 2);
            chart.insert("fighting".to_string(), 1);
            chart.insert("rock".to_string(), 1);
            chart.insert("steel".to_string(), 1);
        }
        "fighting" => {
            chart.insert("flying".to_string(), 1);
            chart.insert("psychic".to_string(), 1);
            chart.insert("bug".to_string(), 2);
            chart.insert("rock".to_string(), 2);
            chart.insert("dark".to_string(), 2);
            chart.insert("fairy".to_string(), 1);
        }
        "poison" => {
            chart.insert("grass".to_string(), 2);
            chart.insert("fighting".to_string(), 2);
            chart.insert("poison".to_string(), 2);
            chart.insert("ground".to_string(), 1);
            chart.insert("psychic".to_string(), 1);
            chart.insert("bug".to_string(), 2);
            chart.insert("fairy".to_string(), 2);
        }
        "ground" => {
            chart.insert("water".to_string(), 1);
            chart.insert("electric".to_string(), 3);
            chart.insert("grass".to_string(), 1);
            chart.insert("ice".to_string(), 1);
            chart.insert("poison".to_string(), 2);
            chart.insert("rock".to_string(), 2);
        }
        "flying" => {
            chart.insert("electric".to_string(), 1);
            chart.insert("grass".to_string(), 2);
            chart.insert("ice".to_string(), 1);
            chart.insert("fighting".to_string(), 2);
            chart.insert("ground".to_string(), 3);
            chart.insert("bug".to_string(), 2);
            chart.insert("rock".to_string(), 1);
        }
        "psychic" => {
            chart.insert("fighting".to_string(), 2);
            chart.insert("psychic".to_string(), 2);
            chart.insert("bug".to_string(), 1);
            chart.insert("ghost".to_string(), 1);
            chart.insert("dark".to_string(), 1);
        }
        "bug" => {
            chart.insert("fire".to_string(), 1);
            chart.insert("grass".to_string(), 2);
            chart.insert("fighting".to_string(), 2);
            chart.insert("ground".to_string(), 2);
            chart.insert("flying".to_string(), 1);
            chart.insert("rock".to_string(), 1);
        }
        "rock" => {
            chart.insert("normal".to_string(), 2);
            chart.insert("fire".to_string(), 2);
            chart.insert("water".to_string(), 1);
            chart.insert("grass".to_string(), 1);
            chart.insert("fighting".to_string(), 1);
            chart.insert("poison".to_string(), 2);
            chart.insert("ground".to_string(), 1);
            chart.insert("flying".to_string(), 2);
            chart.insert("steel".to_string(), 1);
        }
        "ghost" => {
            chart.insert("normal".to_string(), 3);
            chart.insert("fighting".to_string(), 3);
            chart.insert("poison".to_string(), 2);
            chart.insert("bug".to_string(), 2);
            chart.insert("ghost".to_string(), 1);
            chart.insert("dark".to_string(), 1);
        }
        "dragon" => {
            chart.insert("fire".to_string(), 2);
            chart.insert("water".to_string(), 2);
            chart.insert("electric".to_string(), 2);
            chart.insert("grass".to_string(), 2);
            chart.insert("ice".to_string(), 1);
            chart.insert("dragon".to_string(), 1);
            chart.insert("fairy".to_string(), 1);
        }
        "dark" => {
            chart.insert("fighting".to_string(), 1);
            chart.insert("psychic".to_string(), 3);
            chart.insert("bug".to_string(), 1);
            chart.insert("ghost".to_string(), 2);
            chart.insert("dark".to_string(), 2);
            chart.insert("fairy".to_string(), 1);
        }
        "steel" => {
            chart.insert("normal".to_string(), 2);
            chart.insert("fire".to_string(), 1);
            chart.insert("grass".to_string(), 2);
            chart.insert("ice".to_string(), 2);
            chart.insert("fighting".to_string(), 1);
            chart.insert("poison".to_string(), 3);
            chart.insert("ground".to_string(), 1);
            chart.insert("flying".to_string(), 2);
            chart.insert("psychic".to_string(), 2);
            chart.insert("bug".to_string(), 2);
            chart.insert("rock".to_string(), 2);
            chart.insert("dragon".to_string(), 2);
            chart.insert("steel".to_string(), 2);
            chart.insert("fairy".to_string(), 2);
        }
        "fairy" => {
            chart.insert("fighting".to_string(), 2);
            chart.insert("poison".to_string(), 1);
            chart.insert("bug".to_string(), 2);
            chart.insert("dragon".to_string(), 3);
            chart.insert("dark".to_string(), 2);
            chart.insert("steel".to_string(), 1);
        }
        _ => return None,
    }

    Some(chart)
}

fn get_all_types(gen: u8) -> Vec<TypeInfo> {
    // IMPORTANT: This order must match JavaScript's TypeChart object key iteration order
    // which is alphabetical. This affects sample() results in moves like Conversion 2.
    let type_ids = [
        "bug", "dark", "dragon", "electric", "fairy", "fighting", "fire", "flying",
        "ghost", "grass", "ground", "ice", "normal", "poison", "psychic", "rock",
        "steel", "water",
    ];

    type_ids
        .iter()
        .filter_map(|id| {
            let name = capitalize_first(id);
            get_type_data(id, &name, gen)
        })
        .filter(|t| t.gen <= gen)
        .collect()
}
