use crate::*;
use super::embedded;

impl Dex {

    /// Create a Dex for a specific generation
    /// Equivalent to forGen() in dex.ts
    //
    // 	forGen(gen: number) {
    // 		if (!gen) return this;
    // 		return this.mod(`gen${gen}`);
    // 	}
    //
    pub fn for_gen(gen: u8) -> Result<Self, serde_json::Error> {
        let mut dex = Self::load_default()?;
        dex.gen = gen;

        // Load Gen 8-specific unavailable Pokemon list
        // Gen 8 (Sword/Shield) had a limited Pokedex ("Dexit")
        let gen8_past_ids: std::collections::HashSet<ID> = if gen == 8 {
            let gen8_past: Vec<String> = serde_json::from_str(embedded::GEN8_PAST_JSON)?;
            gen8_past.into_iter().map(|s| ID::new(&s)).collect()
        } else {
            std::collections::HashSet::new()
        };

        // JavaScript: Dex.forGen() uses mod system where gen1/pokedex.js, gen2/pokedex.js, etc.
        // physically don't include formes from future generations
        // We replicate this by pattern-matching forme names and marking as Future

        // Generation ranges based on national dex numbers:
        // Gen 1: 1-151, Gen 2: 152-251, Gen 3: 252-386, Gen 4: 387-493,
        // Gen 5: 494-649, Gen 6: 650-721, Gen 7: 722-809, Gen 8: complex, Gen 9: 906+
        let max_num = match gen {
            1 => 151,
            2 => 251,
            3 => 386,
            4 => 493,
            5 => 649,
            6 => 721,
            7 => 809,
            8 => 905,  // Enamorus is 905
            9 => 1025, // Current max
            _ => 1025,
        };

        for (species_id, species) in dex.species.iter_mut() {
            // Gen 8-specific: Check if this Pokemon is unavailable in Gen 8
            let gen8_unavailable = gen == 8 && gen8_past_ids.contains(species_id);

            // JavaScript: Check if this Pokemon/forme was introduced after requested gen
            let is_future = if let Some(species_gen) = species.gen {
                // Explicit gen field (e.g., Pikachu-World has gen: 8)
                species_gen > gen
            } else if species.num > max_num {
                // JavaScript: Pokemon from future generation (base num check)
                // This catches both base species AND their formes
                true
            } else if let Some(ref forme) = species.forme {
                // JavaScript: gen-specific mods don't include certain formes
                // Forme name patterns indicate which generation they were introduced

                // Paldea formes (Gen 9+): Tauros-Paldea, Wooper-Paldea, etc.
                if forme.contains("Paldea") {
                    gen < 9
                // Gen 8+ formes: Hisui, Galar, Gmax
                } else if forme.contains("Hisui")
                    || forme.contains("Galar")
                    || forme.contains("Galarian")
                    || forme.contains("Gmax")
                {
                    gen < 8
                // Gen 7+ formes: Alola, Totem
                } else if forme.contains("Alola")
                    || forme.contains("Alolan")
                    || forme.contains("Totem")
                {
                    gen < 7
                // Gen 6+ formes: Mega Evolution, Cap Pikachu, Cosplay Pikachu
                } else if forme.contains("Mega")
                    || forme.contains("Primal")
                    || forme == "Original"
                    || forme == "Hoenn"
                    || forme == "Sinnoh"
                    || forme == "Kalos"
                    || forme == "Unova"
                    || forme == "Partner"
                    || forme == "World"
                    || forme == "Rock-Star"
                    || forme == "Belle"
                    || forme == "Pop-Star"
                    || forme == "PhD"
                    || forme == "Libre"
                    || forme == "Cosplay"
                {
                    gen < 6
                // Starter/Let's Go formes (Gen 7+)
                } else if forme == "Starter" {
                    gen < 7
                // Gen 5: Therian formes, Black/White Kyurem, forme Keldeo, etc.
                } else if forme.contains("Therian")
                    || forme.contains("Black") && species.name.contains("Kyurem")
                    || forme.contains("White") && species.name.contains("Kyurem")
                    || forme == "Resolute"
                {
                    gen < 5
                // Gen 4: Rotom formes, Giratina-Origin, Shaymin-Sky, Arceus types
                } else if (forme.contains("Heat") || forme.contains("Wash") || forme.contains("Frost") ||
                           forme.contains("Fan") || forme.contains("Mow")) && species.name.contains("Rotom") ||
                          forme == "Origin" ||
                          forme == "Sky" ||
                          // Arceus plate formes
                          (species.num == 493 && forme != "Normal")
                {
                    gen < 4
                // Gen 3: Deoxys formes, Castform formes
                } else if ((forme == "Attack" || forme == "Defense" || forme == "Speed")
                    && species.name.contains("Deoxys"))
                    || ((forme == "Sunny" || forme == "Rainy" || forme == "Snowy")
                        && species.name.contains("Castform"))
                {
                    gen < 3
                } else {
                    false // Forme exists in all gens or is gen-appropriate
                }
            } else {
                false // Base species within gen range
            };

            if is_future {
                // JavaScript: gen1/pokedex.js doesn't include these entries
                species.tier = Some("Illegal".to_string());
                species.is_nonstandard = Some("Future".to_string());
                continue;
            }

            // JavaScript: Gen-specific mods override tier/isNonstandard for species available in this gen
            // JavaScript uses separate data/mods/gen8/formats-data.ts, data/mods/gen9/formats-data.ts, etc.
            // Each gen mod has different "Past" markers for Pokemon unavailable in that generation.
            //
            // LIMITATION: Our Rust code only has Gen 9 formats-data, not gen-specific formats-data.
            // Gen 9's "Past" markers indicate Pokemon unavailable in Gen 9, which is DIFFERENT from
            // Gen 8's "Past" markers (Pokemon unavailable in Gen 8).
            //
            // WORKAROUND: For Gen 1-8, clear ALL "Past" markers (assume National Dex availability).
            // Only keep "Past" markers for Gen 9 (the generation our data comes from).
            // This isn't 100% accurate for Gen 8 but it's the best we can do without gen-specific data.
            //
            // EXCEPTION: Some Pokemon are marked "Past" because they were never officially released
            // (e.g., Floette-Eternal is "Unobtainable" in gen6 mod). These should NEVER have their
            // "Past" markers cleared.
            let current_gen = 9; // The generation our data files are from

            // Pokemon that should always be marked as unavailable (unreleased event Pokemon, etc.)
            let always_unavailable = matches!(
                species.name.as_str(),
                "Floette-Eternal" // Never officially released (AZ's Floette)
            );

            // Magearna-Original is marked "Unobtainable" in gen7 mod but available in Gen 8+ (Isle of Armor DLC)
            let magearna_original_unavailable = species.name == "Magearna-Original" && gen < 8;

            // LGPE-exclusive Pokemon (Let's Go Pikachu/Eevee)
            // These Pokemon (Meltan #808, Melmetal #809) are marked "LGPE" in gen7 mod
            // but "Past" in gen9 data. They should be excluded from Gen 7 but available in Gen 8+.
            let is_lgpe_exclusive = species.num == 808 || species.num == 809; // Meltan, Melmetal
            let lgpe_unavailable_in_gen7 = is_lgpe_exclusive && gen == 7;

            // Gmax formes (Gigantamax)
            // Gmax formes are marked "Gigantamax" in gen8 formats-data but "Past" in gen9 formats-data
            // They should remain marked as non-standard in ALL gens (including Gen 8)
            // because the JavaScript test excludes Pokemon with any isNonstandard value
            let is_gmax_forme = species
                .forme
                .as_deref()
                .map(|f| f.contains("Gmax"))
                .unwrap_or(false);

            let should_clear_past = if always_unavailable
                || magearna_original_unavailable
                || lgpe_unavailable_in_gen7
                || gen8_unavailable
                || is_gmax_forme
            {
                // Never clear "Past" for:
                // - unreleased Pokemon (Floette-Eternal)
                // - Magearna-Original in Gen < 8
                // - LGPE Pokemon in Gen 7
                // - Gen 8 Dexit Pokemon
                // - Gmax formes (should be excluded in all gens)
                false
            } else if gen == current_gen {
                // Gen 9: Keep "Past" markers (they're correct for this gen)
                false
            } else if let Some(species_gen) = species.gen {
                // Has explicit gen field
                // For now, only clear "Past" if it matches exactly
                // TODO: This causes Gen 8 to have 106 formes instead of 107
                // Need to implement proper gen-specific formats-data loading
                species_gen == gen
            } else {
                // No explicit gen field
                // Gen 1-8: Clear ALL "Past" markers (assume National Dex)
                // Note: Gen 8 uses gen8_past_ids for selective filtering
                true
            };

            // Explicitly mark unavailable Pokemon as Illegal/Unobtainable or Past
            if always_unavailable || magearna_original_unavailable {
                species.tier = Some("Illegal".to_string());
                species.is_nonstandard = Some("Unobtainable".to_string());
            } else if gen8_unavailable {
                species.tier = Some("Illegal".to_string());
                species.is_nonstandard = Some("Past".to_string());
            } else if should_clear_past {
                if species.is_nonstandard.as_deref() == Some("Past") {
                    species.is_nonstandard = None;
                }
                if species.tier.as_deref() == Some("Illegal") {
                    species.tier = None;
                }
            }
            // Note: Gmax formes keep their "Past" isNonstandard marker from Gen 9 data
            // This matches the JavaScript behavior where Gmax formes are excluded in Gen 8
            // because they have isNonstandard: "Gigantamax"
        }

        Ok(dex)
    }
}
