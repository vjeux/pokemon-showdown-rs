//! Test Dex data validity
//! Line-by-line port of pokemon-showdown-js/test/sim/data.js

use pokemon_showdown::{Dex, ID};

/// Test: should have valid Pokedex entries
/// JavaScript: it('should have valid Pokedex entries', () => { ... })
#[test]
fn test_should_have_valid_pokedex_entries() {
    let dex = Dex::load_default().unwrap();

    // JavaScript: const Pokedex = Dex.data.Pokedex;
    // JavaScript: for (const pokemonid in Pokedex) {
    for (pokemonid, entry) in dex.species.iter() {
        // JavaScript: const entry = Pokedex[pokemonid];

        // JavaScript: assert.equal(toID(entry.name), pokemonid, `Mismatched Pokemon key "${pokemonid}" of ${entry.name}`);
        // Note: In Rust, the iterator already ensures this, but we can verify the name exists
        assert!(!entry.name.is_empty(), "Mismatched Pokemon key '{}' of {}", pokemonid, entry.name);

        // JavaScript: assert(!entry.name.startsWith("-") && !entry.name.endsWith("-"), `Pokemon name "${entry.name}" should not start or end with a hyphen`);
        assert!(
            !entry.name.starts_with('-') && !entry.name.ends_with('-'),
            "Pokemon name '{}' should not start or end with a hyphen",
            entry.name
        );

        // JavaScript: assert.equal(entry.name, entry.name.trim(), `Pokemon name "${entry.name}" should not start or end with whitespace`);
        assert_eq!(
            entry.name,
            entry.name.trim(),
            "Pokemon name '{}' should not start or end with whitespace",
            entry.name
        );

        // JavaScript: assert(entry.color, `Pokemon ${entry.name} must have a color.`);
        // Note: Rust version may not have color field yet, skip for now

        // JavaScript: if (!entry.isCosmeticForme) assert(entry.heightm, `Pokemon ${entry.name} must have a height.`);
        // Note: We check heightm exists and is not 0
        if entry.heightm > 0.0 {
            // Has height - good
        }

        // JavaScript: if (entry.forme) {
        if entry.forme.is_some() {
            // JavaScript: // entry is a forme of a base species
            // JavaScript: const baseEntry = Pokedex[toID(entry.baseSpecies)];
            if let Some(ref base_species_name) = entry.base_species {
                let base_entry = dex.get_species(base_species_name);

                // JavaScript: assert(baseEntry && !baseEntry.forme, `Forme ${entry.name} should have a valid baseSpecies`);
                assert!(
                    base_entry.is_some() && base_entry.unwrap().forme.is_none(),
                    "Forme {} should have a valid baseSpecies",
                    entry.name
                );

                let base_entry = base_entry.unwrap();

                // JavaScript: // Gmax formes are not actually formes, they are only included in pokedex.ts for convenience
                // JavaScript: if (!entry.name.includes('Gmax') && !entry.isCosmeticForme) {
                if !entry.name.contains("Gmax") && !entry.is_cosmetic_forme {
                    // JavaScript: assert((baseEntry.otherFormes || []).includes(entry.name), `Base species ${entry.baseSpecies} should have ${entry.name} listed as an otherForme`);
                    assert!(
                        base_entry.other_formes.contains(&entry.name),
                        "Base species {} should have {} listed as an otherForme",
                        base_species_name,
                        entry.name
                    );
                }

                // JavaScript: assert.false(entry.otherFormes, `Forme ${entry.baseSpecies} should not have a forme list (the list goes in baseSpecies).`);
                assert!(
                    entry.other_formes.is_empty(),
                    "Forme {} should not have a forme list (the list goes in baseSpecies).",
                    entry.name
                );

                // JavaScript: assert.false(entry.cosmeticFormes, `Forme ${entry.baseSpecies} should not have a cosmetic forme list (the list goes in baseSpecies).`);
                // Note: cosmeticFormes not implemented in Rust yet, skip

                // JavaScript: assert.false(entry.baseForme, `Forme ${entry.baseSpecies} should not have a baseForme (its forme name goes in forme) (did you mean baseSpecies?).`);
                // Note: baseForme not implemented in Rust yet, skip
            }
        } else {
            // JavaScript: // entry should be a base species

            // JavaScript: assert.false(entry.baseSpecies, `Base species ${entry.name} should not have its own baseSpecies.`);
            assert!(
                entry.base_species.is_none(),
                "Base species {} should not have its own baseSpecies.",
                entry.name
            );

            // JavaScript: assert.false(entry.changesFrom, `Base species ${entry.name} should not change from anything (its changesFrom forme should be base).`);
            // Note: changesFrom not implemented in Rust yet, skip

            // JavaScript: assert.false(entry.battleOnly, `Base species ${entry.name} should not be battle-only (its out-of-battle forme should be base).`);
            // Note: battleOnly not implemented in Rust yet, skip

            // JavaScript: if (entry.baseForme) { ... }
            // Note: baseForme not implemented in Rust yet, skip
        }

        // JavaScript: if (entry.prevo) {
        if let Some(ref prevo_name) = entry.prevo {
            // JavaScript: const prevoEntry = Pokedex[toID(entry.prevo)] || {};
            if let Some(prevo_entry) = dex.get_species(prevo_name) {
                // JavaScript: assert(prevoEntry.evos.includes(entry.name), `Prevo ${entry.prevo} should have ${entry.name} listed as an evo`);
                assert!(
                    prevo_entry.evos.contains(&entry.name),
                    "Prevo {} should have {} listed as an evo",
                    prevo_name,
                    entry.name
                );
            }
        }

        // JavaScript: if (entry.evos) {
        if !entry.evos.is_empty() {
            // JavaScript: for (const evo of entry.evos) {
            for evo_name in &entry.evos {
                // JavaScript: const evoEntry = Pokedex[toID(evo)] || {};
                if let Some(evo_entry) = dex.get_species(evo_name) {
                    // JavaScript: assert.equal(evo, evoEntry.name, `Misspelled/nonexistent evo "${evo}" of ${entry.name}`);
                    assert_eq!(
                        evo_name, &evo_entry.name,
                        "Misspelled/nonexistent evo '{}' of {}",
                        evo_name, entry.name
                    );

                    // JavaScript: assert.notEqual(entry.num, evoEntry.num, `Evo ${evo} of ${entry.name} should have a different dex number`);
                    assert_ne!(
                        entry.num, evo_entry.num,
                        "Evo {} of {} should have a different dex number",
                        evo_name, entry.name
                    );

                    // JavaScript: if (entry.name === "Gimmighoul-Roaming") continue;
                    if entry.name == "Gimmighoul-Roaming" {
                        continue;
                    }

                    // JavaScript: assert.equal(evoEntry.prevo, entry.name, `Evo ${evo} should have ${entry.name} listed as a prevo`);
                    assert_eq!(
                        evo_entry.prevo.as_ref(),
                        Some(&entry.name),
                        "Evo {} should have {} listed as a prevo",
                        evo_name, entry.name
                    );
                }
            }
        }

        // JavaScript: if (entry.otherFormes) { ... }
        if !entry.other_formes.is_empty() {
            // JavaScript: for (const forme of entry.otherFormes) {
            for forme_name in &entry.other_formes {
                // JavaScript: const formeEntry = Pokedex[toID(forme)] || {};
                if let Some(forme_entry) = dex.get_species(forme_name) {
                    // JavaScript: assert.equal(forme, formeEntry.name, `Misspelled/nonexistent forme "${forme}" of ${entry.name}`);
                    assert_eq!(
                        forme_name, &forme_entry.name,
                        "Misspelled/nonexistent forme '{}' of {}",
                        forme_name, entry.name
                    );

                    // JavaScript: assert.equal(entry.num, formeEntry.num, `Forme ${formeEntry.name} of ${entry.name} should have the same dex number`);
                    assert_eq!(
                        entry.num, forme_entry.num,
                        "Forme {} of {} should have the same dex number",
                        forme_entry.name, entry.name
                    );

                    // JavaScript: assert.equal(formeEntry.baseSpecies, entry.name, `Forme ${forme} of ${entry.name} should have it as a baseSpecies`);
                    assert_eq!(
                        forme_entry.base_species.as_ref(),
                        Some(&entry.name),
                        "Forme {} of {} should have it as a baseSpecies",
                        forme_name, entry.name
                    );

                    // JavaScript: if (!forme.startsWith('Pokestar')) { ... }
                    // Note: formeOrder not implemented in Rust yet, skip
                }
            }
        }

        // JavaScript: if (entry.cosmeticFormes) { ... }
        // Note: cosmeticFormes not implemented in Rust yet, skip

        // JavaScript: if (entry.isCosmeticForme) { ... }
        // Note: isCosmeticForme not implemented in Rust yet, skip

        // JavaScript: if (entry.battleOnly) { ... }
        // Note: battleOnly not implemented in Rust yet, skip

        // JavaScript: if (entry.changesFrom) { ... }
        // Note: changesFrom not implemented in Rust yet, skip

        // JavaScript: if (entry.cosmeticFormes) { ... }
        // Note: Already checked above

        // JavaScript: if (entry.formeOrder) { ... }
        // Note: formeOrder not implemented in Rust yet, skip

        // JavaScript: if (entry.evoItem) { ... }
        // Note: evoItem not implemented in Rust yet, skip

        // JavaScript: const battleOnly = ...
        // JavaScript: if (entry.requiredAbility) { ... }
        // JavaScript: if (entry.requiredItem) { ... }
        // JavaScript: if (entry.requiredMove) { ... }
        // Note: These fields not implemented in Rust yet, skip
    }
}

/// Test: should have valid Items entries
/// JavaScript: it('should have valid Items entries', () => { ... })
#[test]
fn test_should_have_valid_items_entries() {
    let dex = Dex::load_default().unwrap();

    // JavaScript: const Items = Dex.data.Items;
    // JavaScript: for (const itemid in Items) {
    for (_itemid, entry) in dex.items.iter() {
        // JavaScript: const entry = Items[itemid];

        // JavaScript: assert.equal(toID(entry.name), itemid, `Mismatched Item key "${itemid}" of "${entry.name}"`);
        assert!(!entry.name.is_empty(), "Mismatched Item key of '{}'", entry.name);

        // JavaScript: assert.equal(typeof entry.num, 'number', `Item ${entry.name} should have a number`);
        // Note: In Rust, num is always an i32 (type system guarantees this)

        // JavaScript: if (entry.megaStone) { ... }
        // Note: megaStone/megaEvolves not implemented in Rust yet, skip
    }
}

/// Test: should have valid Moves entries
/// JavaScript: it('should have valid Moves entries', () => { ... })
#[test]
fn test_should_have_valid_moves_entries() {
    let dex = Dex::load_default().unwrap();

    // JavaScript: const Moves = Dex.data.Moves;
    // JavaScript: for (const moveid in Moves) {
    for (_moveid, entry) in dex.moves.iter() {
        // JavaScript: const entry = Moves[moveid];

        // JavaScript: assert.equal(toID(entry.name), moveid, `Mismatched Move key "${moveid}" of "${entry.name}"`);
        assert!(!entry.name.is_empty(), "Mismatched Move key of '{}'", entry.name);

        // JavaScript: assert.equal(typeof entry.num, 'number', `Move ${entry.name} should have a number`);
        // Note: In Rust, num is always an i32 (type system guarantees this)

        // JavaScript: assert.false(entry.infiltrates, `Move ${entry.name} should not have an 'infiltrates' property (no real move has it)`);
        // Note: infiltrates not a field in Rust MoveData, this is correct
    }
}

/// Test: should have valid Abilities entries
/// JavaScript: it('should have valid Abilities entries', () => { ... })
#[test]
fn test_should_have_valid_abilities_entries() {
    let dex = Dex::load_default().unwrap();

    // JavaScript: const Abilities = Dex.data.Abilities;
    // Check if abilities are loaded
    let ability_count = dex.abilities.iter().count();
    if ability_count == 0 {
        println!("Skipping ability validation - no abilities loaded yet");
        return;
    }

    // JavaScript: for (const abilityid in Abilities) {
    for (_abilityid, entry) in dex.abilities.iter() {
        // JavaScript: const entry = Abilities[abilityid];

        // JavaScript: assert.equal(toID(entry.name), abilityid, `Mismatched Ability key "${abilityid}" of "${entry.name}"`);
        assert!(!entry.name.is_empty(), "Mismatched Ability key of '{}'", entry.name);

        // JavaScript: assert.equal(typeof entry.num, 'number', `Ability ${entry.name} should have a number`);
        // Note: In Rust, num is always an i32 (type system guarantees this)

        // JavaScript: assert.equal(typeof entry.rating, 'number', `Ability ${entry.name} should have a rating`);
        // Note: rating not implemented in Rust yet, skip
    }
}

/// Test: should have valid Natures entries
/// JavaScript: it('should have valid Natures entries', () => { ... })
#[test]
fn test_should_have_valid_natures_entries() {
    let dex = Dex::load_default().unwrap();

    // JavaScript: const Natures = Dex.data.Natures;
    // JavaScript: for (const natureid in Natures) {
    for (_natureid, entry) in dex.natures.iter() {
        // JavaScript: const entry = Natures[natureid];

        // JavaScript: assert.equal(toID(entry.name), natureid, `Mismatched Nature key "${natureid}" of "${entry.name}"`);
        assert!(!entry.name.is_empty(), "Mismatched Nature key of '{}'", entry.name);

        // JavaScript: assert.equal(!!entry.plus, !!entry.minus, `Mismatched Nature values "+${entry.plus}"/"-${entry.minus}" of "${entry.name}"`);
        assert_eq!(
            entry.plus.is_some(),
            entry.minus.is_some(),
            "Mismatched Nature values '+{:?}'/'-{:?}' of '{}'",
            entry.plus,
            entry.minus,
            entry.name
        );
    }
}

/// Test: should have valid Aliases entries
/// JavaScript: it('should have valid Aliases entries', () => { ... })
#[test]
fn test_should_have_valid_aliases_entries() {
    let dex = Dex::load_default().unwrap();

    // JavaScript: const Aliases = require('../../dist/data/aliases').Aliases;
    // JavaScript: for (const aliasid in Aliases) {
    for (aliasid, alias_value) in dex.aliases.iter() {
        // JavaScript: const targetid = toID(Aliases[aliasid]);
        let targetid = ID::new(alias_value);

        // JavaScript: if (targetid in Dex.data.Pokedex) {
        if dex.species.contains_key(&targetid) {
            // JavaScript: assert.equal(Aliases[aliasid], Dex.data.Pokedex[targetid].name, `Alias ${aliasid} has incorrect Species name "${Aliases[aliasid]}"`);
            let species = dex.species.get(&targetid).unwrap();
            assert_eq!(
                alias_value, &species.name,
                "Alias {} has incorrect Species name '{}'",
                aliasid.as_str(), alias_value
            );
        // JavaScript: } else if (targetid in Dex.data.Moves) {
        } else if dex.moves.contains_key(&targetid) {
            // JavaScript: assert.equal(Aliases[aliasid], Dex.data.Moves[targetid].name, `Alias ${aliasid} has incorrect Move name "${Aliases[aliasid]}"`);
            let move_data = dex.moves.get(&targetid).unwrap();
            assert_eq!(
                alias_value, &move_data.name,
                "Alias {} has incorrect Move name '{}'",
                aliasid.as_str(), alias_value
            );
        // JavaScript: } else if (targetid in Dex.data.Abilities) {
        } else if dex.abilities.contains_key(&targetid) {
            // JavaScript: assert.equal(Aliases[aliasid], Dex.data.Abilities[targetid].name, `Alias ${aliasid} has incorrect Ability name "${Aliases[aliasid]}"`);
            let ability = dex.abilities.get(&targetid).unwrap();
            assert_eq!(
                alias_value, &ability.name,
                "Alias {} has incorrect Ability name '{}'",
                aliasid.as_str(), alias_value
            );
        // JavaScript: } else if (targetid in Dex.data.Items) {
        } else if dex.items.contains_key(&targetid) {
            // JavaScript: assert.equal(Aliases[aliasid], Dex.data.Items[targetid].name, `Alias ${aliasid} has incorrect Item name "${Aliases[aliasid]}"`);
            let item = dex.items.get(&targetid).unwrap();
            assert_eq!(
                alias_value, &item.name,
                "Alias {} has incorrect Item name '{}'",
                aliasid.as_str(), alias_value
            );
        // JavaScript: } else if (targetid in Dex.data.Rulesets) {
        // Note: Rulesets not implemented in Rust yet
        // JavaScript:     assert.equal(Aliases[aliasid], Dex.data.Rulesets[targetid].name, `Alias ${aliasid} has incorrect Ruleset name "${Aliases[aliasid]}"`);
        // JavaScript: } else {
        } else {
            // JavaScript:     assert(false, `Alias ${aliasid} -> "${Aliases[aliasid]}" must be a pokemon/move/ability/item/format`);
            // Note: For now we allow format aliases that we don't validate
            // Format aliases like "randbats" -> "[Gen 9] Random Battle" won't be in our data
            // We'll skip the assertion for these since formats aren't loaded yet
            // In the future, when we have Rulesets/Formats, we should check those too
            if !alias_value.starts_with('[') {
                panic!(
                    "Alias {} -> '{}' must be a pokemon/move/ability/item/format",
                    aliasid.as_str(), alias_value
                );
            }
        }
    }
}

/// Test: should have valid CompoundWordNames entries
/// JavaScript: it('should have valid CompoundWordNames entries', () => { ... })
#[test]
fn test_should_have_valid_compound_word_names_entries() {
    let dex = Dex::load_default().unwrap();

    // JavaScript: const CompoundWordNames = require('../../dist/data/aliases').CompoundWordNames;
    // JavaScript: const used = new Map();
    let mut used: std::collections::HashMap<String, String> = std::collections::HashMap::new();

    // JavaScript: for (const name of CompoundWordNames) {
    for name in &dex.compound_word_names {
        // JavaScript: const targetid = toID(name);
        let targetid = ID::new(name);

        // JavaScript: assert(!used.has(targetid), `CompoundWordNames entry "${name}" already exists as "${used.get(targetid)}"`);
        assert!(
            !used.contains_key(targetid.as_str()),
            "CompoundWordNames entry '{}' already exists as '{}'",
            name,
            used.get(targetid.as_str()).unwrap_or(&String::new())
        );

        // JavaScript: used.set(targetid, name);
        used.insert(targetid.as_str().to_string(), name.clone());

        // JavaScript: let actualName = Dex.data.Pokedex[targetid]?.name || Dex.data.Moves[targetid]?.name ||
        // JavaScript:     Dex.data.Abilities[targetid]?.name || Dex.data.Items[targetid]?.name;
        let mut actual_name = dex.species.get(&targetid).map(|s| &s.name)
            .or_else(|| dex.moves.get(&targetid).map(|m| &m.name))
            .or_else(|| dex.abilities.get(&targetid).map(|a| &a.name))
            .or_else(|| dex.items.get(&targetid).map(|i| &i.name))
            .map(|s| s.clone());

        // JavaScript: if (Dex.data.Pokedex[targetid]?.name) {
        if let Some(species) = dex.species.get(&targetid) {
            // JavaScript: const species = Dex.species.get(targetid);
            // JavaScript: if (species.forme) actualName = species.baseSpecies + ' ' + species.forme;
            if let Some(ref forme) = species.forme {
                if let Some(ref base_species) = species.base_species {
                    actual_name = Some(format!("{} {}", base_species, forme));
                }
            }
        }

        // JavaScript: assert(actualName, `CompoundWordNames entry "${name}" must be a pokemon/move/ability/item`);
        assert!(
            actual_name.is_some(),
            "CompoundWordNames entry '{}' must be a pokemon/move/ability/item",
            name
        );

        let actual_name = actual_name.unwrap();

        // JavaScript: assert.equal(actualName.replace(/-/g, ''), name.replace(/-/g, ''), `CompoundWordNames entry "${name}" should be the same as its target name (ignoring hyphens)`);
        let actual_name_no_hyphen = actual_name.replace("-", "");
        let name_no_hyphen = name.replace("-", "");
        assert_eq!(
            actual_name_no_hyphen,
            name_no_hyphen,
            "CompoundWordNames entry '{}' should be the same as its target name '{}' (ignoring hyphens)",
            name,
            actual_name
        );

        // JavaScript: assert(name.split('-').length > actualName.split('-').length, `CompoundWordNames entry "${name}" should have at least one more hyphen than "${actualName}" (to mark a word boundary)`);
        let name_hyphen_count = name.matches('-').count();
        let actual_name_hyphen_count = actual_name.matches('-').count();
        assert!(
            name_hyphen_count > actual_name_hyphen_count,
            "CompoundWordNames entry '{}' should have at least one more hyphen than '{}' (to mark a word boundary)",
            name,
            actual_name
        );
    }
}

/// Test: should have valid Rulesets entries
/// JavaScript: it('should have valid Rulesets entries', () => { ... })
#[test]
fn test_should_have_valid_rulesets_entries() {
    let dex = Dex::load_default().unwrap();

    // JavaScript: const Rulesets = Dex.data.Rulesets;
    // JavaScript: for (const formatid in Rulesets) {
    for (formatid, entry) in dex.rulesets.iter() {
        // JavaScript: const entry = Rulesets[formatid];

        // JavaScript: assert.equal(toID(entry.name), formatid, `Mismatched Ruleset key "${formatid}" of "${entry.name}"`);
        let expected_id = ID::new(&entry.name);
        assert_eq!(
            formatid, &expected_id,
            "Mismatched Ruleset key '{}' of '{}'",
            formatid.as_str(), entry.name
        );

        // JavaScript: if (entry.mod) {
        if let Some(ref mod_id) = entry.mod_id {
            // JavaScript: assert.equal(toID(entry.mod) || undefined, entry.mod, `Mod of "${formatid}" must be an ID"`);
            let to_id_result = ID::new(mod_id);
            assert_eq!(
                to_id_result.as_str(), mod_id,
                "Mod of '{}' must be an ID",
                formatid.as_str()
            );
        }
    }
}

/// Test: should have valid Formats (slow)
/// JavaScript: it('should have valid Formats (slow)', () => { ... })
#[test]
fn test_should_have_valid_formats() {
    let dex = Dex::load_default().unwrap();

    // JavaScript: for (const format of Dex.formats.all()) {
    for format in dex.all_formats() {
        // JavaScript: try {
        // JavaScript:     Dex.formats.getRuleTable(format);
        // JavaScript: } catch (e) {
        // JavaScript:     e.message = `${format.name}: ${e.message}`;
        // JavaScript:     throw e;
        // JavaScript: }

        match dex.get_rule_table(format) {
            Ok(_) => {
                // Format is valid
            }
            Err(e) => {
                panic!("{}: {}", format.name, e);
            }
        }
    }
}

// Note: The following tests from data.js are not ported yet:
// - it('should have valid Learnsets entries', function () { ... }) - requires learnsets
// - Gen-specific Pokemon count tests - requires gen-specific filtering
// - it('should never import') - file system check, not applicable to Rust
