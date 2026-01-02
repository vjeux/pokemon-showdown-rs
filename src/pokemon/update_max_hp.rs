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
        // JS: const newBaseMaxHp = this.battle.statModify(this.species.baseStats, this.set, 'hp');
        // Note: Takes new_base_max_hp as parameter instead of calculating from species
        // Note: JavaScript calculates it from species.baseStats and set

        // JS: if (newBaseMaxHp === this.baseMaxhp) return;
        if new_base_max_hp == self.base_maxhp {
            return;
        }

        // JS: this.baseMaxhp = newBaseMaxHp;
        self.base_maxhp = new_base_max_hp;

        // JS: const newMaxHP = this.volatiles['dynamax'] ? (2 * this.baseMaxhp) : this.baseMaxhp;
        let old_maxhp = self.maxhp;
        let new_max_hp = if self.has_volatile(&ID::new("dynamax")) {
            2 * new_base_max_hp
        } else {
            new_base_max_hp
        };
        self.maxhp = new_max_hp;

        // JS: this.hp = this.hp <= 0 ? 0 : Math.max(1, newMaxHP - (this.maxhp - this.hp));
        // Adjust current HP proportionally
        if self.hp > 0 {
            let hp_lost = old_maxhp - self.hp;
            self.hp = new_max_hp.saturating_sub(hp_lost).max(1);
        }

        // JS: if (this.hp) this.battle.add('-heal', this, this.getHealth, '[silent]');
        // Note: Missing battle.add('-heal') message
        // Note: Would need Battle reference
    }
}
