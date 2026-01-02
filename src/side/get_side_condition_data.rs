// JS Source:
// 
// 	getSideConditionData(status: string | Effect): AnyObject {
// 		status = this.battle.dex.conditions.get(status) as Effect;
// 		return this.sideConditions[status.id] || null;
// 	}


use crate::side::*;

impl Side {

    /// Get side condition data
    /// Equivalent to side.ts getSideConditionData()
    //
    // 	getSideConditionData(status: string | Effect): AnyObject {
    // 		status = this.battle.dex.conditions.get(status) as Effect;
    // 		return this.sideConditions[status.id] || null;
    // 	}
    //
    pub fn get_side_condition_data(&self, id: &ID) -> Option<&EffectState> {
        self.side_conditions.get(id)
    }
}
