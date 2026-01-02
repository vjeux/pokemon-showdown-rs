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
        // TODO: implement the same logic as JavaScript
        self.has_volatile(&ID::new("skydrop"))
    }
}
