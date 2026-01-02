// NOTE: This method is NOT in JavaScript - Rust-specific implementation

use crate::*;

impl Pokemon {

    // =========================================================================
    // TARGET METHODS (ported from pokemon.ts)
    // These methods return indices instead of Pokemon references since the
    // actual Pokemon are owned by the Battle.
    // =========================================================================

    /// Get all allies including self
    /// Equivalent to pokemon.ts alliesAndSelf()
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
    // Note: In Rust, Pokemon doesn't have a reference to Battle (borrow checker),
    // so we take Battle as a parameter instead of accessing this.battle
    pub fn allies_and_self(&self, battle: &Battle, include_fainted: bool) -> Vec<(usize, usize)> {
        let mut result = Vec::new();

        if let Some(side) = battle.sides.get(self.side_index) {
            // Iterate through active slots on this side
            for poke_idx in side.active.iter().flatten() {
                if let Some(pokemon) = side.pokemon.get(*poke_idx) {
                    // Include if: include_fainted=true OR pokemon has HP
                    if include_fainted || pokemon.hp > 0 {
                        result.push((self.side_index, *poke_idx));
                    }
                }
            }
        }

        result
    }
}
