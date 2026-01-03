// Dex::getAlias - Get canonical name for an alias
//
// JS Source:
//
// 	getAlias(id: ID): ID | undefined {
// 		return this.loadAliases().get(id);
// 	}

use crate::*;

impl Dex {
    /// Get the canonical name for an alias
    /// Equivalent to getAlias() in dex.ts
    ///
    /// JavaScript (dex.ts):
    ///   getAlias(id: ID): ID | undefined {
    ///     return this.loadAliases().get(id);
    ///   }
    pub fn get_alias(&self, id: &ID) -> Option<String> {
        // return this.loadAliases().get(id);
        self.aliases.get(id).cloned()
    }
}
