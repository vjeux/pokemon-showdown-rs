use crate::side::*;
use crate::*;

impl Side {

    /// Get side condition state
    //
    // 	getSideCondition(status: string | Effect): Effect | null {
    // 		status = this.battle.dex.conditions.get(status) as Effect;
    // 		if (!this.sideConditions[status.id]) return null;
    // 		return status;
    // 	}
    //
    pub fn get_side_condition(&self, id: &ID) -> Option<&EffectState> {
        self.side_conditions.get(id)
    }
}
