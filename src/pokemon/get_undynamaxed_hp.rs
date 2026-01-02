use crate::*;

impl Pokemon {

    /// Get HP as if not dynamaxed
    //
    // 	getUndynamaxedHP(amount?: number) {
    // 		const hp = amount || this.hp;
    // 		if (this.volatiles['dynamax']) {
    // 			return Math.ceil(hp * this.baseMaxhp / this.maxhp);
    // 		}
    // 		return hp;
    // 	}
    //
    pub fn get_undynamaxed_hp(&self) -> i32 {
        if self.has_volatile(&ID::new("dynamax")) {
            // Dynamaxed HP is doubled, so divide by 2
            ((self.hp as f64) * (self.base_maxhp as f64) / (self.maxhp as f64)).ceil() as i32
        } else {
            self.hp
        }
    }
}
