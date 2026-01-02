use crate::*;

impl Pokemon {

    /// Check if max move is disabled
    // TypeScript source:
    // /** This should be passed the base move and not the corresponding max move so we can check how much PP is left. */
    // 	maxMoveDisabled(baseMove: Move | string) {
    // 		baseMove = this.battle.dex.moves.get(baseMove);
    // 		if (!this.getMoveData(baseMove.id)?.pp) return true;
    // 		return !!(baseMove.category === 'Status' && (this.hasItem('assaultvest') || this.volatiles['taunt']));
    // 	}
    //
    pub fn max_move_disabled(&self, base_move_id: &ID) -> bool {
        // JS: baseMove = this.battle.dex.moves.get(baseMove);
        // JS: if (!this.getMoveData(baseMove.id)?.pp) return true;
        if let Some(move_data) = self.get_move_data(base_move_id) {
            if move_data.pp == 0 {
                return true;
            }
        } else {
            // No move data found means move doesn't exist in moveset
            return true;
        }

        // JS: return !!(baseMove.category === 'Status' && (this.hasItem('assaultvest') || this.volatiles['taunt']));
        //
        // Note: Missing move category check
        // Would need Battle reference to get move data from dex
        // Should check: move.category == 'Status' && (hasItem('assaultvest') || has_volatile('taunt'))
        false
    }
}
