// TODO: Implement clearVolatile from JavaScript
//
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

use crate::*;

impl Pokemon {
    // TODO: Implement this method
}
