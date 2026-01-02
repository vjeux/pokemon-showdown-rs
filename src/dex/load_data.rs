// TODO: Implement loadData from JavaScript
//
// JS Source:
// 	loadData(): DexTableData {
// 		if (this.dataCache) return this.dataCache;
// 		dexes['base'].includeMods();
// 		const dataCache: { [k in keyof DexTableData]?: any } = {};
// 
// 		const basePath = this.dataDir + '/';
// 
// 		const Scripts = this.loadDataFile(basePath, 'Scripts') || {};
// 		// We want to inherit most of Scripts but not this.
// 		const init = Scripts.init;
// 		this.parentMod = this.isBase ? '' : (Scripts.inherit || 'base');
// 
// 		let parentDex;
// 		if (this.parentMod) {
// 			parentDex = dexes[this.parentMod];
// 			if (!parentDex || parentDex === this) {
// 				throw new Error(
// 					`Unable to load ${this.currentMod}. 'inherit' in scripts.ts should specify a parent mod from which to inherit data, or must be not specified.`
// 				);
// 			}
// 		}
// 
// 		if (!parentDex) {
// 			// Formats are inherited by mods and used by Rulesets
// 			this.includeFormats();
// 		}
// 		for (const dataType of DATA_TYPES) {
// 			dataCache[dataType] = this.loadDataFile(basePath, dataType);
// 			if (dataType === 'Rulesets' && !parentDex) {
// 				for (const format of this.formats.all()) {
// 					dataCache.Rulesets[format.id] = { ...format, ruleTable: null };
// 				}
// 			}
// 		}
// 		if (parentDex) {
// 			for (const dataType of DATA_TYPES) {
// 				const parentTypedData: DexTable<any> = parentDex.data[dataType];
// 				if (!dataCache[dataType] && !init) {
// 					dataCache[dataType] = parentTypedData;
// 					continue;
// 				}
// 				const childTypedData: DexTable<any> = dataCache[dataType] || (dataCache[dataType] = {});
// 				for (const entryId in parentTypedData) {
// 					if (childTypedData[entryId] === null) {
// 						// null means don't inherit
// 						delete childTypedData[entryId];
// 					} else if (!(entryId in childTypedData)) {
// 						// If it doesn't exist it's inherited from the parent data
// 						childTypedData[entryId] = parentTypedData[entryId];
// 					} else if (childTypedData[entryId]?.inherit) {
// 						// {inherit: true} can be used to modify only parts of the parent data,
// 						// instead of overwriting entirely
// 						delete childTypedData[entryId].inherit;
// 
// 						// Merge parent and child's entry, with child overwriting parent.
// 						childTypedData[entryId] = { ...parentTypedData[entryId], ...childTypedData[entryId] };
// 					}
// 				}
// 			}
// 		}
// 
// 		// Flag the generation. Required for team validator.
// 		this.gen = dataCache.Scripts.gen;
// 		if (!this.gen) throw new Error(`Mod ${this.currentMod} needs a generation number in scripts.js`);
// 		this.dataCache = dataCache as DexTableData;
// 
// 		// Execute initialization script.
// 		if (init) init.call(this);
// 
// 		return this.dataCache;
// 	}

use crate::*;

impl Dex {
    // TODO: Implement this method
}
