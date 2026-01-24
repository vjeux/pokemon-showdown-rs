// JS Source:
//
// 	getSideCondition(status: string | Effect): Effect | null {
// 		status = this.battle.dex.conditions.get(status) as Effect;
// 		if (!this.sideConditions[status.id]) return null;
// 		return status;
// 	}


use crate::side::*;
use crate::event_system::SharedEffectState;

impl Side {

    /// Get side condition state
    //
    // 	getSideCondition(status: string | Effect): Effect | null {
    // 		status = this.battle.dex.conditions.get(status) as Effect;
    // 		if (!this.sideConditions[status.id]) return null;
    // 		return status;
    // 	}
    //
    pub fn get_side_condition(&self, id: &ID) -> Option<&SharedEffectState> {
        self.side_conditions.get(id)
    }
}
