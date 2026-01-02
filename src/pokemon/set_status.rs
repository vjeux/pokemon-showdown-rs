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
        // TODO: implement the same logic as JavaScript
        if !self.status.is_empty() {
            return false;
        }
        self.status = status.clone();
        self.status_state = EffectState::new(status);
        true
    }
}
