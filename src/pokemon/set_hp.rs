use crate::*;

impl Pokemon {

    /// Set HP directly, returns delta
    // TypeScript source:
    // /** Sets HP, returns delta */
    // 	sethp(d: number) {
    // 		if (!this.hp) return 0;
    // 		d = this.battle.trunc(d);
    // 		if (isNaN(d)) return;
    // 		if (d < 1) d = 1;
    // 		d -= this.hp;
    // 		this.hp += d;
    // 		if (this.hp > this.maxhp) {
    // 			d -= this.hp - this.maxhp;
    // 			this.hp = this.maxhp;
    // 		}
    // 		return d;
    // 	}
    //
    pub fn set_hp(&mut self, hp: i32) -> i32 {
        if self.hp == 0 {
            return 0;
        }
        let clamped = hp.clamp(1, self.maxhp);
        let delta = clamped - self.hp;
        self.hp = clamped;
        delta
    }
}
