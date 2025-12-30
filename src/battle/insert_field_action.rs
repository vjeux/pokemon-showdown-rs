use crate::*;

impl Battle {

    /// Insert a Field action into the queue with priority-based positioning
    /// Matches JavaScript's queue.insertChoice() for Field actions
    pub fn insert_field_action(&mut self, action: crate::battle_queue::Action) {
        use crate::battle_queue::Action;

        eprintln!("DEBUG [insert_field_action]: Inserting {:?}, queue_len={}",
                  match &action {
                      Action::Field(f) => format!("Field::{:?}", f.choice),
                      _ => "Unknown".to_string(),
                  },
                  self.queue.list.len());

        // JS: let firstIndex = null; let lastIndex = null;
        let mut first_index: Option<usize> = None;
        let mut last_index: Option<usize> = None;

        // JS: for (const [i, curAction] of this.list.entries()) {
        for (i, cur_action) in self.queue.list.iter().enumerate() {
            // JS: const compared = this.battle.comparePriority(actions[0], curAction);
            let compared = self.compare_action_priority(&action, cur_action);

            // JS: if (compared <= 0 && firstIndex === null) { firstIndex = i; }
            if compared <= 0 && first_index.is_none() {
                first_index = Some(i);
            }

            // JS: if (compared < 0) { lastIndex = i; break; }
            if compared < 0 {
                last_index = Some(i);
                break;
            }
        }

        // JS: if (firstIndex === null) { this.list.push(...actions); }
        if first_index.is_none() {
            self.queue.list.push(action);
        } else {
            // JS: if (lastIndex === null) lastIndex = this.list.length;
            let first = first_index.unwrap();
            let last = last_index.unwrap_or(self.queue.list.len());

            // JS: const index = firstIndex === lastIndex ? firstIndex : this.battle.random(firstIndex, lastIndex + 1);
            let index = if first == last {
                eprintln!("DEBUG [insert_field_action]: first==last={}, no PRNG needed", first);
                first
            } else {
                eprintln!("DEBUG [insert_field_action]: first={}, last={}, calling random_with_range", first, last);
                self.random_with_range(first as i32, (last + 1) as i32) as usize
            };

            // JS: this.list.splice(index, 0, ...actions);
            self.queue.list.insert(index, action);
        }
    }
}
