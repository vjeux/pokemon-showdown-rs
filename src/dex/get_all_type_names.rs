use crate::*;

impl Dex {

    // =========================================================================
    // Type-specific methods (from dex-data.ts)
    // =========================================================================

    /// Get all type names as an iterator
    /// Equivalent to this.dex.types.names() in conversion2.ts
    pub fn get_all_type_names(&self) -> impl Iterator<Item = &String> {
        self.types.keys()
    }
}
