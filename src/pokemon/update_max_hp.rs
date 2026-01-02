use crate::*;

impl Pokemon {

    /// Update max HP (for forme changes)
    //
    // 	updateMaxHp() {
    // 		const newBaseMaxHp = this.battle.statModify(this.species.baseStats, this.set, 'hp');
    // 		if (newBaseMaxHp === this.baseMaxhp) return;
    // 		this.baseMaxhp = newBaseMaxHp;
    // 		const newMaxHP = this.volatiles['dynamax'] ? (2 * this.baseMaxhp) : this.baseMaxhp;
    // 		this.hp = this.hp <= 0 ? 0 : Math.max(1, newMaxHP - (this.maxhp - this.hp));
    // 		this.maxhp = newMaxHP;
    // 		if (this.hp) this.battle.add('-heal', this, this.getHealth, '[silent]');
    // 	}
    //
    pub fn update_max_hp(&mut self, new_base_max_hp: i32) {
        // TODO: implement the same logic as JavaScript

        if new_base_max_hp == self.base_maxhp {
            return;
        }
        self.base_maxhp = new_base_max_hp;
        let old_maxhp = self.maxhp;
        self.maxhp = new_base_max_hp;

        // Adjust current HP proportionally
        if self.hp > 0 {
            let hp_lost = old_maxhp - self.hp;
            self.hp = self.maxhp.saturating_sub(hp_lost).max(1);
        }
    }
}
