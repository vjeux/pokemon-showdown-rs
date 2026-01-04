// NOTE: This method is NOT in JavaScript - Rust-specific implementation

use crate::*;
use crate::dex::AbilityData;
use crate::dex::ConditionData;
use crate::dex::DexJsonData;
use crate::dex::FormatData;
use crate::dex::ItemData;
use crate::dex::MoveData;
use crate::dex::NatureData;
use crate::dex::RulesetData;
use crate::dex::SpeciesData;
use crate::dex::TypeData;
use std::collections::HashMap;
use serde::Deserialize;

impl Dex {

    /// Load data from JSON strings
    pub fn load_from_json(json_data: DexJsonData) -> Result<Self, serde_json::Error> {
        let mut species_raw: HashMap<String, SpeciesData> =
            serde_json::from_str(json_data.species_json)
                .map_err(|e| {
                    eprintln!("Failed to parse species.json: {}", e);
                    e
                })?;
        let moves_raw: HashMap<String, MoveData> = serde_json::from_str(json_data.moves_json)
            .map_err(|e| {
                eprintln!("Failed to parse moves.json: {}", e);
                e
            })?;
        let abilities_raw: HashMap<String, AbilityData> =
            serde_json::from_str(json_data.abilities_json)
                .map_err(|e| {
                    eprintln!("Failed to parse abilities.json: {}", e);
                    e
                })?;
        let items_raw: HashMap<String, ItemData> = serde_json::from_str(json_data.items_json)
            .map_err(|e| {
                eprintln!("Failed to parse items.json: {}", e);
                e
            })?;
        let conditions_raw: HashMap<String, ConditionData> = serde_json::from_str(json_data.conditions_json)
            .map_err(|e| {
                eprintln!("Failed to parse conditions.json: {}", e);
                e
            })?;
        let types: HashMap<String, TypeData> = serde_json::from_str(json_data.types_json)
            .map_err(|e| {
                eprintln!("Failed to parse typechart.json: {}", e);
                e
            })?;
        let natures_raw: HashMap<String, NatureData> =
            serde_json::from_str(json_data.natures_json)
                .map_err(|e| {
                    eprintln!("Failed to parse natures.json: {}", e);
                    e
                })?;
        let rulesets_raw: HashMap<String, RulesetData> =
            serde_json::from_str(json_data.rulesets_json)
                .map_err(|e| {
                    eprintln!("Failed to parse rulesets.json: {}", e);
                    e
                })?;
        let aliases_raw: HashMap<String, String> = serde_json::from_str(json_data.aliases_json)
            .map_err(|e| {
                eprintln!("Failed to parse aliases.json: {}", e);
                e
            })?;
        let compound_word_names: Vec<String> =
            serde_json::from_str(json_data.compound_word_names_json)
                .map_err(|e| {
                    eprintln!("Failed to parse compoundwordnames.json: {}", e);
                    e
                })?;
        let formats: Vec<FormatData> = serde_json::from_str(json_data.formats_json)
            .map_err(|e| {
                eprintln!("Failed to parse formats.json: {}", e);
                e
            })?;

        // Load formats data and merge into species
        #[derive(Deserialize)]
        struct FormatsDataEntry {
            #[serde(default)]
            tier: Option<String>,
            #[serde(rename = "doublesTier", default)]
            doubles_tier: Option<String>,
            #[serde(rename = "natDexTier", default)]
            nat_dex_tier: Option<String>,
            #[serde(rename = "isNonstandard", default)]
            is_nonstandard: Option<String>,
        }
        let formats_data: HashMap<String, FormatsDataEntry> =
            serde_json::from_str(json_data.formats_data_json)?;

        // Merge formats data into species data
        for (species_id, formats_entry) in formats_data {
            if let Some(species) = species_raw.get_mut(&species_id) {
                species.tier = formats_entry.tier;
                species.doubles_tier = formats_entry.doubles_tier;
                species.nat_dex_tier = formats_entry.nat_dex_tier;
                species.is_nonstandard = formats_entry.is_nonstandard;
            }
        }

        // Convert string keys to ID keys
        let species: HashMap<ID, SpeciesData> = species_raw
            .into_iter()
            .map(|(k, v)| (ID::new(&k), v))
            .collect();

        // Generate cosmetic forme entries
        // DISABLED: JavaScript doesn't auto-generate cosmetic forme entries,
        // so we skip this to maintain parity for team generation
        /*
        // Collect species with cosmetic formes first to avoid borrow issues
        let species_with_cosmetic_formes: Vec<(ID, SpeciesData)> = species
            .iter()
            .filter(|(_, s)| !s.cosmetic_formes.is_empty())
            .map(|(id, s)| (id.clone(), s.clone()))
            .collect();

        for (_base_id, base_species) in species_with_cosmetic_formes {
            for forme_name in &base_species.cosmetic_formes {
                let forme_id = ID::new(forme_name);
                // Only create entry if it doesn't already exist
                if let std::collections::hash_map::Entry::Vacant(e) = species.entry(forme_id) {
                    // Extract forme suffix (e.g., "Yellow" from "Flabébé-Yellow")
                    let forme_suffix = if let Some(pos) = forme_name.rfind('-') {
                        &forme_name[pos + 1..]
                    } else {
                        forme_name.as_str()
                    };

                    // Normalize names to NFC (precomposed) form for consistency
                    use unicode_normalization::UnicodeNormalization;
                    let normalized_name: String = forme_name.nfc().collect();
                    let normalized_base_species: String = base_species.name.nfc().collect();

                    let cosmetic_forme = SpeciesData {
                        num: base_species.num,
                        name: normalized_name,
                        types: base_species.types.clone(),
                        base_stats: base_species.base_stats.clone(),
                        abilities: base_species.abilities.clone(),
                        heightm: base_species.heightm,
                        weightkg: base_species.weightkg,
                        gender: base_species.gender.clone(),
                        gender_ratio: base_species.gender_ratio.clone(),
                        evos: base_species.evos.clone(),
                        prevo: base_species.prevo.clone(),
                        evo_level: base_species.evo_level,
                        base_species: Some(normalized_base_species),
                        forme: Some(forme_suffix.to_string()),
                        other_formes: Vec::new(),
                        cosmetic_formes: Vec::new(),
                        is_cosmetic_forme: true,
                        gen: base_species.gen,
                        tags: base_species.tags.clone(),
                        egg_groups: base_species.egg_groups.clone(),
                        battle_only: base_species.battle_only.clone(),
                        forme_order: base_species.forme_order.clone(),
                        required_items: base_species.required_items.clone(),
                        tier: base_species.tier.clone(),
                        doubles_tier: base_species.doubles_tier.clone(),
                        nat_dex_tier: base_species.nat_dex_tier.clone(),
                        is_nonstandard: base_species.is_nonstandard.clone(),
                        exists: base_species.exists,
                    };

                    e.insert(cosmetic_forme);
                }
            }
        }
        */

        let moves = moves_raw
            .into_iter()
            .map(|(k, mut v)| {
                v.id = ID::new(&k);
                (v.id.clone(), v)
            })
            .collect();
        let abilities = abilities_raw
            .into_iter()
            .map(|(k, v)| (ID::new(&k), v))
            .collect();
        let items = items_raw
            .into_iter()
            .map(|(k, v)| (ID::new(&k), v))
            .collect();
        let natures = natures_raw
            .into_iter()
            .map(|(k, v)| (ID::new(&k), v))
            .collect();
        let rulesets = rulesets_raw
            .into_iter()
            .map(|(k, v)| (ID::new(&k), v))
            .collect();
        let aliases = aliases_raw
            .into_iter()
            .map(|(k, v)| (ID::new(&k), v))
            .collect();
        let conditions = conditions_raw
            .into_iter()
            .map(|(k, v)| (ID::new(&k), v))
            .collect();

        Ok(Self {
            species,
            moves,
            abilities,
            items,
            conditions,
            types,
            natures,
            rulesets,
            aliases,
            compound_word_names,
            formats,
            gen: 9, // Default to gen 9
        })
    }
}
