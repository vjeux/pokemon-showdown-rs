use crate::*;
use crate::event_system::EffectState;

impl Pokemon {

    /// Forme change
    // TypeScript source:
    // /**
    // 	 * Changes this Pokemon's forme to match the given speciesId (or species).
    // 	 * This function handles all changes to stats, ability, type, species, etc.
    // 	 * as well as sending all relevant messages sent to the client.
    // 	 */
    // 	formeChange(
    // 		speciesId: string | Species, source: Effect | null = this.battle.effect,
    // 		isPermanent?: boolean, abilitySlot = '0', message?: string
    // 	) {
    // 		const rawSpecies = this.battle.dex.species.get(speciesId);
    //
    // 		const species = this.setSpecies(rawSpecies, source);
    // 		if (!species) return false;
    //
    // 		if (this.battle.gen <= 2) return true;
    //
    // 		// The species the opponent sees
    // 		const apparentSpecies =
    // 			this.illusion ? this.illusion.species.name : species.baseSpecies;
    // 		if (isPermanent) {
    // 			this.baseSpecies = rawSpecies;
    // 			this.details = this.getUpdatedDetails();
    // 			let details = (this.illusion || this).details;
    // 			if (this.terastallized) details += `, tera:${this.terastallized}`;
    // 			this.battle.add('detailschange', this, details);
    // 			this.updateMaxHp();
    // 			if (!source) {
    // 				// Tera forme
    // 				// Ogerpon/Terapagos text goes here
    // 				this.formeRegression = true;
    // 			} else if (source.effectType === 'Item') {
    // 				this.canTerastallize = null; // National Dex behavior
    // 				if (source.zMove) {
    // 					this.battle.add('-burst', this, apparentSpecies, species.requiredItem);
    // 					this.moveThisTurnResult = true; // Ultra Burst counts as an action for Truant
    // 				} else if (source.isPrimalOrb) {
    // 					if (this.illusion) {
    // 						this.ability = '';
    // 						this.battle.add('-primal', this.illusion, species.requiredItem);
    // 					} else {
    // 						this.battle.add('-primal', this, species.requiredItem);
    // 					}
    // 				} else {
    // 					this.battle.add('-mega', this, apparentSpecies, species.requiredItem);
    // 					this.moveThisTurnResult = true; // Mega Evolution counts as an action for Truant
    // 				}
    // 				this.formeRegression = true;
    // 			} else if (source.effectType === 'Status') {
    // 				// Shaymin-Sky -> Shaymin
    // 				this.battle.add('-formechange', this, species.name, message);
    // 			}
    // 		} else {
    // 			if (source?.effectType === 'Ability') {
    // 				this.battle.add('-formechange', this, species.name, message, `[from] ability: ${source.name}`);
    // 			} else {
    // 				this.battle.add('-formechange', this, this.illusion ? this.illusion.species.name : species.name, message);
    // 			}
    // 		}
    // 		if (isPermanent && (!source || !['disguise', 'iceface'].includes(source.id))) {
    // 			if (this.illusion && source) {
    // 				// Tera forme by Ogerpon or Terapagos breaks the Illusion
    // 				this.ability = ''; // Don't allow Illusion to wear off
    // 			}
    // 			const ability = species.abilities[abilitySlot] || species.abilities['0'];
    // 			// Ogerpon's forme change doesn't override permanent abilities
    // 			if (source || !this.getAbility().flags['cantsuppress']) this.setAbility(ability, null, null, true);
    // 			// However, its ability does reset upon switching out
    // 			this.baseAbility = toID(ability);
    // 		}
    // 		if (this.terastallized) {
    // 			this.knownType = true;
    // 			this.apparentType = this.terastallized;
    // 		}
    // 		return true;
    // 	}
    //
    pub fn forme_change(
        &mut self,
        new_species_id: ID,
        new_types: Vec<String>,
        new_ability: Option<ID>,
    ) {
        self.species_id = new_species_id;
        self.types = new_types;
        if let Some(ability) = new_ability {
            self.ability = ability.clone();
            self.ability_state = EffectState::new(ability);
            self.ability_state.target = Some((self.side_index, self.position));
        }
    }
}
