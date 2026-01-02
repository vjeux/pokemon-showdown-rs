use crate::*;

impl Pokemon {

    /// Run immunity check
    /// Equivalent to runImmunity in pokemon.ts
    // TypeScript source:
    // /** false = immune, true = not immune */
    // 	runImmunity(source: ActiveMove | string, message?: string | boolean) {
    // 		if (!source) return true;
    // 		const type: string = typeof source !== 'string' ? source.type : source;
    // 		if (typeof source !== 'string') {
    // 			if (source.ignoreImmunity && (source.ignoreImmunity === true || source.ignoreImmunity[type])) {
    // 				return true;
    // 			}
    // 		}
    // 		if (!type || type === '???') return true;
    // 		if (!this.battle.dex.types.isName(type)) {
    // 			throw new Error("Use runStatusImmunity for " + type);
    // 		}
    //
    // 		const negateImmunity = !this.battle.runEvent('NegateImmunity', this, type);
    // 		const notImmune = type === 'Ground' ?
    // 			this.isGrounded(negateImmunity) :
    // 			negateImmunity || this.battle.dex.getImmunity(type, this);
    // 		if (notImmune) return true;
    // 		if (!message) return false;
    // 		if (notImmune === null) {
    // 			this.battle.add('-immune', this, '[from] ability: Levitate');
    // 		} else {
    // 			this.battle.add('-immune', this);
    // 		}
    // 		return false;
    // 	}
    //
    pub fn run_immunity(&self, move_type: &str) -> bool {
        // JS: const negateImmunity = !this.battle.runEvent('NegateImmunity', this, type);
        // JS: const notImmune = type === 'Ground' ?
        // JS:     this.isGrounded(negateImmunity) :
        // JS:     negateImmunity || this.battle.dex.getImmunity(type, this);
        // JS: if (notImmune) return true;
        // JS: return false;
        //
        // Note: Simplified implementation. Full implementation would require:
        // - Refactoring to associated function taking Battle reference
        // - Calling runEvent('NegateImmunity')
        // - Calling isGrounded() for Ground type
        // - Calling battle.dex.getImmunity() for proper type immunity checks
        // - Adding battle.add('-immune') messages when appropriate
        let effectiveness = self.run_effectiveness(move_type);
        effectiveness > 0.0
    }
}
