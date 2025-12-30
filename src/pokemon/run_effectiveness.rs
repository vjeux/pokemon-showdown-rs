use crate::*;

impl Pokemon {

    /// Run type effectiveness check
    /// Equivalent to runEffectiveness in pokemon.ts
    //
    // 	runEffectiveness(move: ActiveMove) {
    // 		let totalTypeMod = 0;
    // 		if (this.terastallized && move.type === 'Stellar') {
    // 			totalTypeMod = 1;
    // 		} else {
    // 			for (const type of this.getTypes()) {
    // 				let typeMod = this.battle.dex.getEffectiveness(move, type);
    // 				typeMod = this.battle.singleEvent('Effectiveness', move, null, this, type, move, typeMod);
    // 				totalTypeMod += this.battle.runEvent('Effectiveness', this, type, move, typeMod);
    // 			}
    // 		}
    // 		if (this.species.name === 'Terapagos-Terastal' && this.hasAbility('Tera Shell') &&
    // 			!this.battle.suppressingAbility(this)) {
    // 			if (this.abilityState.resisted) return -1; // all hits of multi-hit move should be not very effective
    // 			if (move.category === 'Status' || move.id === 'struggle' || !this.runImmunity(move) ||
    // 				totalTypeMod < 0 || this.hp < this.maxhp) {
    // 				return totalTypeMod;
    // 			}
    //
    // 			this.battle.add('-activate', this, 'ability: Tera Shell');
    // 			this.abilityState.resisted = true;
    // 			return -1;
    // 		}
    // 		return totalTypeMod;
    // 	}
    //
    pub fn run_effectiveness(&self, move_type: &str) -> f64 {
        crate::data::typechart::get_effectiveness_multi(move_type, &self.types)
    }
}
