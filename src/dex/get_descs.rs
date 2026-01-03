// Implement getDescs from JavaScript
//
// JS Source:
//
// 	getDescs(table: keyof TextTableData, id: ID, dataEntry: AnyObject) {
// 		if (dataEntry.shortDesc) {
// 			return {
// 				desc: dataEntry.desc,
// 				shortDesc: dataEntry.shortDesc,
// 			};
// 		}
// 		const entry = this.loadTextData()[table][id];
// 		if (!entry) return null;
// 		const descs = {
// 			desc: '',
// 			shortDesc: '',
// 		};
// 		for (let i = this.gen; i < dexes['base'].gen; i++) {
// 			const curDesc = entry[`gen${i}` as keyof typeof entry]?.desc;
// 			const curShortDesc = entry[`gen${i}` as keyof typeof entry]?.shortDesc;
// 			if (!descs.desc && curDesc) {
// 				descs.desc = curDesc;
// 			}
// 			if (!descs.shortDesc && curShortDesc) {
// 				descs.shortDesc = curShortDesc;
// 			}
// 			if (descs.desc && descs.shortDesc) break;
// 		}
// 		if (!descs.shortDesc) descs.shortDesc = entry.shortDesc || '';
// 		if (!descs.desc) descs.desc = entry.desc || descs.shortDesc;
// 		return descs;
// 	}

use crate::*;

/// Description returned from get_descs
#[derive(Debug, Clone)]
pub struct Descriptions {
    pub desc: String,
    pub short_desc: String,
}

impl Dex {
    /// Get descriptions for an entry
    /// Equivalent to ModdedDex.getDescs() in sim/dex.ts
    ///
    /// JavaScript source (sim/dex.ts):
    /// ```typescript
    /// getDescs(table: keyof TextTableData, id: ID, dataEntry: AnyObject) {
    ///     if (dataEntry.shortDesc) {
    ///         return { desc: dataEntry.desc, shortDesc: dataEntry.shortDesc };
    ///     }
    ///     const entry = this.loadTextData()[table][id];
    ///     if (!entry) return null;
    ///     // ... (generation-specific description lookup)
    ///     return descs;
    /// }
    /// ```
    ///
    /// This method is not currently needed because description data is already
    /// loaded in the JSON files. If text-only description files are added later,
    /// this method would need to be fully implemented.
    pub fn get_descs(
        &self,
        _table: &str,
        _id: &crate::dex_data::ID,
        short_desc: Option<&str>,
        desc: Option<&str>,
    ) -> Option<Descriptions> {
        // JavaScript: if (dataEntry.shortDesc) return { desc, shortDesc };
        if let Some(sd) = short_desc {
            return Some(Descriptions {
                desc: desc.unwrap_or(sd).to_string(),
                short_desc: sd.to_string(),
            });
        }

        // JavaScript: const entry = this.loadTextData()[table][id];
        // JavaScript: if (!entry) return null;
        // In Rust, we don't have separate text data files yet,
        // descriptions come directly from JSON data
        // Return None to indicate no text-only descriptions available
        None
    }
}

