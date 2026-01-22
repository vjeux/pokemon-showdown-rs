///! Battle add_slot_condition method
///!
///! JavaScript source (side.ts addSlotCondition):
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

use crate::battle::{Battle, Effect};
use crate::dex_data::ID;
use crate::event::EventResult;
use crate::event_system::EffectState;

impl Battle {
    /// Add a slot condition to a side, firing the Start event
    ///
    /// This is a wrapper that initializes the effect state with source info
    /// and fires the singleEvent('Start', ...) callback, matching JavaScript behavior.
    pub fn add_slot_condition(
        &mut self,
        target_pos: (usize, usize),
        status_id: ID,
        source_pos: Option<(usize, usize)>,
        source_effect: Option<&Effect>,
        duration: Option<i32>,
    ) -> bool {
        // JavaScript: source ??= this.battle.event?.target || null;
        let source_pos = source_pos.or_else(|| self.event.as_ref().and_then(|e| e.target));

        let side_idx = target_pos.0;

        // JavaScript: if (target instanceof Pokemon) target = target.position;
        // We need to use the Pokemon's active position, not their party index
        // target_pos.1 is the party index, but slot_conditions are indexed by active position
        let slot = {
            let pokemon = match self.pokemon_at(target_pos.0, target_pos.1) {
                Some(p) => p,
                None => return false,
            };
            pokemon.position
        };

        // Check if slot is valid
        let slot_valid = self.sides.get(side_idx)
            .map(|side| slot < side.slot_conditions.len())
            .unwrap_or(false);

        if !slot_valid {
            return false;
        }

        // if (this.slotConditions[target][status.id]) {
        //     if (!status.onRestart) return false;
        //     return this.battle.singleEvent('Restart', ...);
        // }
        let already_exists = self.sides.get(side_idx)
            .and_then(|side| side.slot_conditions.get(slot))
            .map(|conds| conds.contains_key(&status_id))
            .unwrap_or(false);

        if already_exists {
            // TODO: Handle onRestart
            return false;
        }

        // Get source slot for effect state (active position, not party index)
        // JavaScript: sourceSlot: source.getSlot() - this returns the active position
        let source_slot = source_pos.and_then(|pos| {
            self.pokemon_at(pos.0, pos.1).map(|p| p.position)
        });

        // const conditionState = this.slotConditions[target][status.id] = this.battle.initEffectState({...});
        let mut state = EffectState::new(status_id.clone());
        state.duration = duration;
        state.source = source_pos;
        state.source_slot = source_slot;
        state.target = Some(target_pos);
        // Set unique effect_order for tie-breaking when sorting handlers
        // JavaScript: initEffectState increments effectOrder counter
        state.effect_order = self.effect_order;
        self.effect_order += 1;

        // Insert the condition
        if let Some(side) = self.sides.get_mut(side_idx) {
            if let Some(conds) = side.slot_conditions.get_mut(slot) {
                conds.insert(status_id.clone(), state);
            }
        }

        // Get the active Pokemon in this slot for the Start event target
        let active_target = self.sides.get(side_idx)
            .and_then(|side| side.active.get(slot))
            .and_then(|&active| active)
            .map(|_| target_pos);

        // if (!this.battle.singleEvent('Start', status, conditionState, this.active[target], source, sourceEffect)) {
        let start_result = self.single_event(
            "Start",
            &Effect::slot_condition(status_id.clone()),
            None,
            active_target,
            source_pos,
            source_effect,
            None,
        );

        // If Start returned false, remove the condition
        if matches!(start_result, EventResult::Boolean(false)) {
            if let Some(side) = self.sides.get_mut(side_idx) {
                if let Some(conds) = side.slot_conditions.get_mut(slot) {
                    conds.remove(&status_id);
                }
            }
            return false;
        }

        true
    }
}
