// NOTE: This method is NOT in JavaScript - Rust-specific implementation

use crate::battle_queue::Action;
use crate::battle_queue::BattleQueue;

impl BattleQueue {

    /// Get an iterator over the actions
    pub fn iter(&self) -> impl Iterator<Item = &Action> {
        self.list.iter()
    }
}
