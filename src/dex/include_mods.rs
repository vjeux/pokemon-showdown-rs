// TODO: Implement includeMods from JavaScript
//
// JS Source:
// 
// 	includeMods(): this {
// 		if (!this.isBase) throw new Error(`This must be called on the base Dex`);
// 		if (this.modsLoaded) return this;
// 
// 		for (const mod of fs.readdirSync(MODS_DIR)) {
// 			dexes[mod] = new ModdedDex(mod);
// 		}
// 		this.modsLoaded = true;
// 
// 		return this;
// 	}

use crate::*;

impl Dex {
    // TODO: Implement this method
}
