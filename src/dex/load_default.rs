use crate::*;
use crate::dex::DexJsonData;
use super::embedded;

impl Dex {
    /// Load the embedded default data
    pub fn load_default() -> Result<Self, serde_json::Error> {
        Self::load_from_json(DexJsonData {
            species_json: embedded::SPECIES_JSON,
            moves_json: embedded::MOVES_JSON,
            abilities_json: embedded::ABILITIES_JSON,
            items_json: embedded::ITEMS_JSON,
            types_json: embedded::TYPES_JSON,
            natures_json: embedded::NATURES_JSON,
            rulesets_json: embedded::RULESETS_JSON,
            aliases_json: embedded::ALIASES_JSON,
            compound_word_names_json: embedded::COMPOUNDWORDNAMES_JSON,
            formats_json: embedded::FORMATS_JSON,
            formats_data_json: embedded::FORMATS_DATA_JSON,
        })
    }
}
