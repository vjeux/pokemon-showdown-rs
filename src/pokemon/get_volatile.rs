use crate::*;
use crate::event_system::EffectState;

impl Pokemon {

    /// Get volatile state
    //
    // 	getVolatile(status: string | Effect) {
    // 		status = this.battle.dex.conditions.get(status) as Effect;
    // 		if (!this.volatiles[status.id]) return null;
    // 		return status;
    // 	}
    //
    pub fn get_volatile(&self, id: &ID) -> Option<&EffectState> {
        self.volatiles.get(id)
    }
}
