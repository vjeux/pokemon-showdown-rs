use crate::side::*;
use crate::event_system::SharedEffectState;

impl Side {

    /// Get slot condition data
    //
    // 	getSlotCondition(target: Pokemon | number, status: string | Effect) {
    // 		if (target instanceof Pokemon) target = target.position;
    // 		status = this.battle.dex.conditions.get(status) as Effect;
    // 		if (!this.slotConditions[target][status.id]) return null;
    // 		return status;
    // 	}
    //
    pub fn get_slot_condition(&self, slot: usize, id: &ID) -> Option<&SharedEffectState> {
        self.slot_conditions.get(slot)?.get(id)
    }
}
