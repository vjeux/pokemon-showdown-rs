use crate::*;

impl Pokemon {

    /// Check if Pokemon is in Sky Drop
    //
    // 	isSkyDropped() {
    // 		if (this.volatiles['skydrop']) return true;
    // 		for (const foeActive of this.side.foe.active) {
    // 			if (foeActive.volatiles['skydrop'] && foeActive.volatiles['skydrop'].source === this) {
    // 				return true;
    // 			}
    // 		}
    // 		return false;
    // 	}
    //
    pub fn is_sky_dropped(&self) -> bool {
        // JS: if (this.volatiles['skydrop']) return true;
        if self.has_volatile(&ID::new("skydrop")) {
            return true;
        }

        // JS: for (const foeActive of this.side.foe.active) {
        // JS:     if (foeActive.volatiles['skydrop'] && foeActive.volatiles['skydrop'].source === this) {
        // JS:         return true;
        // JS:     }
        // JS: }
        // JS: return false;
        //
        // Note: Missing check for foe active Pokemon with skydrop where source is this pokemon
        // Would need Battle reference to access side.foe.active
        // Would also need EffectState.source field to track who initiated Sky Drop
        false
    }
}
