use crate::battle_queue::{BattleQueue, Action};
use crate::battle::Battle;

impl BattleQueue {

    /// Insert a choice into the queue with priority-based positioning and PRNG tie-breaking
    /// Equivalent to TypeScript's queue.insertChoice() (lines 370-402 in battle-queue.ts)
    ///
    /// TypeScript source:
    /// ```typescript
    /// insertChoice(choices: ActionChoice | ActionChoice[], midTurn = false) {
    ///     if (Array.isArray(choices)) {
    ///         for (const choice of choices) {
    ///             this.insertChoice(choice);
    ///         }
    ///         return;
    ///     }
    ///     const choice = choices;
    ///
    ///     if (choice.pokemon) {
    ///         choice.pokemon.updateSpeed();
    ///     }
    ///     const actions = this.resolveAction(choice, midTurn);
    ///
    ///     let firstIndex = null;
    ///     let lastIndex = null;
    ///     for (const [i, curAction] of this.list.entries()) {
    ///         const compared = this.battle.comparePriority(actions[0], curAction);
    ///         if (compared <= 0 && firstIndex === null) {
    ///             firstIndex = i;
    ///         }
    ///         if (compared < 0) {
    ///             lastIndex = i;
    ///             break;
    ///         }
    ///     }
    ///
    ///     if (firstIndex === null) {
    ///         this.list.push(...actions);
    ///     } else {
    ///         if (lastIndex === null) lastIndex = this.list.length;
    ///         const index = firstIndex === lastIndex ? firstIndex : this.battle.random(firstIndex, lastIndex + 1);
    ///         this.list.splice(index, 0, ...actions);
    ///     }
    /// }
    /// ```
    ///
    /// In Rust, we split this into two parts:
    /// 1. Caller resolves the action (converts from ActionChoice to Action)
    /// 2. This method handles the insertion with PRNG tie-breaking
    ///
    /// This matches the JavaScript logic 1-to-1 but adapts to Rust's ownership model
    /// by taking Battle as a parameter instead of storing a reference.
    pub fn insert_choice(&mut self, battle: &mut Battle, action: Action) {
        debug_elog!("[QUEUE.INSERT_CHOICE] Queue BEFORE insert:");
        for (i, act) in self.list.iter().enumerate() {
            debug_elog!("  Index {}: priority={}, speed={}, order={:?}",
                i, act.priority(), act.speed(), act.order());
        }

        // JS: let firstIndex = null; let lastIndex = null;
        let mut first_index: Option<usize> = None;
        let mut last_index: Option<usize> = None;

        // JS: for (const [i, curAction] of this.list.entries()) {
        for (i, cur_action) in self.list.iter().enumerate() {
            // JS: const compared = this.battle.comparePriority(actions[0], curAction);
            let compared = Battle::compare_action_priority(&action, cur_action);

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
            self.list.push(action);
        } else {
            // JS: if (lastIndex === null) lastIndex = this.list.length;
            let first = first_index.unwrap();
            let last = last_index.unwrap_or(self.list.len());

            // JS: const index = firstIndex === lastIndex ? firstIndex : this.battle.random(firstIndex, lastIndex + 1);
            let index = if first == last {
                first
            } else {
                battle.random_with_range(first as i32, (last + 1) as i32) as usize
            };

            debug_elog!("[QUEUE.INSERT_CHOICE] Inserting at index {} (first={}, last={})", index, first, last);
            // JS: this.list.splice(index, 0, ...actions);
            self.list.insert(index, action);
        }

        debug_elog!("[QUEUE.INSERT_CHOICE] Queue AFTER insert:");
        for (i, act) in self.list.iter().enumerate() {
            debug_elog!("  Index {}: priority={}, speed={}, order={:?}",
                i, act.priority(), act.speed(), act.order());
        }
    }
}
