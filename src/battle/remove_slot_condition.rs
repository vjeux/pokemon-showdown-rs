///! Battle remove_slot_condition method
///!
///! JavaScript source (side.ts removeSlotCondition):
// 	removeSlotCondition(target: Pokemon | number, status: string | Effect) {
// 		if (target instanceof Pokemon) target = target.position;
// 		status = this.battle.dex.conditions.get(status) as Effect;
// 		if (!this.slotConditions[target][status.id]) return false;
// 		this.battle.singleEvent('End', status, this.slotConditions[target][status.id], this.active[target]);
// 		delete this.slotConditions[target][status.id];
// 		return true;
// 	}

use crate::battle::Battle;
use crate::dex_data::ID;

impl Battle {
    /// Remove a slot condition from a side, firing the End event
    ///
    /// This is a wrapper around Side::remove_slot_condition that also fires
    /// the singleEvent('End', ...) callback, matching JavaScript behavior.
    pub fn remove_slot_condition(&mut self, side_idx: usize, slot: usize, id: &ID) -> bool {
        // if (!this.slotConditions[target][status.id]) return false;
        let has_condition = self.sides.get(side_idx)
            .and_then(|side| side.slot_conditions.get(slot))
            .map(|conds| conds.contains_key(id))
            .unwrap_or(false);

        if !has_condition {
            return false;
        }

        // Get the active Pokemon in this slot for the End event target
        // JavaScript: this.active[target] - gets the actual Pokemon, we need its party index
        // self.sides[side_idx].active[slot] contains the party index of the active Pokemon at that slot
        let target_pos = self.sides.get(side_idx)
            .and_then(|side| side.active.get(slot))
            .and_then(|&active| active)
            .map(|party_idx| (side_idx, party_idx));  // Use party index, not active slot

        // Clone the state so on_end can access hp, source, etc.
        let state_owned = self.sides.get(side_idx)
            .and_then(|side| side.slot_conditions.get(slot))
            .and_then(|conds| conds.get(id))
            .cloned();

        // this.battle.singleEvent('End', status, this.slotConditions[target][status.id], this.active[target]);
        // IMPORTANT: Create effect with effect_holder set to (side_idx, slot) so with_effect_state_ref
        // can look up slot_conditions[slot] correctly. The target is the party index (for the Pokemon),
        // but effect_holder must be the active slot position.
        let mut effect = crate::battle::Effect::slot_condition(id.clone());
        effect.effect_holder = Some((side_idx, slot));
        effect.side_index = Some(side_idx);

        self.single_event(
            "End",
            &effect,
            state_owned.as_ref(),  // Pass the state so on_end can access hp
            target_pos,
            None,
            None,
            None,
        );

        // delete this.slotConditions[target][status.id];
        if let Some(side) = self.sides.get_mut(side_idx) {
            if let Some(conds) = side.slot_conditions.get_mut(slot) {
                conds.remove(id);
            }
        }

        true
    }
}
