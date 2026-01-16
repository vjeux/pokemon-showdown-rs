//! Global static Dex for performance
//!
//! This loads the Dex once on first access and caches it for all subsequent uses.
//! This is critical for performance - parsing ~2MB of JSON per battle is expensive.

use once_cell::sync::Lazy;
use super::Dex;
use super::DexJsonData;
use super::embedded;

/// Global static Dex that's loaded once on first access
pub static GLOBAL_DEX: Lazy<Dex> = Lazy::new(|| {
    Dex::load_from_json(DexJsonData {
        species_json: embedded::SPECIES_JSON,
        moves_json: embedded::MOVES_JSON,
        abilities_json: embedded::ABILITIES_JSON,
        items_json: embedded::ITEMS_JSON,
        conditions_json: embedded::CONDITIONS_JSON,
        types_json: embedded::TYPES_JSON,
        natures_json: embedded::NATURES_JSON,
        rulesets_json: embedded::RULESETS_JSON,
        aliases_json: embedded::ALIASES_JSON,
        compound_word_names_json: embedded::COMPOUNDWORDNAMES_JSON,
        formats_json: embedded::FORMATS_JSON,
        formats_data_json: embedded::FORMATS_DATA_JSON,
    }).expect("Failed to load embedded Dex data")
});

impl Dex {
    /// Get a clone of the global Dex (cached, fast)
    ///
    /// This is the preferred way to get a Dex for battles.
    /// The Dex data is parsed once on first access and cloned thereafter.
    pub fn global() -> Self {
        GLOBAL_DEX.clone()
    }
}
