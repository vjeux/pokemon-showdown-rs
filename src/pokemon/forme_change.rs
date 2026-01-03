//! Pokemon::forme_change implementation
//!
//! Pokemon Showdown - http://pokemonshowdown.com/

use crate::*;
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
    pub fn forme_change(
        &mut self,
        battle: &mut Battle,
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
        let set_species_result = self.set_species(battle, &species_id, source_id.as_ref(), false);

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

                if let Some(ability_name) = source_name {
                    // this.battle.add('-formechange', this, species.name, message, `[from] ability: ${source.name}`);
                    let pokemon_slot = self.get_slot();

                    use crate::battle::Arg;
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
                    let pokemon_slot = self.get_slot();

                    use crate::battle::Arg;
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
                let pokemon_slot = self.get_slot();

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
            // TODO: Implement permanent forme changes (detailschange, updateMaxHp, ability changes, etc.)
            // For now, just log that it's not implemented
            eprintln!("WARNING: Permanent forme changes not yet fully implemented");
            return false;
        }

        // if (this.terastallized) {
        //     this.knownType = true;
        //     this.apparentType = this.terastallized;
        // }
        // TODO: Implement terastallized type handling

        true
    }
}
