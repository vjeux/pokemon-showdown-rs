// Implement loadAliases from JavaScript
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
// 		... (fuzzy alias generation code)
// 		return this.aliases!;
// 	}

use crate::*;
use std::collections::HashMap;

impl Dex {
    /// Load aliases - loads alias mappings for Pokemon, moves, items, abilities
    /// Equivalent to ModdedDex.loadAliases() in sim/dex.ts
    ///
    /// JavaScript source (sim/dex.ts):
    /// ```typescript
    /// loadAliases(): NonNullable<ModdedDex['aliases']> {
    ///     if (!this.isBase) return Dex.loadAliases();
    ///     if (this.aliases) return this.aliases;
    ///     const exported = require(path.resolve(DATA_DIR, 'aliases'));
    ///     const aliases = new Map<ID, ID>();
    ///     for (const [alias, target] of Object.entries(exported.Aliases)) {
    ///         aliases.set(alias as ID, toID(target));
    ///     }
    ///     const compoundNames = new Map<ID, string>();
    ///     for (const name of exported.CompoundWordNames) {
    ///         compoundNames.set(toID(name), name);
    ///     }
    ///     // ... (fuzzy alias generation)
    ///     return this.aliases!;
    /// }
    /// ```
    ///
    /// In Rust, aliases are already loaded from JSON during Dex::new() or Dex::load_default(),
    /// so this method just returns a reference to the existing aliases HashMap.
    /// The fuzzy alias generation logic is not yet implemented in Rust.
    pub fn load_aliases(&self) -> &HashMap<crate::dex_data::ID, String> {
        // JavaScript: if (!this.isBase) return Dex.loadAliases();
        // In Rust, we only have one Dex instance (no mod system yet)

        // JavaScript: if (this.aliases) return this.aliases;
        // In Rust, aliases are already loaded in constructor from aliases.json

        // JavaScript: const exported = require(path.resolve(DATA_DIR, 'aliases'));
        // JavaScript: const aliases = new Map<ID, ID>();
        // JavaScript: for (const [alias, target] of Object.entries(exported.Aliases)) {
        // JavaScript:     aliases.set(alias as ID, toID(target));
        // JavaScript: }
        // In Rust, this is done in load_from_json.rs

        // JavaScript: (fuzzy alias generation)
        // Not yet implemented in Rust - would generate acronyms, forme variations, etc.

        // JavaScript: return this.aliases!;
        &self.aliases
    }

    /// Generate fuzzy aliases for autocomplete/search
    /// This is part of JavaScript's loadAliases() but separated for clarity
    /// Not yet implemented - would generate:
    /// - Acronyms (e.g., "tb" for "Thunderbolt")
    /// - Forme variations (e.g., "alolanpikachu" for "Pikachu-Alola")
    /// - Partial names (e.g., "bolt" for "Thunderbolt")
    pub fn generate_fuzzy_aliases(&mut self) {
        // TODO: Implement fuzzy alias generation
        // JavaScript generates:
        // - Full acronyms: "tb" -> "thunderbolt"
        // - Acronym + last word: "tbolt" -> "thunderbolt"
        // - Forme prefixes: "alolan", "galarian", "hisuian", "paldean"
        // - Mega variations: "megapikachu", "pikachumega", "mpikachu", "pikachum"
        // - Word parts: each word in multi-word names

        // For now, this is a no-op
        // When implemented, would populate self.fuzzy_aliases HashMap
    }
}

