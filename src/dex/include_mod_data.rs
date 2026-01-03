// Implement includeModData from JavaScript
//
// JS Source:
//
// 	includeModData(): this {
// 		for (const mod in this.dexes) {
// 			dexes[mod].includeData();
// 		}
// 		return this;
// 	}

use crate::*;

impl Dex {
    /// Include mod data - ensures all mod data is loaded
    /// Equivalent to ModdedDex.includeModData() in sim/dex.ts
    ///
    /// JavaScript source (sim/dex.ts):
    /// ```typescript
    /// includeModData(): this {
    ///     for (const mod in this.dexes) {
    ///         dexes[mod].includeData();
    ///     }
    ///     return this;
    /// }
    /// ```
    ///
    /// In Rust, we don't currently support mods (gen8ou, etc.),
    /// so this is a no-op that just returns self for chaining.
    /// When mod support is added, this would iterate through mod dexes
    /// and call include_data() on each.
    pub fn include_mod_data(&mut self) -> &mut Self {
        // JavaScript: for (const mod in this.dexes) { dexes[mod].includeData(); }
        // In Rust, mod support not yet implemented
        // When implemented, would iterate self.dexes and call include_data()

        // JavaScript: return this;
        self
    }
}
