use crate::*;

impl Pokemon {

    /// Check if Pokemon has a specific move
    //
    // 	hasMove(moveid: string) {
    // 		moveid = toID(moveid);
    // 		if (moveid.substr(0, 11) === 'hiddenpower') moveid = 'hiddenpower';
    // 		for (const moveSlot of this.moveSlots) {
    // 			if (moveid === moveSlot.id) {
    // 				return moveid;
    // 			}
    // 		}
    // 		return false;
    // 	}
    //
    pub fn has_move(&self, move_id: &str) -> bool {
        let id = crate::dex_data::to_id(move_id);
        self.move_slots.iter().any(|slot| slot.id.as_str() == id)
    }
}
