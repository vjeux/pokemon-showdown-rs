use crate::*;
use crate::battle_queue::Action;
use crate::battle_queue::BattleQueue;

impl BattleQueue {

    pub fn add_choice_raw(&mut self, action: Action) {
        self.list.push(action);
    }
}
