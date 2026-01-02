use crate::*;

impl Pokemon {

    /// Mark Pokemon as fainted and queue faint
    /// Returns the amount of damage dealt (HP before faint)
    // TypeScript source:
    // /**
    // 	 * This function only puts the pokemon in the faint queue;
    // 	 * actually setting of this.fainted comes later when the
    // 	 * faint queue is resolved.
    // 	 *
    // 	 * Returns the amount of damage actually dealt
    // 	 */
    // 	faint(source: Pokemon | null = null, effect: Effect | null = null) {
    // 		if (this.fainted || this.faintQueued) return 0;
    // 		const d = this.hp;
    // 		this.hp = 0;
    // 		this.switchFlag = false;
    // 		this.faintQueued = true;
    // 		this.battle.faintQueue.push({
    // 			target: this,
    // 			source,
    // 			effect,
    // 		});
    // 		return d;
    // 	}
    //
    pub fn faint(&mut self) -> i32 {
        // JS: if (this.fainted || this.faintQueued) return 0;
        if self.fainted || self.faint_queued {
            return 0;
        }

        // JS: const d = this.hp;
        let damage = self.hp;

        // JS: this.hp = 0;
        self.hp = 0;

        // JS: this.switchFlag = false;
        self.switch_flag = false;

        // JS: this.faintQueued = true;
        self.faint_queued = true;

        // JS: this.battle.faintQueue.push({
        // JS:     target: this,
        // JS:     source,
        // JS:     effect,
        // JS: });
        // Note: Missing source and effect parameters
        // Note: Missing battle.faintQueue.push() - would need Battle reference
        // Pokemon is marked as faint_queued but not added to battle's faint queue
        // This is a borrow checker workaround - caller must add to faint queue

        // JS: return d;
        damage
    }
}
