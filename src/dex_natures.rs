//! DexNatures - Natures lookup helper
//!
//! Equivalent to DexNatures class in dex-data.ts

use crate::dex::{Dex, NatureData};
use crate::dex_data::ID;

/// Helper struct for nature lookups
/// JavaScript equivalent: DexNatures (sim/dex-data.ts)
/// 1 field in JavaScript (dex)
pub struct DexNatures<'a> {
    /// Dex reference
    /// JavaScript: readonly dex: ModdedDex
    pub(crate) dex: &'a Dex,
}

impl<'a> DexNatures<'a> {
    /// Get nature data by name or ID
    /// Equivalent to DexNatures.get() in dex-data.ts
    pub fn get(&self, name: &str) -> Option<&'a NatureData> {
        let id = ID::new(name);
        self.dex.natures.get(&id)
    }

    /// Get all natures data
    /// Equivalent to DexNatures.all() in dex-data.ts
    pub fn all(&self) -> Vec<&'a NatureData> {
        self.dex.natures.values().collect()
    }
}
