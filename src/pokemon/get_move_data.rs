use crate::*;
use crate::pokemon::MoveSlot;

impl Pokemon {

    /// Get move slot data
    //
    // 	getMoveData(move: string | Move) {
    // 		move = this.battle.dex.moves.get(move);
    // 		for (const moveSlot of this.moveSlots) {
    // 			if (moveSlot.id === move.id) {
    // 				return moveSlot;
    // 			}
    // 		}
    // 		return null;
    // 	}
    //
    pub fn get_move_data(&self, move_id: &ID) -> Option<&MoveSlot> {
        self.move_slots.iter().find(|slot| &slot.id == move_id)
    }
}
