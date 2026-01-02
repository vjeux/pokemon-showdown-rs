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
    pub fn max_move_disabled(&self, battle: &Battle, base_move_id: &ID) -> bool {
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
        if let Some(base_move) = battle.dex.moves().get(base_move_id.as_str()) {
            if base_move.category == "Status" {
                return self.has_item(battle, &["assaultvest"]) || self.has_volatile(&ID::new("taunt"));
            }
        }

        false
    }
}
