use crate::*;
use crate::battle_actions::MoveHitData;

impl Pokemon {

    /// Get move hit data for tracking hit results
    /// Equivalent to pokemon.ts getMoveHitData()
    ///
    /// Returns a reference to MoveHitData for this target's slot
    /// Note: This requires mutable access to Battle to get/create the entry
    //
    // 	getMoveHitData(move: ActiveMove) {
    // 		if (!move.moveHitData) move.moveHitData = {};
    // 		const slot = this.getSlot();
    // 		return move.moveHitData[slot] || (move.moveHitData[slot] = {
    // 			crit: false,
    // 			typeMod: 0,
    // 			zBrokeProtect: false,
    // 		});
    // 	}
    //
    /// Note: In Rust, we implement this as a Battle method since we need mutable access to active_move
    /// Use battle.get_move_hit_data(pokemon_pos) instead
    pub fn get_move_hit_data(&self, _move_id: &ID) -> MoveHitData {
        // This method is deprecated - use Battle::get_move_hit_data instead
        // Kept for compatibility but returns default
        MoveHitData::default()
    }
}
