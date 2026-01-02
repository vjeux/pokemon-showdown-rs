use crate::*;
use crate::event_system::EffectState;

impl Pokemon {

    /// Set status condition
    // TypeScript source:
    //
    //
    // 	setStatus(
    // 		status: string | Condition,
    // 		source: Pokemon | null = null,
    // 		sourceEffect: Effect | null = null,
    // 		ignoreImmunities = false
    // 	) {
    // 		if (!this.hp) return false;
    // 		status = this.battle.dex.conditions.get(status);
    // 		if (this.battle.event) {
    // 			if (!source) source = this.battle.event.source;
    // 			if (!sourceEffect) sourceEffect = this.battle.effect;
    // 		}
    // 		if (!source) source = this;
    //
    // 		if (this.status === status.id) {
    // 			if ((sourceEffect as Move)?.status === this.status) {
    // 				this.battle.add('-fail', this, this.status);
    // 			} else if ((sourceEffect as Move)?.status) {
    // 				this.battle.add('-fail', source);
    // 				this.battle.attrLastMove('[still]');
    // 			}
    // 			return false;
    // 		}
    //
    // 		if (
    // 			!ignoreImmunities && status.id && !(source?.hasAbility('corrosion') && ['tox', 'psn'].includes(status.id))
    // 		) {
    // 			// the game currently never ignores immunities
    // 			if (!this.runStatusImmunity(status.id === 'tox' ? 'psn' : status.id)) {
    // 				this.battle.debug('immune to status');
    // 				if ((sourceEffect as Move)?.status) {
    // 					this.battle.add('-immune', this);
    // 				}
    // 				return false;
    // 			}
    // 		}
    // 		const prevStatus = this.status;
    // 		const prevStatusState = this.statusState;
    // 		if (status.id) {
    // 			const result: boolean = this.battle.runEvent('SetStatus', this, source, sourceEffect, status);
    // 			if (!result) {
    // 				this.battle.debug('set status [' + status.id + '] interrupted');
    // 				return result;
    // 			}
    // 		}
    //
    // 		this.status = status.id;
    // 		this.statusState = this.battle.initEffectState({ id: status.id, target: this });
    // 		if (source) this.statusState.source = source;
    // 		if (status.duration) this.statusState.duration = status.duration;
    // 		if (status.durationCallback) {
    // 			this.statusState.duration = status.durationCallback.call(this.battle, this, source, sourceEffect);
    // 		}
    //
    // 		if (status.id && !this.battle.singleEvent('Start', status, this.statusState, this, source, sourceEffect)) {
    // 			this.battle.debug('status start [' + status.id + '] interrupted');
    // 			// cancel the setstatus
    // 			this.status = prevStatus;
    // 			this.statusState = prevStatusState;
    // 			return false;
    // 		}
    // 		if (status.id && !this.battle.runEvent('AfterSetStatus', this, source, sourceEffect, status)) {
    // 			return false;
    // 		}
    // 		return true;
    // 	}
    //
    pub fn set_status(&mut self, status: ID) -> bool {
        // JS: if (!this.hp) return false;
        // ✅ NOW IMPLEMENTED: HP check - returns false if fainted
        if self.hp == 0 {
            return false;
        }

        // JS: status = this.battle.dex.conditions.get(status);
        // Note: In Rust we receive ID directly, would need Battle reference to get full condition data

        // JS: if (this.battle.event) {
        // JS:     if (!source) source = this.battle.event.source;
        // JS:     if (!sourceEffect) sourceEffect = this.battle.effect;
        // JS: }
        // JS: if (!source) source = this;
        // Note: Missing source, sourceEffect, ignoreImmunities parameters

        // JS: if (this.status === status.id) {
        // JS:     if ((sourceEffect as Move)?.status === this.status) {
        // JS:         this.battle.add('-fail', this, this.status);
        // JS:     } else if ((sourceEffect as Move)?.status) {
        // JS:         this.battle.add('-fail', source);
        // JS:         this.battle.attrLastMove('[still]');
        // JS:     }
        // JS:     return false;
        // JS: }
        // Note: Has basic check but missing different failure messages
        if !self.status.is_empty() {
            return false;
        }

        // JS: if (!ignoreImmunities && status.id && ...) {
        // JS:     if (!this.runStatusImmunity(status.id === 'tox' ? 'psn' : status.id)) {
        // JS:         this.battle.debug('immune to status');
        // JS:         if ((sourceEffect as Move)?.status) {
        // JS:             this.battle.add('-immune', this);
        // JS:         }
        // JS:         return false;
        // JS:     }
        // JS: }
        // Note: Missing runStatusImmunity check and Corrosion ability exception

        // JS: const prevStatus = this.status;
        // JS: const prevStatusState = this.statusState;
        // Note: Not storing previous status for rollback

        // JS: if (status.id) {
        // JS:     const result = this.battle.runEvent('SetStatus', this, source, sourceEffect, status);
        // JS:     if (!result) {
        // JS:         this.battle.debug('set status [' + status.id + '] interrupted');
        // JS:         return result;
        // JS:     }
        // JS: }
        // Note: Missing runEvent('SetStatus')

        // JS: this.status = status.id;
        // JS: this.statusState = this.battle.initEffectState({ id: status.id, target: this });
        // JS: if (source) this.statusState.source = source;
        // JS: if (status.duration) this.statusState.duration = status.duration;
        // JS: if (status.durationCallback) {
        // JS:     this.statusState.duration = status.durationCallback.call(...);
        // JS: }
        self.status = status.clone();
        self.status_state = EffectState::new(status);
        self.status_state.target = Some((self.side_index, self.position));
        // Note: Missing source assignment, duration logic, durationCallback

        // JS: if (status.id && !this.battle.singleEvent('Start', status, this.statusState, this, source, sourceEffect)) {
        // JS:     this.battle.debug('status start [' + status.id + '] interrupted');
        // JS:     this.status = prevStatus;
        // JS:     this.statusState = prevStatusState;
        // JS:     return false;
        // JS: }
        // Note: Missing singleEvent('Start') and rollback logic

        // JS: if (status.id && !this.battle.runEvent('AfterSetStatus', this, source, sourceEffect, status)) {
        // JS:     return false;
        // JS: }
        // Note: Missing runEvent('AfterSetStatus')

        // JS: return true;
        true
    }
}
