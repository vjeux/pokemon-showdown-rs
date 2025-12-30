use crate::side::*;
use crate::*;

impl Side {

    /// Remove a side condition
    //
    // 	removeSideCondition(status: string | Effect): boolean {
    // 		status = this.battle.dex.conditions.get(status) as Effect;
    // 		if (!this.sideConditions[status.id]) return false;
    // 		this.battle.singleEvent('SideEnd', status, this.sideConditions[status.id], this);
    // 		delete this.sideConditions[status.id];
    // 		return true;
    // 	}
    //
    pub fn remove_side_condition(&mut self, id: &ID) -> bool {
        self.side_conditions.remove(id).is_some()
    }
}
