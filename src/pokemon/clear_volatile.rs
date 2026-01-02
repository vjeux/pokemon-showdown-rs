// JS Source:
//
// 	clearVolatile(includeSwitchFlags = true) {
// 		this.boosts = {
// 			atk: 0,
// 			def: 0,
// 			spa: 0,
// 			spd: 0,
// 			spe: 0,
// 			accuracy: 0,
// 			evasion: 0,
// 		};
//
// 		if (this.battle.gen === 1 && this.baseMoves.includes('mimic' as ID) && !this.transformed) {
// 			const moveslot = this.baseMoves.indexOf('mimic' as ID);
// 			const mimicPP = this.moveSlots[moveslot] ? this.moveSlots[moveslot].pp : 16;
// 			this.moveSlots = this.baseMoveSlots.slice();
// 			this.moveSlots[moveslot].pp = mimicPP;
// 		} else {
// 			this.moveSlots = this.baseMoveSlots.slice();
// 		}
//
// 		this.transformed = false;
// 		this.ability = this.baseAbility;
// 		this.hpType = this.baseHpType;
// 		this.hpPower = this.baseHpPower;
// 		if (this.canTerastallize === false) this.canTerastallize = this.teraType;
// 		for (const i in this.volatiles) {
// 			if (this.volatiles[i].linkedStatus) {
// 				this.removeLinkedVolatiles(this.volatiles[i].linkedStatus, this.volatiles[i].linkedPokemon);
// 			}
// 		}
// 		if (this.species.name === 'Eternatus-Eternamax' && this.volatiles['dynamax']) {
// 			this.volatiles = { dynamax: this.volatiles['dynamax'] };
// 		} else {
// 			this.volatiles = {};
// 		}
// 		if (includeSwitchFlags) {
// 			this.switchFlag = false;
// 			this.forceSwitchFlag = false;
// 		}
//
// 		this.lastMove = null;
// 		if (this.battle.gen === 2) this.lastMoveEncore = null;
// 		this.lastMoveUsed = null;
// 		this.moveThisTurn = '';
// 		this.moveLastTurnResult = undefined;
// 		this.moveThisTurnResult = undefined;
//
// 		this.lastDamage = 0;
// 		this.attackedBy = [];
// 		this.hurtThisTurn = null;
// 		this.newlySwitched = true;
// 		this.beingCalledBack = false;
//
// 		this.volatileStaleness = undefined;
//
// 		delete this.abilityState.started;
// 		delete this.itemState.started;
//
// 		this.setSpecies(this.baseSpecies);
// 	}
//
// Note: In Rust, this method requires mutable access to Battle to call set_species()

use crate::*;
use crate::dex_data::BoostsTable;

impl Pokemon {
    /// Clear volatile conditions and reset to switch-in state
    /// Equivalent to pokemon.ts clearVolatile()
    pub fn clear_volatile(&mut self, battle: &mut Battle, include_switch_flags: bool) {
        // JS: this.boosts = { atk: 0, def: 0, spa: 0, spd: 0, spe: 0, accuracy: 0, evasion: 0 };
        self.boosts = BoostsTable::default();

        // JS: if (this.battle.gen === 1 && this.baseMoves.includes('mimic') && !this.transformed) {...}
        // Handle Gen 1 Mimic special case - preserve PP when resetting move slots
        if battle.gen == 1 && !self.transformed {
            // Check if base moves include mimic
            let mimic_id = ID::from("mimic");
            if let Some(mimic_slot_idx) = self.base_move_slots.iter().position(|slot| slot.id == mimic_id) {
                // Save the current PP for mimic
                let mimic_pp = self.move_slots.get(mimic_slot_idx)
                    .map(|slot| slot.pp)
                    .unwrap_or(16);

                // Reset move slots
                self.move_slots = self.base_move_slots.clone();

                // Restore mimic PP
                if let Some(slot) = self.move_slots.get_mut(mimic_slot_idx) {
                    slot.pp = mimic_pp;
                }
            } else {
                // No mimic, just reset normally
                self.move_slots = self.base_move_slots.clone();
            }
        } else {
            // JS: this.moveSlots = this.baseMoveSlots.slice();
            self.move_slots = self.base_move_slots.clone();
        }

        // JS: this.transformed = false;
        self.transformed = false;

        // JS: this.ability = this.baseAbility;
        self.ability = self.base_ability.clone();

        // JS: this.hpType = this.baseHpType;
        // Note: hpPower is not in the Rust Pokemon struct (Hidden Power power is fixed in modern gens)
        // Keep hp_type as-is since there's no baseHpType tracking

        // JS: if (this.canTerastallize === false) this.canTerastallize = this.teraType;
        if self.can_terastallize == Some("false".to_string()) {
            self.can_terastallize = self.tera_type.clone();
        }

        // JS: for (const i in this.volatiles) { if (this.volatiles[i].linkedStatus) {...} }
        // Note: Missing removeLinkedVolatiles() call for each linked volatile
        // This requires properly implementing linked_pokemon and linked_status in EffectState.data
        // For now, skip this step as the infrastructure isn't fully implemented
        // See remove_linked_volatiles.rs for details on what's needed

        // JS: if (this.species.name === 'Eternatus-Eternamax' && this.volatiles['dynamax']) {...}
        // Special case for Eternamax - preserve dynamax volatile
        if self.species_id.as_str() == "eternatuseternamax" {
            let dynamax_id = ID::from("dynamax");
            if let Some(dynamax_state) = self.volatiles.get(&dynamax_id).cloned() {
                self.volatiles.clear();
                self.volatiles.insert(dynamax_id, dynamax_state);
            } else {
                self.volatiles.clear();
            }
        } else {
            // JS: this.volatiles = {};
            self.volatiles.clear();
        }

        // JS: if (includeSwitchFlags) {...}
        if include_switch_flags {
            self.switch_flag = false;
            self.force_switch_flag = false;
        }

        // JS: this.lastMove = null;
        // JS: if (this.battle.gen === 2) this.lastMoveEncore = null;  // Not tracked in Rust
        // JS: this.lastMoveUsed = null;
        // JS: this.moveThisTurn = '';
        // JS: this.moveLastTurnResult = undefined;
        // JS: this.moveThisTurnResult = undefined;
        self.last_move = None;
        self.last_move_used = None;
        self.move_this_turn = None;
        self.move_last_turn_result = None;
        self.move_this_turn_result = None;

        // JS: this.lastDamage = 0;
        // JS: this.attackedBy = [];
        // JS: this.hurtThisTurn = null;
        self.last_damage = 0;
        self.attacked_by.clear();
        self.hurt_this_turn = None;

        // JS: this.newlySwitched = true;
        // JS: this.beingCalledBack = false;
        self.newly_switched = true;
        self.being_called_back = false;

        // JS: this.volatileStaleness = undefined;
        self.volatile_staleness = None;

        // JS: delete this.abilityState.started;
        // JS: delete this.itemState.started;
        // Note: EffectState in Rust doesn't have a started field, so nothing to delete

        // JS: this.setSpecies(this.baseSpecies);
        let base_species = self.base_species.clone();
        self.set_species(battle, &base_species, None, false);
    }
}
