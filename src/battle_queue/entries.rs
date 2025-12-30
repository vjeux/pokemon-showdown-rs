use crate::battle_queue::Action;
use crate::battle_queue::BattleQueue;

impl BattleQueue {

    /// Get entries as iterator with indices
    // 	entries() {
    // 		return this.list.entries();
    // 	}
    //
    pub fn entries(&self) -> impl Iterator<Item = (usize, &Action)> {
        self.list.iter().enumerate()
    }
}
