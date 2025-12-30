use crate::*;

impl Pokemon {

    /// Heal HP
    // TypeScript source:
    // /** Returns the amount of damage actually healed */
    // 	heal(d: number, source: Pokemon | null = null, effect: Effect | null = null) {
    // 		if (!this.hp) return false;
    // 		d = this.battle.trunc(d);
    // 		if (isNaN(d)) return false;
    // 		if (d <= 0) return false;
    // 		if (this.hp >= this.maxhp) return false;
    // 		this.hp += d;
    // 		if (this.hp > this.maxhp) {
    // 			d -= this.hp - this.maxhp;
    // 			this.hp = this.maxhp;
    // 		}
    // 		return d;
    // 	}
    //
    pub fn heal(&mut self, amount: i32) -> i32 {
        if self.hp >= self.maxhp {
            return 0;
        }
        let actual = amount.min(self.maxhp - self.hp);
        self.hp += actual;
        actual
    }
}
