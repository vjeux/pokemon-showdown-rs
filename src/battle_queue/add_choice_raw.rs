// NOTE: This method is NOT in JavaScript - Rust-specific implementation

use crate::battle_queue::Action;
use crate::battle_queue::BattleQueue;

impl BattleQueue {

    /// Add a pre-resolved action directly to the queue
    /// Equivalent to `this.queue.list.push(action)` in TypeScript
    /// This is a borrow-checker helper - use when you already have a resolved Action
    /// and don't need the resolution logic from addChoice()
    pub fn add_choice_raw(&mut self, action: Action) {
        if let Action::Move(ref move_action) = action {
            eprintln!("[QUEUE add_choice_raw] Adding Move {} from ({}, {}), queue size before: {}",
                move_action.move_id.as_str(), move_action.side_index, move_action.pokemon_index, self.list.len());
        }
        self.list.push(action);
    }
}
