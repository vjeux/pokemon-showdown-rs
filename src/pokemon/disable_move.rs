use crate::*;

impl Pokemon {

    /// Disable a move
    //
    // 	disableMove(moveid: string, isHidden?: boolean, sourceEffect?: Effect) {
    // 		if (!sourceEffect && this.battle.event) {
    // 			sourceEffect = this.battle.effect;
    // 		}
    // 		moveid = toID(moveid);
    //
    // 		for (const moveSlot of this.moveSlots) {
    // 			if (moveSlot.id === moveid && moveSlot.disabled !== true) {
    // 				moveSlot.disabled = isHidden ? 'hidden' : true;
    // 				moveSlot.disabledSource = (sourceEffect?.name || moveSlot.move);
    // 			}
    // 		}
    // 	}
    //
    pub fn disable_move(&mut self, move_id: &str, source: Option<String>) {
        let id = crate::dex_data::to_id(move_id);
        if let Some(slot) = self.move_slots.iter_mut().find(|s| s.id.as_str() == id) {
            slot.disabled = true;
            slot.disabled_source = source;
        }
    }
}
