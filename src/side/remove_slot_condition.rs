use crate::side::*;

impl Side {

    /// Remove a slot condition
    //
    // 	removeSlotCondition(target: Pokemon | number, status: string | Effect) {
    // 		if (target instanceof Pokemon) target = target.position;
    // 		status = this.battle.dex.conditions.get(status) as Effect;
    // 		if (!this.slotConditions[target][status.id]) return false;
    // 		this.battle.singleEvent('End', status, this.slotConditions[target][status.id], this.active[target]);
    // 		delete this.slotConditions[target][status.id];
    // 		return true;
    // 	}
    //
    pub fn remove_slot_condition(&mut self, slot: usize, id: &ID) -> bool {
        self.slot_conditions
            .get_mut(slot)
            .map(|conds| conds.remove(id).is_some())
            .unwrap_or(false)
    }
}
