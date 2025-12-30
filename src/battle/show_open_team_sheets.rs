use crate::*;

impl Battle {

    /// Show open team sheets to players
    /// Equivalent to battle.ts showOpenTeamSheets() (battle.ts:3183-3221, 39 lines)
    ///
    // TODO: INCOMPLETE IMPLEMENTATION - Missing Teams.pack() infrastructure
    // Missing from TypeScript version:
    // 1. Teams.pack() serialization function (teams.ts:119-200+)
    //    - Converts PokemonSet[] to compact string format
    //    - Needs Teams struct/module with pack() and packName() methods
    // 2. Once Teams.pack() exists, uncomment the final this.add() calls
    //
    // Architectural difference:
    // - TypeScript Pokemon has `set` field storing original PokemonSet
    // - Rust Pokemon doesn't store original set, so we reconstruct from current fields
    // - This is acceptable since team sheets show current state, not original
    //
    // 	showOpenTeamSheets() {
    // 		if (this.turn !== 0) return;
    // 		for (const side of this.sides) {
    // 			const team = side.pokemon.map(pokemon => {
    // 				const set = pokemon.set;
    // 				const newSet: PokemonSet = {
    // 					name: '',
    // 					species: set.species,
    // 					item: set.item,
    // 					ability: set.ability,
    // 					moves: set.moves,
    // 					nature: '',
    // 					gender: pokemon.gender,
    // 					evs: null!,
    // 					ivs: null!,
    // 					level: set.level,
    // 				};
    // 				if (this.gen === 8) newSet.gigantamax = set.gigantamax;
    // 				if (this.gen === 9) newSet.teraType = set.teraType;
    // 				// Only display Hidden Power type if the Pokemon has Hidden Power
    // 				// This is based on how team sheets were written in past VGC formats
    // 				if (set.moves.some(m => this.dex.moves.get(m).id === 'hiddenpower')) newSet.hpType = set.hpType;
    // 				// This is done so the client doesn't flag Zacian/Zamazenta as illusions
    // 				// when they use their signature move
    // 				if ((toID(set.species) === 'zacian' && toID(set.item) === 'rustedsword') ||
    // 					(toID(set.species) === 'zamazenta' && toID(set.item) === 'rustedshield')) {
    // 					newSet.species = Dex.species.get(set.species + 'crowned').name;
    // 					const crowned: { [k: string]: string } = {
    // 						'Zacian-Crowned': 'behemothblade', 'Zamazenta-Crowned': 'behemothbash',
    // 					};
    // 					const ironHeadIndex = set.moves.map(toID).indexOf('ironhead' as ID);
    // 					if (ironHeadIndex >= 0) {
    // 						newSet.moves[ironHeadIndex] = crowned[newSet.species];
    // 					}
    // 				}
    // 				return newSet;
    // 			});
    //
    // 			this.add('showteam', side.id, Teams.pack(team));
    // 		}
    // 	}
    //
    pub fn show_open_team_sheets(&mut self) {
        // JS: if (this.turn !== 0) return;
        if self.turn != 0 {
            return;
        }

        // JS: for (const side of this.sides) { ... }
        for side_idx in 0..self.sides.len() {
            let (team, side_id) = {
                let side = match self.sides.get(side_idx) {
                    Some(s) => s,
                    None => continue,
                };

                // JS: const team = side.pokemon.map(pokemon => { ... })
                let team: Vec<PokemonSet> = side.pokemon.iter().map(|pokemon| {
                    // Rust doesn't store pokemon.set, so reconstruct from current fields

                    // Get species name (convert ID to string)
                    let species = pokemon.species_id.to_string();

                    // Get item name (convert ID to string)
                    let item = pokemon.item.to_string();

                    // Get ability name (convert ID to string)
                    let ability = pokemon.ability.to_string();

                    // Get moves from move_slots
                    let moves: Vec<String> = pokemon.move_slots.iter()
                        .map(|slot| slot.move_name.clone())
                        .collect();

                    // JS: const newSet: PokemonSet = { ... }
                    let mut new_set = PokemonSet {
                        name: String::new(), // Hidden
                        species: species.clone(),
                        item: item.clone(),
                        ability: ability.clone(),
                        moves: moves.clone(),
                        nature: String::new(), // Hidden
                        gender: pokemon.gender.clone(),
                        evs: crate::dex_data::StatsTable::default(), // Hidden (null! in TypeScript)
                        ivs: crate::dex_data::StatsTable::default(), // Hidden (null! in TypeScript)
                        level: pokemon.level,
                        shiny: false, // Don't reveal shiny status
                        happiness: 255, // Default value
                        pokeball: "pokeball".to_string(),
                        hptype: None, // Will be set below if Hidden Power
                        dynamax_level: 0,
                        gigantamax: false, // Will be set below for Gen 8
                        tera_type: None, // Will be set below for Gen 9
                    };

                    // JS: if (this.gen === 8) newSet.gigantamax = set.gigantamax;
                    if self.gen == 8 {
                        new_set.gigantamax = pokemon.gigantamax;
                    }

                    // JS: if (this.gen === 9) newSet.teraType = set.teraType;
                    if self.gen == 9 {
                        new_set.tera_type = pokemon.tera_type.clone();
                    }

                    // JS: if (set.moves.some(m => this.dex.moves.get(m).id === 'hiddenpower')) newSet.hpType = set.hpType;
                    let has_hidden_power = moves.iter().any(|m| {
                        ID::new(m).as_str() == "hiddenpower"
                    });
                    if has_hidden_power {
                        new_set.hptype = pokemon.hp_type.clone();
                    }

                    // JS: if ((toID(set.species) === 'zacian' && toID(set.item) === 'rustedsword') || ...)
                    let species_id = ID::new(&species);
                    let item_id = ID::new(&item);

                    if (species_id.as_str() == "zacian" && item_id.as_str() == "rustedsword") ||
                       (species_id.as_str() == "zamazenta" && item_id.as_str() == "rustedshield") {

                        // JS: newSet.species = Dex.species.get(set.species + 'crowned').name;
                        let crowned_species = format!("{}-Crowned", species);
                        new_set.species = crowned_species.clone();

                        // JS: const crowned: { [k: string]: string } = { 'Zacian-Crowned': 'behemothblade', ... }
                        let signature_move = if crowned_species == "Zacian-Crowned" {
                            "Behemoth Blade"
                        } else {
                            "Behemoth Bash"
                        };

                        // JS: const ironHeadIndex = set.moves.map(toID).indexOf('ironhead' as ID);
                        if let Some(iron_head_idx) = new_set.moves.iter().position(|m| {
                            ID::new(m).as_str() == "ironhead"
                        }) {
                            // JS: newSet.moves[ironHeadIndex] = crowned[newSet.species];
                            new_set.moves[iron_head_idx] = signature_move.to_string();
                        }
                    }

                    new_set
                }).collect();

                (team, side.id.clone())
            };

            // JS: this.add('showteam', side.id, Teams.pack(team));
            // TODO: Implement Teams.pack() to serialize team to compact string format
            // For now, just indicate teams are revealed without showing actual data
            // TEMPORARY: Show message instead of packed team until Teams.pack() is implemented
            // When Teams.pack() exists, replace this with:
            // let packed_team = Teams::pack(&team);
            // self.add("showteam", &[side_id.to_str().into(), packed_team.into()]);
            let _ = team; // Suppress unused variable warning
            self.add("-message", &[format!("Team sheet revealed for {}", side_id.to_str()).into()]);
        }
    }
}
