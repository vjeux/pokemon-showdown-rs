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
    pub fn get_undynamaxed_hp(&self, amount: Option<i32>) -> i32 {
        // const hp = amount || this.hp;
        let hp = amount.unwrap_or(self.hp);

        // if (this.volatiles['dynamax'])
        if self.has_volatile(&ID::new("dynamax")) {
            // return Math.ceil(hp * this.baseMaxhp / this.maxhp);
            ((hp as f64) * (self.base_maxhp as f64) / (self.maxhp as f64)).ceil() as i32
        } else {
            // return hp;
            hp
        }
    }
}
