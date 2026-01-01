use crate::battle_queue::Action;
use crate::battle_queue::BattleQueue;

impl BattleQueue {

    /// Get the next action (removes from front)
    // TypeScript source:
    //
    //
    // 	shift() {
    // 		return this.list.shift();
    // 	}
    //
    pub fn shift(&mut self) -> Option<Action> {
        if self.list.is_empty() {
            None
        } else {
            let action = self.list.remove(0);
            eprintln!("[QUEUE.SHIFT] Removed action: priority={}, speed={}, order={:?}",
                action.priority(), action.speed(), action.order());
            eprintln!("[QUEUE.SHIFT] Queue now has {} actions remaining", self.list.len());
            Some(action)
        }
    }
}
