// TODO: Implement loadAliases from JavaScript
//
// JS Source:
// 
// 	loadAliases(): NonNullable<ModdedDex['aliases']> {
// 		if (!this.isBase) return Dex.loadAliases();
// 		if (this.aliases) return this.aliases;
// 		const exported = require(path.resolve(DATA_DIR, 'aliases'));
// 		const aliases = new Map<ID, ID>();
// 		for (const [alias, target] of Object.entries(exported.Aliases)) {
// 			aliases.set(alias as ID, toID(target));
// 		}
// 		const compoundNames = new Map<ID, string>();
// 		for (const name of exported.CompoundWordNames) {
// 			compoundNames.set(toID(name), name);
// 		}
// 
// 		const fuzzyAliases = new Map<ID, ID[]>();
// 		const addFuzzy = (alias: ID, target: ID) => {
// 			if (alias === target) return;
// 			if (alias.length < 2) return;
// 			const prev = fuzzyAliases.get(alias) || [];
// 			if (!prev.includes(target)) prev.push(target);
// 			fuzzyAliases.set(alias, prev);
// 		};
// 		const addFuzzyForme = (alias: ID, target: ID, forme: ID, formeLetter: ID) => {
// 			addFuzzy(`${alias}${forme}` as ID, target);
// 			if (!forme) return;
// 			addFuzzy(`${alias}${formeLetter}` as ID, target);
// 			addFuzzy(`${formeLetter}${alias}` as ID, target);
// 			if (forme === 'alola') addFuzzy(`alolan${alias}` as ID, target);
// 			else if (forme === 'galar') addFuzzy(`galarian${alias}` as ID, target);
// 			else if (forme === 'hisui') addFuzzy(`hisuian${alias}` as ID, target);
// 			else if (forme === 'paldea') addFuzzy(`paldean${alias}` as ID, target);
// 			else if (forme === 'megax') addFuzzy(`mega${alias}x` as ID, target);
// 			else if (forme === 'megay') addFuzzy(`mega${alias}y` as ID, target);
// 			else addFuzzy(`${forme}${alias}` as ID, target);
// 
// 			if (forme === 'megax' || forme === 'megay') {
// 				addFuzzy(`mega${alias}` as ID, target);
// 				addFuzzy(`${alias}mega` as ID, target);
// 				addFuzzy(`m${alias}` as ID, target);
// 				addFuzzy(`${alias}m` as ID, target);
// 			}
// 		};
// 		for (const table of ['Items', 'Abilities', 'Moves', 'Pokedex'] as const) {
// 			const data = this.data[table];
// 			for (const [id, entry] of Object.entries(data) as [ID, DexTableData[typeof table][string]][]) {
// 				let name = compoundNames.get(id) || entry.name;
// 				let forme = '' as ID;
// 				let formeLetter = '' as ID;
// 				if (name.includes('(')) {
// 					addFuzzy(toID(name.split('(')[0]), id);
// 				}
// 				if (table === 'Pokedex') {
// 					// can't Dex.species.get; aliases isn't loaded
// 					const species = entry as DexTableData['Pokedex'][string];
// 					const baseid = toID(species.baseSpecies);
// 					if (baseid && baseid !== id) {
// 						name = compoundNames.get(baseid) || baseid;
// 					}
// 					forme = toID(species.forme || species.baseForme);
// 					if (forme === 'fan') {
// 						formeLetter = 's' as ID;
// 					} else if (forme === 'bloodmoon') {
// 						formeLetter = 'bm' as ID;
// 					} else {
// 						// not doing baseForme as a hack to make aliases point to base forme
// 						formeLetter = (species.forme || '').split(/ |-/).map(part => toID(part).charAt(0)).join('') as ID;
// 					}
// 					addFuzzy(forme, id);
// 				}
// 
// 				addFuzzyForme(toID(name), id, forme, formeLetter);
// 				const fullSplit = name.split(/ |-/).map(toID);
// 				if (fullSplit.length < 2) continue;
// 				const fullAcronym = fullSplit.map(x => x.charAt(0)).join('');
// 				addFuzzyForme(fullAcronym as ID, id, forme, formeLetter);
// 				const fullAcronymWord = fullAcronym + fullSplit[fullSplit.length - 1].slice(1);
// 				addFuzzyForme(fullAcronymWord as ID, id, forme, formeLetter);
// 				for (const wordPart of fullSplit) addFuzzyForme(wordPart, id, forme, formeLetter);
// 
// 				const spaceSplit = name.split(' ').map(toID);
// 				if (spaceSplit.length !== fullSplit.length) {
// 					const spaceAcronym = spaceSplit.map(x => x.charAt(0)).join('');
// 					addFuzzyForme(spaceAcronym as ID, id, forme, formeLetter);
// 					const spaceAcronymWord = spaceAcronym + spaceSplit[spaceSplit.length - 1].slice(1);
// 					addFuzzyForme(spaceAcronymWord as ID, id, forme, formeLetter);
// 					for (const word of fullSplit) addFuzzyForme(word, id, forme, formeLetter);
// 				}
// 			}
// 		}
// 
// 		(this as any).aliases = aliases satisfies this['aliases'];
// 		(this as any).fuzzyAliases = fuzzyAliases satisfies this['fuzzyAliases'];
// 		return this.aliases!;
// 	}

use crate::*;

impl Dex {
    // TODO: Implement this method
}
