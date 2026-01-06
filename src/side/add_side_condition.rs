// JS Source:
// 
// 	addSideCondition(
// 		status: string | Condition, source: Pokemon | 'debug' | null = null, sourceEffect: Effect | null = null
// 	): boolean {
// 		if (!source && this.battle.event?.target) source = this.battle.event.target;
// 		if (source === 'debug') source = this.active[0];
// 		if (!source) throw new Error(`setting sidecond without a source`);
// 		if (!source.getSlot) source = (source as any as Side).active[0];
// 
// 		status = this.battle.dex.conditions.get(status);
// 		if (this.sideConditions[status.id]) {
// 			if (!(status as any).onSideRestart) return false;
// 			return this.battle.singleEvent('SideRestart', status, this.sideConditions[status.id], this, source, sourceEffect);
// 		}
// 		this.sideConditions[status.id] = this.battle.initEffectState({
// 			id: status.id,
// 			target: this,
// 			source,
// 			sourceSlot: source.getSlot(),
// 			duration: status.duration,
// 		});
// 		if (status.durationCallback) {
// 			this.sideConditions[status.id].duration =
// 				status.durationCallback.call(this.battle, this.active[0], source, sourceEffect);
// 		}
// 		if (!this.battle.singleEvent('SideStart', status, this.sideConditions[status.id], this, source, sourceEffect)) {
// 			delete this.sideConditions[status.id];
// 			return false;
// 		}
// 		this.battle.runEvent('SideConditionStart', this, source, status);
// 		return true;
// 	}


use crate::side::*;
use crate::battle::Battle;
use crate::dex_data::ID;

impl Side {
    /// Add a side condition
    /// Equivalent to addSideCondition() in side.ts
    ///
    /// JavaScript (side.ts):
    ///   addSideCondition(
    ///     status: string | Condition, source: Pokemon | 'debug' | null = null,
    ///     sourceEffect: Effect | null = null
    ///   ): boolean { ... }
    ///
    /// Note: This implementation takes battle as a parameter for event handling.
    /// The JavaScript version accesses this.battle directly.
    pub fn add_side_condition_full(
        &mut self,
        battle: &mut Battle,
        status_id: ID,
        source_pos: Option<(usize, usize)>,
        source_effect: Option<ID>,
    ) -> bool {
        // JavaScript: if (!source && this.battle.event?.target) source = this.battle.event.target;
        // JavaScript: if (source === 'debug') source = this.active[0];
        // JavaScript: if (!source) throw new Error(`setting sidecond without a source`);
        // JavaScript: if (!source.getSlot) source = (source as any as Side).active[0];
        // Note: Simplified - we require source_pos to be provided

        // JavaScript: status = this.battle.dex.conditions.get(status);
        let status_duration = {
            if let Some(condition) = battle.dex.conditions().get_by_id(&status_id) {
                condition.duration
            } else {
                return false; // Condition not found
            }
        };

        // JavaScript: if (this.sideConditions[status.id]) { ... return onSideRestart ... }
        if self.side_conditions.contains_key(&status_id) {
            // if (!(status as any).onSideRestart) return false;
            // return this.battle.singleEvent('SideRestart', status, ...);
            if battle.has_callback(&status_id, "SideRestart") {
                let side_idx = self.n;
                let result = battle.single_event_side(
                    "SideRestart",
                    &status_id,
                    side_idx,
                    source_pos,
                    source_effect.as_ref(),
                );
                // Convert EventResult to bool
                return match result {
                    crate::event::EventResult::Boolean(b) => b,
                    crate::event::EventResult::Number(n) => n != 0,
                    _ => false,
                };
            }
            return false;
        }

        // JavaScript: this.sideConditions[status.id] = this.battle.initEffectState({ ... });
        let source_slot = source_pos.map(|(_, pokemon_index)| pokemon_index);

        let mut effect_state = crate::event_system::EffectState::new(status_id.clone());
        effect_state.duration = status_duration;
        effect_state.source = source_pos;
        effect_state.source_slot = source_slot;

        // JavaScript: if (status.durationCallback) { ... }
        // Call durationCallback if it exists
        let target_pos = if !self.active.is_empty() {
            Some((self.n, 0)) // self.active[0]
        } else {
            None
        };

        let result = battle.call_duration_callback(
            &status_id,
            target_pos,
            source_pos,
            source_effect.as_ref().map(|id| id.as_str()),
        );

        if let crate::event::EventResult::Number(duration) = result {
            effect_state.duration = Some(duration);
        }

        // Store side index before borrowing battle mutably
        let side_idx = self.n;

        // Store effect state before firing events
        self.side_conditions.insert(status_id.clone(), effect_state.clone());

        // JavaScript: if (!this.battle.singleEvent('SideStart', status, ...)) { delete; return false; }
        let side_start_result = battle.single_event_side(
            "SideStart",
            &status_id,
            side_idx,
            source_pos,
            source_effect.as_ref(),
        );

        // Check if SideStart event failed
        use crate::event::EventResult;
        if matches!(side_start_result, EventResult::Stop | EventResult::Null) {
            // delete this.sideConditions[status.id];
            self.side_conditions.remove(&status_id);
            return false;
        }

        // JavaScript: this.battle.runEvent('SideConditionStart', this, source, status);
        battle.run_event(
            "SideConditionStart",
            Some(crate::event::EventTarget::Side(side_idx)),
            source_pos,
            Some(&status_id),
            crate::event::EventResult::Continue,
            false,
            false,
        );

        true
    }

    /// Simplified add_side_condition for backward compatibility
    /// Use add_side_condition_full for full event system integration
    pub fn add_side_condition(&mut self, id: ID, duration: Option<i32>) -> bool {
        if self.side_conditions.contains_key(&id) {
            return false;
        }
        let mut state = EffectState::new(id.clone());
        state.duration = duration;
        self.side_conditions.insert(id, state);
        true
    }
}

