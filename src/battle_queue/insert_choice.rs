use crate::battle_queue::Action;
use crate::battle_queue::BattleQueue;

impl BattleQueue {

    /// Insert a choice at the front of the queue (for immediate execution)
    // TypeScript source:
    // /**
    // 	 * Inserts the passed action into the action queue when it normally
    // 	 * would have happened (sorting by priority/speed), without
    // 	 * re-sorting the existing actions.
    // 	 */
    // 	insertChoice(choices: ActionChoice | ActionChoice[], midTurn = false) {
    // 		if (Array.isArray(choices)) {
    // 			for (const choice of choices) {
    // 				this.insertChoice(choice);
    // 			}
    // 			return;
    // 		}
    // 		const choice = choices;
    //
    // 		if (choice.pokemon) {
    // 			choice.pokemon.updateSpeed();
    // 		}
    // 		const actions = this.resolveAction(choice, midTurn);
    //
    // 		let firstIndex = null;
    // 		let lastIndex = null;
    // 		for (const [i, curAction] of this.list.entries()) {
    // 			const compared = this.battle.comparePriority(actions[0], curAction);
    // 			if (compared <= 0 && firstIndex === null) {
    // 				firstIndex = i;
    // 			}
    // 			if (compared < 0) {
    // 				lastIndex = i;
    // 				break;
    // 			}
    // 		}
    //
    // 		if (firstIndex === null) {
    // 			this.list.push(...actions);
    // 		} else {
    // 			if (lastIndex === null) lastIndex = this.list.length;
    // 			const index = firstIndex === lastIndex ? firstIndex : this.battle.random(firstIndex, lastIndex + 1);
    // 			this.list.splice(index, 0, ...actions);
    // 		}
    // 	}
    //
    pub fn insert_choice(&mut self, action: Action) {
        self.list.insert(0, action);
    }
}
