use crate::battle_queue::BattleQueue;

impl BattleQueue {

    /// Clear all actions
    //
    // 	clear() {
    // 		this.list = [];
    // 	}
    //
    pub fn clear(&mut self) {
        self.list.clear();
    }
}
