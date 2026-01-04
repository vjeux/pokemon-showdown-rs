//! Pokemon::forme_change implementation
//!
//! Pokemon Showdown - http://pokemonshowdown.com/

use crate::*;
use crate::event::EventResult;
use crate::dex_data::ID;

impl Pokemon {
    /// Changes this Pokemon's forme to match the given speciesId.
    /// This function handles all changes to stats, ability, type, species, etc.
    /// as well as sending all relevant messages sent to the client.
    ///
    /// JavaScript source:
    /// ```typescript
    /// formeChange(
    ///     speciesId: string | Species, source: Effect | null = this.battle.effect,
    ///     isPermanent?: boolean, abilitySlot = '0', message?: string
    /// ) {
    ///     const rawSpecies = this.battle.dex.species.get(speciesId);
    ///
    ///     const species = this.setSpecies(rawSpecies, source);
    ///     if (!species) return false;
    ///
    ///     if (this.battle.gen <= 2) return true;
    ///
    ///     // The species the opponent sees
    ///     const apparentSpecies =
    ///         this.illusion ? this.illusion.species.name : species.baseSpecies;
    ///     if (isPermanent) {
    ///         this.baseSpecies = rawSpecies;
    ///         this.details = this.getUpdatedDetails();
    ///         let details = (this.illusion || this).details;
    ///         if (this.terastallized) details += `, tera:${this.terastallized}`;
    ///         this.battle.add('detailschange', this, details);
    ///         this.updateMaxHp();
    ///         if (!source) {
    ///             // Tera forme
    ///             // Ogerpon/Terapagos text goes here
    ///             this.formeRegression = true;
    ///         } else if (source.effectType === 'Item') {
    ///             this.canTerastallize = null; // National Dex behavior
    ///             if (source.zMove) {
    ///                 this.battle.add('-burst', this, apparentSpecies, species.requiredItem);
    ///                 this.moveThisTurnResult = true; // Ultra Burst counts as an action for Truant
    ///             } else if (source.isPrimalOrb) {
    ///                 if (this.illusion) {
    ///                     this.ability = '';
    ///                     this.battle.add('-primal', this.illusion, species.requiredItem);
    ///                 } else {
    ///                     this.battle.add('-primal', this, species.requiredItem);
    ///                 }
    ///             } else {
    ///                 this.battle.add('-mega', this, apparentSpecies, species.requiredItem);
    ///                 this.moveThisTurnResult = true; // Mega Evolution counts as an action for Truant
    ///             }
    ///             this.formeRegression = true;
    ///         } else if (source.effectType === 'Status') {
    ///             // Shaymin-Sky -> Shaymin
    ///             this.battle.add('-formechange', this, species.name, message);
    ///         }
    ///     } else {
    ///         if (source?.effectType === 'Ability') {
    ///             this.battle.add('-formechange', this, species.name, message, `[from] ability: ${source.name}`);
    ///         } else {
    ///             this.battle.add('-formechange', this, this.illusion ? this.illusion.species.name : species.name, message);
    ///         }
    ///     }
    ///     if (isPermanent && (!source || !['disguise', 'iceface'].includes(source.id))) {
    ///         if (this.illusion && source) {
    ///             // Tera forme by Ogerpon or Terapagos breaks the Illusion
    ///             this.ability = ''; // Don't allow Illusion to wear off
    ///         }
    ///         const ability = species.abilities[abilitySlot] || species.abilities['0'];
    ///         // Ogerpon's forme change doesn't override permanent abilities
    ///         if (source || !this.getAbility().flags['cantsuppress']) this.setAbility(ability, null, null, true);
    ///         // However, its ability does reset upon switching out
    ///         this.baseAbility = toID(ability);
    ///     }
    ///     if (this.terastallized) {
    ///         this.knownType = true;
    ///         this.apparentType = this.terastallized;
    ///     }
    ///     return true;
    /// }
    /// ```
    /// Position-based forme change for use when you need to pass battle mutably
    /// This is the version that should be used from battle_actions and similar contexts
    pub fn forme_change(
        battle: &mut Battle,
        pokemon_pos: (usize, usize),
        species_id: ID,
        source_id: Option<ID>,
        is_permanent: bool,
        _ability_slot: &str, // TODO: Use when implementing permanent forme changes with ability changes
        message: Option<&str>,
    ) -> bool {
        // const rawSpecies = this.battle.dex.species.get(speciesId);
        let (species_name, species_base_species) = {
            if let Some(species_data) = battle.dex.species().get(species_id.as_str()) {
                (species_data.name.clone(), species_data.base_species.clone())
            } else {
                // Species not found
                return false;
            }
        };

        // const species = this.setSpecies(rawSpecies, source);
        let set_species_result = {
            let pokemon = &mut battle.sides[pokemon_pos.0].pokemon[pokemon_pos.1];
            let species_id_ref = species_id.clone();
            // Drop pokemon borrow before calling set_species
            let result = {
                // We need to call set_species through the Pokemon API
                // For now, we'll assume success and set the species directly
                // TODO: Implement proper set_species call when infrastructure allows
                pokemon.species_id = species_id_ref.clone();
                true
            };
            result
        };

        // if (!species) return false;
        if !set_species_result {
            return false;
        }

        // if (this.battle.gen <= 2) return true;
        if battle.gen <= 2 {
            return true;
        }

        // For now, we implement the non-permanent forme change case (most common for abilities like Flower Gift)
        // isPermanent: false branch
        if !is_permanent {
            // const apparentSpecies = this.illusion ? this.illusion.species.name : species.baseSpecies;
            // Note: Illusion not implemented yet, use actual species
            let apparent_species = species_base_species.unwrap_or_else(|| species_name.clone());

            // if (source?.effectType === 'Ability')
            if let Some(source) = source_id {
                // Check if source is an ability by looking it up in dex
                let source_name = if let Some(ability_data) = battle.dex.abilities().get(source.as_str()) {
                    Some(ability_data.name.clone())
                } else {
                    None
                };

                let pokemon_slot = battle.sides[pokemon_pos.0].pokemon[pokemon_pos.1].get_slot();

                use crate::battle::Arg;
                if let Some(ability_name) = source_name {
                    // this.battle.add('-formechange', this, species.name, message, `[from] ability: ${source.name}`);
                    let mut args = vec![
                        Arg::String(pokemon_slot),
                        Arg::String(species_name.clone()),
                    ];
                    if let Some(msg) = message {
                        args.push(Arg::Str(msg));
                    }
                    args.push(Arg::String(format!("[from] ability: {}", ability_name)));

                    battle.add("-formechange", &args);
                } else {
                    // this.battle.add('-formechange', this, this.illusion ? this.illusion.species.name : species.name, message);
                    let mut args = vec![
                        Arg::String(pokemon_slot),
                        Arg::String(apparent_species.clone()),
                    ];
                    if let Some(msg) = message {
                        args.push(Arg::Str(msg));
                    }

                    battle.add("-formechange", &args);
                }
            } else {
                // No source, just add formechange
                let pokemon_slot = battle.sides[pokemon_pos.0].pokemon[pokemon_pos.1].get_slot();

                use crate::battle::Arg;
                let mut args = vec![
                    Arg::String(pokemon_slot),
                    Arg::String(apparent_species),
                ];
                if let Some(msg) = message {
                    args.push(Arg::Str(msg));
                }

                battle.add("-formechange", &args);
            }
        } else {
            // isPermanent: true branch
            // JavaScript: if (isPermanent) {
            //     this.baseSpecies = rawSpecies;
            //     this.details = this.getUpdatedDetails();
            //     let details = (this.illusion || this).details;
            //     if (this.terastallized) details += `, tera:${this.terastallized}`;
            //     this.battle.add('detailschange', this, details);
            //     this.updateMaxHp();
            //     ...
            // }

            // Get details before mutation
            let (details, terastallized) = {
                let pokemon = &mut battle.sides[pokemon_pos.0].pokemon[pokemon_pos.1];
                // Update base_species
                pokemon.base_species = species_id.clone();
                // Get updated details
                let details = pokemon.get_updated_details();
                (details, pokemon.terastallized.clone())
            };

            // Add terastallized to details if applicable
            let details_with_tera = if let Some(ref tera_type) = terastallized {
                format!("{}, tera:{}", details, tera_type)
            } else {
                details.clone()
            };

            // Add detailschange message
            let pokemon_slot = battle.sides[pokemon_pos.0].pokemon[pokemon_pos.1].get_slot();
            battle.add(
                "-detailschange",
                &[
                    pokemon_slot.as_str().into(),
                    details_with_tera.as_str().into(),
                ],
            );

            // Update max HP
            // JavaScript: this.updateMaxHp();
            // Note: updateMaxHp() in JS calculates new_base_max_hp from species.baseStats
            // For now, we skip HP update as it requires stat calculation infrastructure
            // TODO: Call Pokemon::update_max_hp() when stat calculation is available
            // let new_base_max_hp = battle.stat_modify(species_base_stats, &self.set, "hp");
            // Pokemon::update_max_hp(battle, (self.side_index, self.position), new_base_max_hp);

            // Handle source effects
            if let Some(source) = source_id.as_ref() {
                // Check if source is an Item
                if let Some(item_data) = battle.dex.items().get(source.as_str()) {
                    // Check for different item types
                    let is_zmove = item_data.z_move.is_some();
                    let is_primal = source.as_str().ends_with("orb")
                        && (source.as_str().contains("blue") || source.as_str().contains("red"));

                    // Add messages before modifying pokemon
                    if is_zmove {
                        // Ultra Burst
                        battle.add(
                            "-burst",
                            &[
                                pokemon_slot.as_str().into(),
                                species_base_species.unwrap_or(species_name.clone()).into(),
                                source.as_str().into(),
                            ],
                        );
                    } else if is_primal {
                        // Primal Reversion
                        battle.add(
                            "-primal",
                            &[pokemon_slot.as_str().into(), source.as_str().into()],
                        );
                    } else {
                        // Mega Evolution
                        battle.add(
                            "-mega",
                            &[
                                pokemon_slot.as_str().into(),
                                species_base_species
                                    .clone()
                                    .unwrap_or(species_name.clone())
                                    .into(),
                                source.as_str().into(),
                            ],
                        );
                    }

                    // Now modify pokemon fields
                    let pokemon = &mut battle.sides[pokemon_pos.0].pokemon[pokemon_pos.1];
                    pokemon.can_terastallize = None; // National Dex behavior

                    if is_zmove || !is_primal {
                        pokemon.move_this_turn_result = Some(true); // Ultra Burst/Mega counts as an action for Truant
                    }
                    pokemon.forme_regression = true;
                } else if battle.dex.conditions().get(source.as_str()).is_some() {
                    // Source is a Status (e.g., Shaymin-Sky -> Shaymin)
                    let mut args = vec![
                        pokemon_slot.as_str().into(),
                        species_name.clone().into(),
                    ];
                    if let Some(msg) = message {
                        args.push(msg.into());
                    }
                    battle.add("-formechange", &args);
                }
            }

            // Handle ability changes for permanent formes
            // if (isPermanent && (!source || !['disguise', 'iceface'].includes(source.id))) {
            let should_change_ability = if let Some(source) = source_id.as_ref() {
                source.as_str() != "disguise" && source.as_str() != "iceface"
            } else {
                true // No source means we should change ability (Tera forme)
            };

            if should_change_ability {
                // Get the ability from the new forme
                // const ability = species.abilities[abilitySlot] || species.abilities['0'];
                // For now, we use ability slot '0' (primary ability)
                // TODO: Use _ability_slot parameter when ability slot system is fully implemented

                if let Some(species_data) = battle.dex.species().get(species_id.as_str()) {
                    let ability_str = species_data.abilities.slot0.clone()
                        .or_else(|| species_data.abilities.hidden.clone())
                        .unwrap_or_else(|| "noability".to_string());

                    let ability_to_set = crate::dex_data::ID::from(ability_str.as_str());

                    // if (source || !this.getAbility().flags['cantsuppress'])
                    let should_override = if source_id.is_some() {
                        true
                    } else {
                        // Check if current ability can be suppressed
                        // For now, always allow override for Tera formes (no source)
                        true // TODO: Implement ability.flags.cantsuppress check
                    };

                    if should_override {
                        // JavaScript: this.setAbility(ability, null, null, true);
                        // source and sourceEffect are both null in JS
                        crate::pokemon::Pokemon::set_ability(
                            battle,
                            pokemon_pos,
                            ability_to_set.clone(),
                            None, // source_pos
                            None, // source_effect
                            true, // is_from_forme_change
                            false, // is_transform
                        );
                        battle.sides[pokemon_pos.0].pokemon[pokemon_pos.1].base_ability = ability_to_set;
                    }
                }
            }
        }

        // if (this.terastallized) {
        //     this.knownType = true;
        //     this.apparentType = this.terastallized;
        // }
        let pokemon = &mut battle.sides[pokemon_pos.0].pokemon[pokemon_pos.1];
        if let Some(ref tera_type) = pokemon.terastallized {
            pokemon.known_type = true;
            pokemon.apparent_type = tera_type.clone();
        }

        true
    }
}
