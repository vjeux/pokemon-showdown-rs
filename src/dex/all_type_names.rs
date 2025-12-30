use crate::*;

impl Dex {

    /// Get all type names
    /// Equivalent to DexTypes.names() in dex-data.ts
    pub fn all_type_names(&self) -> Vec<&String> {
        self.types.keys().collect()
    }
}
