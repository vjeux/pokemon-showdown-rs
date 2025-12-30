use crate::*;

impl Pokemon {

    /// Check if Pokemon is protected
    // TypeScript source:
    // /** Specifically: is protected against a single-target damaging move */
    // 	isProtected() {
    // 		return !!(
    // 			this.volatiles['protect'] || this.volatiles['detect'] || this.volatiles['maxguard'] ||
    // 			this.volatiles['kingsshield'] || this.volatiles['spikyshield'] || this.volatiles['banefulbunker'] ||
    // 			this.volatiles['obstruct'] || this.volatiles['silktrap'] || this.volatiles['burningbulwark']
    // 		);
    // 	}
    //
    pub fn is_protected(&self) -> bool {
        self.has_volatile(&ID::new("protect"))
            || self.has_volatile(&ID::new("banefulbunker"))
            || self.has_volatile(&ID::new("kingsshield"))
            || self.has_volatile(&ID::new("spikyshield"))
            || self.has_volatile(&ID::new("silktrap"))
            || self.has_volatile(&ID::new("burningbulwark"))
    }
}
