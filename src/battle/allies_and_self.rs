use crate::*;

impl Battle {

    /// Get all allies including self
    /// Equivalent to pokemon.ts alliesAndSelf() which calls side.allies()
    ///
    // 	alliesAndSelf(): Pokemon[] {
    // 		return this.side.allies();
    // 	}
    //
    // side.allies(all?: boolean) implementation:
    // 	allies(all?: boolean) {
    // 		// called during the first switch-in, so `active` can still contain nulls at this point
    // 		let allies = this.activeTeam().filter(ally => ally);
    // 		if (!all) allies = allies.filter(ally => !!ally.hp);
    //
    // 		return allies;
    // 	}
    //
    pub fn allies_and_self(&self, side_idx: usize, include_fainted: bool) -> Vec<(usize, usize)> {
        let mut result = Vec::new();

        if let Some(side) = self.sides.get(side_idx) {
            // Iterate through active slots on this side
            for poke_idx in side.active.iter().flatten() {
                if let Some(pokemon) = side.pokemon.get(*poke_idx) {
                    // Include if: include_fainted=true OR pokemon has HP
                    if include_fainted || pokemon.hp > 0 {
                        result.push((side_idx, *poke_idx));
                    }
                }
            }
        }

        result
    }
}
