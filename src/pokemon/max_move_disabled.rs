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
        // Check if the base move has PP
        if let Some(move_data) = self.get_move_data(base_move_id) {
            if move_data.pp == 0 {
                return true;
            }
        }
        // Status moves are disabled by Assault Vest or Taunt when dynamaxed
        // Would need move data to check category
        false
    }
}
