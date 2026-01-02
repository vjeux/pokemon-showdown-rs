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
        if self.fainted || self.faint_queued {
            return 0;
        }
        let damage = self.hp;
        self.hp = 0;
        self.switch_flag = false;
        self.faint_queued = true;

        // TODO: implement the same logic as JavaScript
        //      this.battle.faintQueue.push({
        //          target: this,
        //          source,
        //          effect,
        //      });
        damage
    }
}
