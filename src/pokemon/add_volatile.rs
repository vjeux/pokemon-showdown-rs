use crate::*;
use crate::event_system::EffectState;

impl Pokemon {

    /// Add a volatile condition
    //
    // 	addVolatile(
    // 		status: string | Condition, source: Pokemon | null = null, sourceEffect: Effect | null = null,
    // 		linkedStatus: string | Condition | null = null
    // 	): boolean | any {
    // 		let result;
    // 		status = this.battle.dex.conditions.get(status);
    // 		if (!this.hp && !status.affectsFainted) return false;
    // 		if (linkedStatus && source && !source.hp) return false;
    // 		if (this.battle.event) {
    // 			if (!source) source = this.battle.event.source;
    // 			if (!sourceEffect) sourceEffect = this.battle.effect;
    // 		}
    // 		if (!source) source = this;
    //
    // 		if (this.volatiles[status.id]) {
    // 			if (!status.onRestart) return false;
    // 			return this.battle.singleEvent('Restart', status, this.volatiles[status.id], this, source, sourceEffect);
    // 		}
    // 		if (!this.runStatusImmunity(status.id)) {
    // 			this.battle.debug('immune to volatile status');
    // 			if ((sourceEffect as Move)?.status) {
    // 				this.battle.add('-immune', this);
    // 			}
    // 			return false;
    // 		}
    // 		result = this.battle.runEvent('TryAddVolatile', this, source, sourceEffect, status);
    // 		if (!result) {
    // 			this.battle.debug('add volatile [' + status.id + '] interrupted');
    // 			return result;
    // 		}
    // 		this.volatiles[status.id] = this.battle.initEffectState({ id: status.id, name: status.name, target: this });
    // 		if (source) {
    // 			this.volatiles[status.id].source = source;
    // 			this.volatiles[status.id].sourceSlot = source.getSlot();
    // 		}
    // 		if (sourceEffect) this.volatiles[status.id].sourceEffect = sourceEffect;
    // 		if (status.duration) this.volatiles[status.id].duration = status.duration;
    // 		if (status.durationCallback) {
    // 			this.volatiles[status.id].duration = status.durationCallback.call(this.battle, this, source, sourceEffect);
    // 		}
    // 		result = this.battle.singleEvent('Start', status, this.volatiles[status.id], this, source, sourceEffect);
    // 		if (!result) {
    // 			// cancel
    // 			delete this.volatiles[status.id];
    // 			return result;
    // 		}
    // 		if (linkedStatus && source) {
    // 			if (!source.volatiles[linkedStatus.toString()]) {
    // 				source.addVolatile(linkedStatus, this, sourceEffect);
    // 				source.volatiles[linkedStatus.toString()].linkedPokemon = [this];
    // 				source.volatiles[linkedStatus.toString()].linkedStatus = status;
    // 			} else {
    // 				source.volatiles[linkedStatus.toString()].linkedPokemon.push(this);
    // 			}
    // 			this.volatiles[status.toString()].linkedPokemon = [source];
    // 			this.volatiles[status.toString()].linkedStatus = linkedStatus;
    // 		}
    // 		return true;
    // 	}
    //
    pub fn add_volatile(&mut self, id: ID) -> bool {
        if self.volatiles.contains_key(&id) {
            return false;
        }
        self.volatiles.insert(id.clone(), EffectState::new(id));
        true
    }
}
