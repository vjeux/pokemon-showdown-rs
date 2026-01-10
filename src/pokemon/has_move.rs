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
    // TODO: Verify move parameter type matches JavaScript's ActiveMove usage
    pub fn has_move(&self, move_id: &str) -> bool {
        // JS: moveid = toID(moveid);
        let mut id = crate::dex_data::to_id(move_id);

        // JS: if (moveid.substr(0, 11) === 'hiddenpower') moveid = 'hiddenpower';
        if id.starts_with("hiddenpower") {
            id = "hiddenpower".to_string();
        }

        // JS: for (const moveSlot of this.moveSlots) { if (moveid === moveSlot.id) return moveid; }
        // JS: return false;
        self.move_slots.iter().any(|slot| slot.id.as_str() == id)
    }
}
