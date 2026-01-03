// NOTE: This method is NOT in JavaScript - Rust-specific implementation

use crate::*;

impl Battle {

    /// Start the first turn (after team preview)
    /// Equivalent to TypeScript runAction() case 'start' (battle.ts:2634-2701)
    /// Note: In TS this is part of runAction switch statement, extracted to separate method in Rust
    pub fn start_battle(&mut self) {
        // JS: for (const side of this.sides) {
        // JS:     if (side.pokemonLeft) side.pokemonLeft = side.pokemon.length;
        // JS:     this.add('teamsize', side.id, side.pokemon.length);
        // JS: }
        for side_idx in 0..self.sides.len() {
            let pokemon_len = self.sides[side_idx].pokemon.len();
            if self.sides[side_idx].pokemon_left > 0 {
                self.sides[side_idx].pokemon_left = pokemon_len;
            }
            let side_id = self.sides[side_idx].id_str().to_string();
            self.add("teamsize", &[Arg::Str(&side_id), Arg::String(pokemon_len.to_string())]);
        }

        // JS: this.add('start');
        self.add("start", &[]);

        // JS: // Change Zacian/Zamazenta into their Crowned formes
        // JS: for (const pokemon of this.getAllPokemon()) { ... }
        // Collect all pokemon positions first to avoid borrow issues
        let all_pokemon_positions: Vec<(usize, usize)> = self
            .sides
            .iter()
            .enumerate()
            .flat_map(|(side_idx, side)| {
                (0..side.pokemon.len()).map(move |poke_idx| (side_idx, poke_idx))
            })
            .collect();

        for (side_idx, poke_idx) in all_pokemon_positions.clone() {
            // JS: let rawSpecies: Species | null = null;
            // JS: if (pokemon.species.id === 'zacian' && pokemon.item === 'rustedsword') {
            // JS:     rawSpecies = this.dex.species.get('Zacian-Crowned');
            // JS: } else if (pokemon.species.id === 'zamazenta' && pokemon.item === 'rustedshield') {
            // JS:     rawSpecies = this.dex.species.get('Zamazenta-Crowned');
            // JS: }
            let (species_id, item) = {
                let pokemon = &self.sides[side_idx].pokemon[poke_idx];
                (pokemon.species_id.clone(), pokemon.item.clone())
            };

            let new_species = if species_id.as_str() == "zacian" && item.as_str() == "rustedsword" {
                Some(ID::new("zaciancrowned"))
            } else if species_id.as_str() == "zamazenta" && item.as_str() == "rustedshield" {
                Some(ID::new("zamazentacrowned"))
            } else {
                None
            };

            // JS: if (!rawSpecies) continue;
            if new_species.is_none() {
                continue;
            }
            let new_species = new_species.unwrap();

            // JS: const species = pokemon.setSpecies(rawSpecies);
            // JS: if (!species) continue;
            let success = {
                let pokemon = &mut self.sides[side_idx].pokemon[poke_idx];
                unsafe {
                    let p = pokemon as *mut Pokemon;
                    let b = self as *mut Battle;
                    (*p).set_species(&mut *b, &new_species, None, false)
                }
            };

            if !success {
                continue;
            }

            // JS: pokemon.baseSpecies = rawSpecies;
            self.sides[side_idx].pokemon[poke_idx].base_species = new_species.clone();

            // JS: pokemon.details = pokemon.getUpdatedDetails();
            let details = self.sides[side_idx].pokemon[poke_idx].get_updated_details();
            self.sides[side_idx].pokemon[poke_idx].details = details;

            // JS: pokemon.setAbility(species.abilities['0'], null, null, true);
            // TODO: Implement setAbility()

            // JS: pokemon.baseAbility = pokemon.ability;
            let ability = self.sides[side_idx].pokemon[poke_idx].ability.clone();
            self.sides[side_idx].pokemon[poke_idx].base_ability = ability;

            // JS: const behemothMove: { [k: string]: string } = {
            // JS:     'Zacian-Crowned': 'behemothblade', 'Zamazenta-Crowned': 'behemothbash',
            // JS: };
            // JS: const ironHeadIndex = pokemon.baseMoves.indexOf('ironhead');
            // JS: if (ironHeadIndex >= 0) { ... replace with behemoth move ... }
            // TODO: Implement Iron Head -> Behemoth move replacement
        }

        // JS: this.format.onBattleStart?.call(this);
        // JS: for (const rule of this.ruleTable.keys()) {
        // JS:     if ('+*-!'.includes(rule.charAt(0))) continue;
        // JS:     const subFormat = this.dex.formats.get(rule);
        // JS:     subFormat.onBattleStart?.call(this);
        // JS: }
        // TODO: Format callbacks - emit events for now
        self.run_event("FormatBattleStart", None, None, None, None);

        if let Some(ref rule_table) = self.rule_table.clone() {
            let rule_keys: Vec<String> = rule_table.keys().cloned().collect();
            for rule in rule_keys {
                if let Some(first_char) = rule.chars().next() {
                    if first_char == '+' || first_char == '*' || first_char == '-' || first_char == '!' {
                        continue;
                    }
                }
                self.run_event(&format!("RuleBattleStart:{}", rule), None, None, None, None);
            }
        }

        // JS: for (const side of this.sides) {
        // JS:     for (let i = 0; i < side.active.length; i++) {
        // JS:         if (!side.pokemonLeft) {
        // JS:             // forfeited before starting
        // JS:             side.active[i] = side.pokemon[i];
        // JS:             side.active[i].fainted = true;
        // JS:             side.active[i].hp = 0;
        // JS:         } else {
        // JS:             this.actions.switchIn(side.pokemon[i], i);
        // JS:         }
        // JS:     }
        // JS: }
        for side_idx in 0..self.sides.len() {
            let active_len = self.sides[side_idx].active.len();
            let pokemon_left = self.sides[side_idx].pokemon_left;

            for slot in 0..active_len {
                if pokemon_left == 0 {
                    // Forfeited before starting
                    self.sides[side_idx].active[slot] = Some(slot);
                    self.sides[side_idx].pokemon[slot].fainted = true;
                    self.sides[side_idx].pokemon[slot].hp = 0;
                } else {
                    // JS: this.actions.switchIn(side.pokemon[i], i);
                    crate::battle_actions::switch_in(self, side_idx, slot, slot, None, false);
                }
            }
        }

        // JS: for (const pokemon of this.getAllPokemon()) {
        // JS:     this.singleEvent('Start', this.dex.conditions.getByID(pokemon.species.id), pokemon.speciesState, pokemon);
        // JS: }
        for (side_idx, poke_idx) in all_pokemon_positions {
            let species_id = self.sides[side_idx].pokemon[poke_idx].species_id.clone();
            self.single_event("Start", &species_id, Some((side_idx, poke_idx)), None, None);
        }

        // JS: this.midTurn = true;
        self.mid_turn = true;
    }
}
