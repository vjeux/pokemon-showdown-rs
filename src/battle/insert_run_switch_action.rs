use crate::*;

impl Battle {

    /// Insert a runSwitch action into the queue with priority-based positioning
    /// Matches JavaScript's queue.insertChoice() behavior from battle-queue.js line 260
    //
    // JS: insertChoice(choices, midTurn = false) {
    //     const choice = choices;
    //     if (choice.pokemon) { choice.pokemon.updateSpeed(); }
    //     const actions = this.resolveAction(choice, midTurn);
    //     let firstIndex = null;
    //     let lastIndex = null;
    //     for (const [i, curAction] of this.list.entries()) {
    //       const compared = this.battle.comparePriority(actions[0], curAction);
    //       if (compared <= 0 && firstIndex === null) { firstIndex = i; }
    //       if (compared < 0) { lastIndex = i; break; }
    //     }
    //     if (firstIndex === null) { this.list.push(...actions); }
    //     else {
    //       if (lastIndex === null) lastIndex = this.list.length;
    //       const index = firstIndex === lastIndex ? firstIndex : this.battle.random(firstIndex, lastIndex + 1);
    //       this.list.splice(index, 0, ...actions);
    //     }
    //   }
    pub fn insert_run_switch_action(&mut self, side_index: usize, pokemon_index: usize) {
        use crate::battle_queue::{Action, PokemonAction, PokemonActionType};

        eprintln!("DEBUG [insert_run_switch_action]: Inserting runSwitch for side={}, pokemon={}, queue_len={}",
                  side_index, pokemon_index, self.queue.list.len());

        // Update Pokemon speed
        // JS: if (choice.pokemon) { choice.pokemon.updateSpeed(); }
        self.sides[side_index].pokemon[pokemon_index].update_speed();

        // Create the runSwitch action
        let action = Action::Pokemon(PokemonAction {
            choice: PokemonActionType::RunSwitch,
            order: 101,
            priority: 0,
            speed: self.sides[side_index].pokemon[pokemon_index].speed,
            pokemon_index,
            side_index,
            event: None,
        });

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
                eprintln!("DEBUG [insert_run_switch_action]: first==last={}, no PRNG needed", first);
                first
            } else {
                eprintln!("DEBUG [insert_run_switch_action]: first={}, last={}, calling random_with_range", first, last);
                self.random_with_range(first as i32, (last + 1) as i32) as usize
            };

            // JS: this.list.splice(index, 0, ...actions);
            self.queue.list.insert(index, action);
        }
    }
}
