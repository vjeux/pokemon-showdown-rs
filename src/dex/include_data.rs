// Implement includeData from JavaScript
//
// JS Source:
//
// 	includeData(): this {
// 		this.loadData();
// 		return this;
// 	}

use crate::*;

impl Dex {
    /// Include data - ensures data is loaded
    /// Equivalent to ModdedDex.includeData() in sim/dex.ts
    ///
    /// JavaScript source (sim/dex.ts):
    /// ```typescript
    /// includeData(): this {
    ///     this.loadData();
    ///     return this;
    /// }
    /// ```
    ///
    /// In Rust, data is loaded during Dex::new() or Dex::load_default(),
    /// so this method is essentially a no-op that returns self for chaining.
    pub fn include_data(&mut self) -> &mut Self {
        // JavaScript: this.loadData();
        // In Rust, loadData() is called in Dex::new() or Dex::load_default()
        // Data is already loaded at construction time, so this is a no-op

        // JavaScript: return this;
        self
    }
}
