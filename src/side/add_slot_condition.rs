use crate::side::*;
use crate::*;

impl Side {

    /// Add a slot condition
    //
    // 	addSlotCondition(
    // 		target: Pokemon | number, status: string | Condition, source: Pokemon | 'debug' | null = null,
    // 		sourceEffect: Effect | null = null
    // 	) {
    // 		source ??= this.battle.event?.target || null;
    // 		if (source === 'debug') source = this.active[0];
    // 		if (target instanceof Pokemon) target = target.position;
    // 		if (!source) throw new Error(`setting sidecond without a source`);
    //
    // 		status = this.battle.dex.conditions.get(status);
    // 		if (this.slotConditions[target][status.id]) {
    // 			if (!status.onRestart) return false;
    // 			return this.battle.singleEvent('Restart', status, this.slotConditions[target][status.id], this, source, sourceEffect);
    // 		}
    // 		const conditionState = this.slotConditions[target][status.id] = this.battle.initEffectState({
    // 			id: status.id,
    // 			target: this,
    // 			source,
    // 			sourceSlot: source.getSlot(),
    // 			isSlotCondition: true,
    // 			duration: status.duration,
    // 		});
    // 		if (status.durationCallback) {
    // 			conditionState.duration =
    // 				status.durationCallback.call(this.battle, this.active[0], source, sourceEffect);
    // 		}
    // 		if (!this.battle.singleEvent('Start', status, conditionState, this.active[target], source, sourceEffect)) {
    // 			delete this.slotConditions[target][status.id];
    // 			return false;
    // 		}
    // 		return true;
    // 	}
    //
    pub fn add_slot_condition(&mut self, slot: usize, id: ID, duration: Option<i32>) -> bool {
        if slot >= self.slot_conditions.len() {
            return false;
        }
        if self.slot_conditions[slot].contains_key(&id) {
            return false;
        }
        let mut state = EffectState::new(id.clone());
        state.duration = duration;
        self.slot_conditions[slot].insert(id, state);
        true
    }
}
