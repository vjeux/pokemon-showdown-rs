use crate::*;
use crate::battle_actions::ActiveMove;

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
    pub fn has_move(&self, active_move: &ActiveMove) -> bool {
        // JS: moveid = toID(moveid);
        let mut id = active_move.id.as_str().to_string();

        // JS: if (moveid.substr(0, 11) === 'hiddenpower') moveid = 'hiddenpower';
        if id.starts_with("hiddenpower") {
            id = "hiddenpower".to_string();
        }

        // JS: for (const moveSlot of this.moveSlots) { if (moveid === moveSlot.id) return moveid; }
        // JS: return false;
        self.move_slots.iter().any(|slot| slot.id.as_str() == id)
    }
}
