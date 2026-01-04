use crate::battle_queue::{Action, BattleQueue};

impl BattleQueue {

    /// Prioritize an action (move it to the front)
    // TypeScript source:
    // /**
    // 	 * Makes the passed action happen next (skipping speed order).
    // 	 */
    // 	prioritizeAction(action: MoveAction | SwitchAction, sourceEffect?: Effect) {
    // 		for (const [i, curAction] of this.list.entries()) {
    // 			if (curAction === action) {
    // 				this.list.splice(i, 1);
    // 				break;
    // 			}
    // 		}
    // 		action.sourceEffect = sourceEffect;
    // 		action.order = 3;
    // 		this.list.unshift(action);
    // 	}
    //
    pub fn prioritize_action(&mut self, side_index: usize, pokemon_index: usize) -> bool {
        let pos = self.list.iter().position(|action| {
            action.side_index() == Some(side_index) && action.pokemon_index() == Some(pokemon_index)
        });
        if let Some(i) = pos {
            let mut action = self.list.remove(i);
            // JS: action.order = 3;
            match &mut action {
                crate::battle_queue::Action::Move(m) => m.order = 3,
                crate::battle_queue::Action::Switch(s) => s.order = 3,
                crate::battle_queue::Action::Pokemon(p) => p.order = 3,
                _ => {}
            }
            self.list.insert(0, action);
            true
        } else {
            false
        }
    }

    /// Prioritize a specific action object (add it to the front of the queue)
    /// This variant takes an action object directly instead of looking it up by pokemon
    /// Equivalent to: prioritizeAction(action: MoveAction | SwitchAction, sourceEffect?: Effect)
    pub fn prioritize_action_object(&mut self, mut action: Action) {
        // action.order = 3;
        match &mut action {
            crate::battle_queue::Action::Move(m) => m.order = 3,
            crate::battle_queue::Action::Switch(s) => s.order = 3,
            crate::battle_queue::Action::Pokemon(p) => p.order = 3,
            _ => {}
        }
        // this.list.unshift(action);
        self.list.insert(0, action);
    }
}
