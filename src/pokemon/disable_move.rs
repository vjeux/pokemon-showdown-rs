// JS Source:
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
// Note: In Rust, MoveSlot.disabled is bool, not bool|'hidden', so we can't
// track 'hidden' status. We track disabled as bool only.

use crate::*;
use crate::battle::Effect;

impl Pokemon {
    /// Disable a move
    /// Equivalent to pokemon.ts disableMove()
    pub fn disable_move(&mut self, move_id: &str, _is_hidden: bool, source_effect: Option<&Effect>) {
        // JS: moveid = toID(moveid);
        let id = crate::dex_data::to_id(move_id);

        // Get source effect name if provided
        // JS: moveSlot.disabledSource = (sourceEffect?.name || moveSlot.move);
        let source_effect_name = source_effect.map(|eff| {
            // Get the name from the effect based on its type
            eff.id.to_string()
        });

        // JS: for (const moveSlot of this.moveSlots) {
        for slot in &mut self.move_slots {
            // JS: if (moveSlot.id === moveid && moveSlot.disabled !== true) {
            if slot.id.as_str() == id && !slot.disabled {
                // JS: moveSlot.disabled = isHidden ? 'hidden' : true;
                // Note: In Rust, disabled is bool, can't be 'hidden'
                // We just set to true for both cases
                slot.disabled = true;

                // JS: moveSlot.disabledSource = (sourceEffect?.name || moveSlot.move);
                slot.disabled_source = source_effect_name.clone().or_else(|| Some(slot.move_name.clone()));
            }
        }
    }
}
