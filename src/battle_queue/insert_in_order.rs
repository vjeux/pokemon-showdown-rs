use crate::*;
use crate::battle_queue::Action;
use crate::battle_queue::BattleQueue;

impl BattleQueue {

    /// Insert action maintaining sort order
    pub fn insert_in_order(&mut self, action: Action) {
        // Find the right position based on priority
        let mut insert_pos = self.list.len();

        for (i, existing) in self.list.iter().enumerate() {
            // Order: lower first
            let order_cmp = action.order().cmp(&existing.order());
            if order_cmp == std::cmp::Ordering::Less {
                insert_pos = i;
                break;
            } else if order_cmp == std::cmp::Ordering::Greater {
                continue;
            }

            // Priority: higher first
            if action.priority() > existing.priority() {
                insert_pos = i;
                break;
            } else if action.priority() < existing.priority() {
                continue;
            }

            // Fractional priority: higher first
            if action.fractional_priority() > existing.fractional_priority() {
                insert_pos = i;
                break;
            } else if action.fractional_priority() < existing.fractional_priority() {
                continue;
            }

            // Speed: higher first
            if action.speed() > existing.speed() {
                insert_pos = i;
                break;
            }
        }

        self.list.insert(insert_pos, action);
    }
}
