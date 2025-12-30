use crate::battle_queue::Action;
use crate::battle_queue::BattleQueue;

impl BattleQueue {

    /// Add one or more action choices and resolve them
    //
    // 	addChoice(choices: ActionChoice | ActionChoice[]) {
    // 		if (!Array.isArray(choices)) choices = [choices];
    // 		for (const choice of choices) {
    // 			const resolvedChoices = this.resolveAction(choice);
    // 			this.list.push(...resolvedChoices);
    // 			for (const resolvedChoice of resolvedChoices) {
    // 				if (resolvedChoice && resolvedChoice.choice === 'move' && resolvedChoice.move.id !== 'recharge') {
    // 					resolvedChoice.pokemon.side.lastSelectedMove = resolvedChoice.move.id;
    // 				}
    // 			}
    // 		}
    // 	}
    //
    // BattleQueue.prototype.addChoice (lines 302-313 in battle-queue.ts)
    // 	addChoice(choices: ActionChoice | ActionChoice[]) {
    // 		if (!Array.isArray(choices)) choices = [choices];
    // 		for (const choice of choices) {
    // 			const resolvedChoices = this.resolveAction(choice);
    // 			this.list.push(...resolvedChoices);
    // 			for (const resolvedChoice of resolvedChoices) {
    // 				if (resolvedChoice && resolvedChoice.choice === 'move' && resolvedChoice.move.id !== 'recharge') {
    // 					resolvedChoice.pokemon.side.lastSelectedMove = resolvedChoice.move.id;
    // 				}
    // 			}
    // 		}
    // 	}
    pub fn add_choice(&mut self, battle: &mut crate::battle::Battle, side_action: &crate::side::ChosenAction, side_idx: usize) {
        // JS: const resolvedChoices = this.resolveAction(choice);
        let resolved_actions = battle.resolve_action(side_action, side_idx);

        // JS: this.list.push(...resolvedChoices);
        for action in resolved_actions {
            self.list.push(action.clone());

            // JS: if (resolvedChoice && resolvedChoice.choice === 'move' && resolvedChoice.move.id !== 'recharge') {
            // JS:     resolvedChoice.pokemon.side.lastSelectedMove = resolvedChoice.move.id;
            // JS: }
            if let Action::Move(ref move_action) = action {
                if move_action.move_id.as_str() != "recharge" {
                    battle.sides[side_idx].last_selected_move = move_action.move_id.clone();
                }
            }
        }
    }
}
