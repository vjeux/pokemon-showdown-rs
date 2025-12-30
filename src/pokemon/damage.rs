use crate::*;

impl Pokemon {

    /// Apply damage to Pokemon
    /// Returns actual damage dealt
    //
    // 	damage(d: number, source: Pokemon | null = null, effect: Effect | null = null) {
    // 		if (!this.hp || isNaN(d) || d <= 0) return 0;
    // 		if (d < 1 && d > 0) d = 1;
    // 		d = this.battle.trunc(d);
    // 		this.hp -= d;
    // 		if (this.hp <= 0) {
    // 			d += this.hp;
    // 			this.faint(source, effect);
    // 		}
    // 		return d;
    // 	}
    //
    pub fn damage(&mut self, amount: i32) -> i32 {
        if self.hp == 0 || amount == 0 {
            return 0;
        }
        let actual = amount.min(self.hp);
        self.hp = self.hp.saturating_sub(amount);
        if self.hp == 0 {
            self.faint_queued = true;
        }
        actual
    }
}
