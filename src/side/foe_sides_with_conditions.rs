use crate::side::*;

impl Side {

    /// Iterate through foe side conditions
    /// Equivalent to side.ts foeSidesWithConditions()
    // TypeScript source:
    // /** Intended as a way to iterate through all foe side conditions - do not use for anything else. */
    // 	foeSidesWithConditions() {
    // 		if (this.battle.gameType === 'freeforall') return this.battle.sides.filter(side => side !== this);
    //
    // 		return [this.foe];
    // 	}
    //
    pub fn foe_sides_with_conditions<'a>(&self, battle_sides: &'a [Side]) -> Vec<&'a Side> {
        // JavaScript: return [this.foe] for non-freeforall games
        // This returns all foe sides (not self), regardless of whether they have conditions
        // The name "foeSidesWithConditions" is misleading - it's for iterating foe sides
        // when adding conditions, not filtering sides that already have conditions
        battle_sides
            .iter()
            .filter(|s| s.n != self.n)
            .collect()
    }
}
