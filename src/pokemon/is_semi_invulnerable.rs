use crate::*;

impl Pokemon {

    /// Check if Pokemon is semi-invulnerable (Fly, Dig, Dive, etc.)
    //
    // 	isSemiInvulnerable() {
    // 		return (this.volatiles['fly'] || this.volatiles['bounce'] || this.volatiles['dive'] || this.volatiles['dig'] ||
    // 			this.volatiles['phantomforce'] || this.volatiles['shadowforce'] || this.isSkyDropped());
    // 	}
    //
    pub fn is_semi_invulnerable(&self) -> bool {
        // JS: return (this.volatiles['fly'] || this.volatiles['bounce'] || this.volatiles['dive'] || this.volatiles['dig'] || this.volatiles['phantomforce'] || this.volatiles['shadowforce'] || this.isSkyDropped());
        self.has_volatile(&ID::new("fly"))
            || self.has_volatile(&ID::new("bounce"))
            || self.has_volatile(&ID::new("dive"))
            || self.has_volatile(&ID::new("dig"))
            || self.has_volatile(&ID::new("phantomforce"))
            || self.has_volatile(&ID::new("shadowforce"))
            || self.is_sky_dropped()
    }
}
