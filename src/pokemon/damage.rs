use crate::*;
use crate::battle::{Effect, FaintData};

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
    pub fn damage(
        &mut self,
        mut amount: i32,
        pokemon_pos: (usize, usize),
        source: Option<(usize, usize)>,
        effect: Option<&Effect>,
        faint_queue: &mut Vec<FaintData>,
    ) -> i32 {
        // JS: if (!this.hp || isNaN(d) || d <= 0) return 0;
        if self.hp == 0 || amount <= 0 {
            return 0;
        }

        // JS: if (d < 1 && d > 0) d = 1;
        if amount > 0 && amount < 1 {
            amount = 1;
        }

        // JS: this.hp -= d;
        self.hp -= amount;

        // JS: if (this.hp <= 0) {
        //         d += this.hp;  // Adjust damage down if HP went negative
        //         this.faint(source, effect);
        //     }
        if self.hp <= 0 {
            amount += self.hp; // self.hp is negative, so this subtracts from amount
            self.hp = 0;

            // JS: this.faint(source, effect);
            // Check if already queued or fainted
            if self.fainted || self.faint_queued {
                // Already queued/fainted, don't add again
                return amount;
            }

            debug_elog!("[DAMAGE] Pokemon {} fainted! Adding to faint_queue. pokemon_pos={:?}, source={:?}",
                self.name, pokemon_pos, source);

            // JS equivalent: this.faint(source, effect) -> sets faintQueued and adds to faintQueue
            // Note: switch_flag clearing happens in faint() but not relevant here since we're in damage()
            self.faint_queued = true;

            faint_queue.push(FaintData {
                target: pokemon_pos,
                source,
                effect: effect.cloned(),
            });

            debug_elog!("[DAMAGE] faint_queue now has {} entries", faint_queue.len());
        }

        // JS: return d;
        amount
    }
}
