use crate::*;

impl Pokemon {

    /// Take damage
    pub fn take_damage(&mut self, damage: i32) -> i32 {
        let actual = damage.min(self.hp);
        self.hp = self.hp.saturating_sub(damage);
        if self.hp == 0 && !self.fainted {
            self.faint_queued = true;
        }
        self.hurt_this_turn = Some(self.hp);
        self.last_damage = actual;
        actual
    }
}
