use crate::side::*;
use crate::*;

impl Side {

    /// Get allies (all active Pokemon on this side)
    // 	allies(all?: boolean) {
    // 		// called during the first switch-in, so `active` can still contain nulls at this point
    // 		let allies = this.activeTeam().filter(ally => ally);
    // 		if (!all) allies = allies.filter(ally => !!ally.hp);
    //
    // 		return allies;
    // 	}
    //
    pub fn allies(&self, include_fainted: bool) -> Vec<usize> {
        let mut allies = Vec::new();
        for idx in self.active.iter().flatten() {
            if let Some(pokemon) = self.pokemon.get(*idx) {
                if include_fainted || !pokemon.is_fainted() {
                    allies.push(*idx);
                }
            }
        }
        allies
    }
}
