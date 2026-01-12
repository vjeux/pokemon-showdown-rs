//! DexSpecies - Species lookup helper
//!
//! Equivalent to DexSpecies class in dex-species.ts

use crate::dex::{Dex, SpeciesData};
use crate::dex_data::ID;

/// Helper struct for species lookups
/// JavaScript equivalent: DexSpecies (sim/dex-species.ts)
/// 1 field in JavaScript (dex)
pub struct DexSpecies<'a> {
    /// Dex reference
    /// JavaScript: readonly dex: ModdedDex
    pub(crate) dex: &'a Dex,
}

impl<'a> DexSpecies<'a> {
    /// Get species data by name or ID
    /// Equivalent to DexSpecies.get() in dex-species.ts
    // get(name?: string | Species): Species {
    //     if (name && typeof name !== 'string') return name;
    //
    //     let id = '' as ID;
    //     if (name) {
    //         name = name.trim();
    //         id = toID(name);
    //         if (id === 'nidoran' && name.endsWith('♀')) {
    //             id = 'nidoranf' as ID;
    //         } else if (id === 'nidoran' && name.endsWith('♂')) {
    //             id = 'nidoranm' as ID;
    //         }
    //     }
    //     return this.getByID(id);
    // }
    pub fn get(&self, name: &str) -> Option<&'a SpeciesData> {
        let id = ID::new(name);

        // Try direct lookup first
        if let Some(species) = self.dex.species.get(&id) {
            // JS: if (this.dex.data.Pokedex?.[id]?.isCosmeticForme)
            // If this is a cosmetic forme with no stats, we need to get base species
            // In JavaScript, this is handled by spreading base species then cosmetic forme
            // We detect this by checking if baseStats are all zero
            if species.is_cosmetic_forme ||
               (species.base_stats.hp == 0 && species.base_stats.atk == 0 &&
                species.base_stats.def == 0 && species.base_stats.spa == 0 &&
                species.base_stats.spd == 0 && species.base_stats.spe == 0) {
                // JS: species = this.get(alias);
                // Get the base species (via baseSpecies field or alias)
                if let Some(ref base_species_name) = species.base_species {
                    if let Some(base_species) = self.get(base_species_name) {
                        // JavaScript merges: { ...baseSpecies, ...cosmeticForme }
                        // But in Rust we can't mutate the returned reference
                        // The merge happens during JSON loading, not at lookup time
                        // So we just return the base species data
                        return Some(base_species);
                    }
                }
            }
            return Some(species);
        }

        // Try alias lookup
        if let Some(canonical_name) = self.dex.aliases.get(&id) {
            let canonical_id = ID::new(canonical_name);
            return self.dex.species.get(&canonical_id);
        }
        None
    }

    /// Get species by ID
    /// Equivalent to DexSpecies.getByID() in dex-species.ts
    // getByID(id: ID): Species {
    //     if (id === '') return EMPTY_SPECIES;
    //     let species: Mutable<Species> | undefined = this.speciesCache.get(id);
    //     if (species) return species;

    //     const alias = this.dex.getAlias(id);
    //     if (alias) {
    //         if (this.dex.data.FormatsData.hasOwnProperty(id)) {
    //             // special event ID
    //             species = new Species({
    //                 ...this.dex.data.Pokedex[alias],
    //                 ...this.dex.data.FormatsData[id],
    //                 name: id,
    //             });
    //             species.abilities = { 0: species.abilities['S']! };
    //         } else {
    //             species = this.get(alias);
    //             if (this.dex.data.Pokedex?.[id]?.isCosmeticForme) {
    //                 const cosmeticForme = this.dex.data.Pokedex[id];
    //                 species = new Species({
    //                     ...species,
    //                     ...cosmeticForme,
    //                     name: species.baseSpecies + '-' + cosmeticForme.forme!, // Forme always exists on cosmetic forme entries
    //                     baseForme: "",
    //                     otherFormes: null,
    //                     cosmeticFormes: null,
    //                 });
    //             }
    //             if (species.cosmeticFormes) {
    //                 for (const forme of species.cosmeticFormes) {
    //                     if (this.dex.data.Pokedex.hasOwnProperty(toID(forme))) continue;
    //                     if (toID(forme) === id) {
    //                         species = new Species({
    //                             ...species,
    //                             name: forme,
    //                             forme: forme.slice(species.name.length + 1),
    //                             baseSpecies: species.name,
    //                             baseForme: "",
    //                             isCosmeticForme: true,
    //                             otherFormes: null,
    //                             cosmeticFormes: null,
    //                         });
    //                         break;
    //                     }
    //                 }
    //             }
    //         }
    //         this.speciesCache.set(id, this.dex.deepFreeze(species));
    //         return species;
    //     }
    //
    //     if (!this.dex.data.Pokedex.hasOwnProperty(id)) {
    //         let aliasTo = '';
    //         const formeNames: { [k: IDEntry]: IDEntry[] } = {
    //             alola: ['a', 'alola', 'alolan'],
    //             galar: ['g', 'galar', 'galarian'],
    //             hisui: ['h', 'hisui', 'hisuian'],
    //             paldea: ['p', 'paldea', 'paldean'],
    //             mega: ['m', 'mega'],
    //             primal: ['p', 'primal'],
    //         };
    //         for (const forme in formeNames) {
    //             let pokeName = '';
    //             for (const i of formeNames[forme as ID]) {
    //                 if (id.startsWith(i)) {
    //                     pokeName = id.slice(i.length);
    //                 } else if (id.endsWith(i)) {
    //                     pokeName = id.slice(0, -i.length);
    //                 }
    //             }
    //             pokeName = this.dex.getAlias(pokeName as ID) || pokeName;
    //             if (this.dex.data.Pokedex[pokeName + forme]) {
    //                 aliasTo = pokeName + forme;
    //                 break;
    //             }
    //         }
    //         if (aliasTo) {
    //             species = this.get(aliasTo);
    //             if (species.exists) {
    //                 this.speciesCache.set(id, species);
    //                 return species;
    //             }
    //         }
    //     }
    //     if (id && this.dex.data.Pokedex.hasOwnProperty(id)) {
    //         const pokedexData = this.dex.data.Pokedex[id];
    //         const baseSpeciesTags = pokedexData.baseSpecies && this.dex.data.Pokedex[toID(pokedexData.baseSpecies)].tags;
    //         species = new Species({
    //             tags: baseSpeciesTags,
    //             ...pokedexData,
    //             ...this.dex.data.FormatsData[id],
    //         });
    //         // Inherit any statuses from the base species (Arceus, Silvally).
    //         const baseSpeciesStatuses = this.dex.data.Conditions[toID(species.baseSpecies)];
    //         if (baseSpeciesStatuses !== undefined) {
    //             for (const key in baseSpeciesStatuses) {
    //                 if (!(key in species)) {
    //                     (species as any)[key] = (baseSpeciesStatuses as any)[key];
    //                 }
    //             }
    //         }
    //         if (!species.tier && !species.doublesTier && !species.natDexTier && species.baseSpecies !== species.name) {
    //             if (species.baseSpecies === 'Mimikyu') {
    //                 species.tier = this.dex.data.FormatsData[toID(species.baseSpecies)].tier || 'Illegal';
    //                 species.doublesTier = this.dex.data.FormatsData[toID(species.baseSpecies)].doublesTier || species.tier as any;
    //                 species.natDexTier = this.dex.data.FormatsData[toID(species.baseSpecies)].natDexTier || species.tier;
    //             } else if (species.id.endsWith('totem')) {
    //                 species.tier = this.dex.data.FormatsData[species.id.slice(0, -5)].tier || 'Illegal';
    //                 species.doublesTier = this.dex.data.FormatsData[species.id.slice(0, -5)].doublesTier || species.tier as any;
    //                 species.natDexTier = this.dex.data.FormatsData[species.id.slice(0, -5)].natDexTier || species.tier;
    //             } else if (species.battleOnly) {
    //                 species.tier = this.dex.data.FormatsData[toID(species.battleOnly)]?.tier || 'Illegal';
    //                 species.doublesTier = this.dex.data.FormatsData[toID(species.battleOnly)]?.doublesTier || species.tier as any;
    //                 species.natDexTier = this.dex.data.FormatsData[toID(species.battleOnly)]?.natDexTier || species.tier;
    //             } else {
    //                 const baseFormatsData = this.dex.data.FormatsData[toID(species.baseSpecies)];
    //                 if (!baseFormatsData) {
    //                     throw new Error(`${species.baseSpecies} has no formats-data entry`);
    //                 }
    //                 species.tier = baseFormatsData.tier || 'Illegal';
    //                 species.doublesTier = baseFormatsData.doublesTier || species.tier as any;
    //                 species.natDexTier = baseFormatsData.natDexTier || species.tier;
    //             }
    //         }
    //         if (!species.tier) species.tier = 'Illegal';
    //         if (!species.doublesTier) species.doublesTier = species.tier as any;
    //         if (!species.natDexTier) species.natDexTier = species.tier;
    //         if (species.gen > this.dex.gen) {
    //             species.tier = 'Illegal';
    //             species.doublesTier = 'Illegal';
    //             species.natDexTier = 'Illegal';
    //             species.isNonstandard = 'Future';
    //         }
    //         if (this.dex.currentMod === 'gen7letsgo' && !species.isNonstandard) {
    //             const isLetsGo = (
    //                 species.gen <= 7 && (species.num <= 151 || ['Meltan', 'Melmetal'].includes(species.name)) &&
    //                 (!species.forme || species.isMega || (['Alola', 'Starter'].includes(species.forme) &&
    //                     species.name !== 'Pikachu-Alola'))
    //             );
    //             if (!isLetsGo) species.isNonstandard = 'Past';
    //         }
    //         if (this.dex.currentMod === 'gen8bdsp' &&
    //             (!species.isNonstandard || ["Gigantamax", "CAP"].includes(species.isNonstandard))) {
    //             if (species.gen > 4 || (species.num < 1 && species.isNonstandard !== 'CAP') ||
    //                 species.id === 'pichuspikyeared') {
    //                 species.isNonstandard = 'Future';
    //                 species.tier = species.doublesTier = species.natDexTier = 'Illegal';
    //             }
    //         }
    //         species.nfe = species.evos.some(evo => {
    //             const evoSpecies = this.get(evo);
    //             return !evoSpecies.isNonstandard ||
    //                 evoSpecies.isNonstandard === species?.isNonstandard ||
    //                 // Pokemon with Hisui evolutions
    //                 evoSpecies.isNonstandard === "Unobtainable";
    //         });
    //         species.canHatch = species.canHatch ||
    //             (!['Ditto', 'Undiscovered'].includes(species.eggGroups[0]) && !species.prevo && species.name !== 'Manaphy');
    //         if (this.dex.gen === 1) species.bst -= species.baseStats.spd;
    //         if (this.dex.gen < 5) {
    //             species.abilities = this.dex.deepClone(species.abilities);
    //             delete species.abilities['H'];
    //         }
    //         if (this.dex.gen === 3 && this.dex.abilities.get(species.abilities['1']).gen === 4) delete species.abilities['1'];
    //
    //         if (this.dex.parentMod) {
    //             // if this species is exactly identical to parentMod's species, reuse parentMod's copy
    //             const parentMod = this.dex.mod(this.dex.parentMod);
    //             if (this.dex.data.Pokedex[id] === parentMod.data.Pokedex[id]) {
    //                 const parentSpecies = parentMod.species.getByID(id);
    //                 // checking tier cheaply filters out some non-matches.
    //                 // The construction logic is very complex so we ultimately need to do a deep equality check
    //                 if (species.tier === parentSpecies.tier && isDeepStrictEqual(species, parentSpecies)) {
    //                     species = parentSpecies;
    //                 }
    //             }
    //         }
    //     } else {
    //         species = new Species({
    //             id, name: id,
    //             exists: false, tier: 'Illegal', doublesTier: 'Illegal', natDexTier: 'Illegal', isNonstandard: 'Custom',
    //         });
    //     }
    //     if (species.exists) this.speciesCache.set(id, this.dex.deepFreeze(species));
    //     return species;
    // }
    pub fn get_by_id(&self, id: &ID) -> Option<&'a SpeciesData> {
        // Use get() to ensure cosmetic forme fallback is applied
        self.get(id.as_str())
    }

    /// Get all species data
    /// Equivalent to DexSpecies.all() in dex-species.ts
    // all(): readonly Species[] {
    //     if (this.allCache) return this.allCache;
    //     const species = [];
    //     for (const id in this.dex.data.Pokedex) {
    //         species.push(this.getByID(id as ID));
    //     }
    //     this.allCache = Object.freeze(species);
    //     return this.allCache;
    // }
    pub fn all(&self) -> Vec<&'a SpeciesData> {
        self.dex.species.values().collect()
    }
}
