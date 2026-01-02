use crate::*;
use crate::pokemon::Attacker;

impl Pokemon {

    /// Get last damager info
    /// Equivalent to getLastDamagedBy in pokemon.ts
    //
    // 	getLastDamagedBy(filterOutSameSide: boolean) {
    // 		const damagedBy: Attacker[] = this.attackedBy.filter(attacker => (
    // 			typeof attacker.damageValue === 'number' &&
    // 			(filterOutSameSide === undefined || !this.isAlly(attacker.source))
    // 		));
    // 		if (damagedBy.length === 0) return undefined;
    // 		return damagedBy[damagedBy.length - 1];
    // 	}
    //
    pub fn get_last_damaged_by(&self, _filter_out_same_side: bool) -> Option<Attacker> {
        // JS: const damagedBy: Attacker[] = this.attackedBy.filter(attacker => (
        // JS:     typeof attacker.damageValue === 'number' &&
        // JS:     (filterOutSameSide === undefined || !this.isAlly(attacker.source))
        // JS: ));
        // JS: if (damagedBy.length === 0) return undefined;
        // JS: return damagedBy[damagedBy.length - 1];
        //
        // Note: Missing attackedBy: Vec<Attacker> field on Pokemon struct
        // Note: Missing tracking of all attacks in battle
        // Note: Missing filtering by damageValue type (number)
        // Note: Missing isAlly check when filterOutSameSide is true
        // Note: Should return last attacker that dealt damage
        //
        // Full implementation needs:
        // - attackedBy: Vec<Attacker> field on Pokemon
        // - Tracking of all attacks in battle (push to attackedBy on damage)
        // - Filter by typeof attacker.damageValue === 'number'
        // - Filter by !this.isAlly(attacker.source) when filterOutSameSide
        // - Return last element of filtered array
        None
    }
}
