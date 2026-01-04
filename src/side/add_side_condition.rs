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
        _source_effect: Option<ID>,
    ) -> bool {
        // JavaScript: if (!source && this.battle.event?.target) source = this.battle.event.target;
        // JavaScript: if (source === 'debug') source = this.active[0];
        // JavaScript: if (!source) throw new Error(`setting sidecond without a source`);
        // JavaScript: if (!source.getSlot) source = (source as any as Side).active[0];
        // Note: Simplified - we require source_pos to be provided

        // JavaScript: status = this.battle.dex.conditions.get(status);
        let (status_duration, _has_duration_callback, _has_on_side_restart) = {
            if let Some(condition) = battle.dex.conditions().get_by_id(&status_id) {
                (
                    condition.duration,
                    false, // TODO: Check for durationCallback
                    false, // TODO: Check for onSideRestart
                )
            } else {
                return false; // Condition not found
            }
        };

        // JavaScript: if (this.sideConditions[status.id]) { ... return onSideRestart ... }
        if self.side_conditions.contains_key(&status_id) {
            // TODO: if (!(status as any).onSideRestart) return false;
            // TODO: return this.battle.singleEvent('SideRestart', status, ...);
            return false;
        }

        // JavaScript: this.sideConditions[status.id] = this.battle.initEffectState({ ... });
        let source_slot = source_pos.map(|(_, pokemon_index)| pokemon_index);

        let mut effect_state = crate::event_system::EffectState::new(status_id.clone());
        effect_state.duration = status_duration;
        effect_state.source = source_pos;
        effect_state.source_slot = source_slot;

        // JavaScript: if (status.durationCallback) { ... }
        // TODO: Implement durationCallback support when callback system is ready

        // Store effect state before firing events
        self.side_conditions.insert(status_id.clone(), effect_state.clone());

        // JavaScript: if (!this.battle.singleEvent('SideStart', status, ...)) { delete; return false; }
        // TODO: Implement singleEvent call when event system is ready
        // For now, we assume SideStart succeeds

        // JavaScript: this.battle.runEvent('SideConditionStart', this, source, status);
        // TODO: Implement runEvent call when event system is ready

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

