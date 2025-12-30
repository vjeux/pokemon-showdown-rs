use crate::*;
use crate::event_system::EffectState;

impl Pokemon {

    /// Set ability
    //
    // 	setAbility(
    // 		ability: string | Ability, source?: Pokemon | null, sourceEffect?: Effect | null,
    // 		isFromFormeChange = false, isTransform = false,
    // 	) {
    // 		if (!this.hp) return false;
    // 		if (typeof ability === 'string') ability = this.battle.dex.abilities.get(ability);
    // 		if (!sourceEffect && this.battle.effect) sourceEffect = this.battle.effect;
    // 		const oldAbility = this.battle.dex.abilities.get(this.ability);
    // 		if (!isFromFormeChange) {
    // 			if (ability.flags['cantsuppress'] || this.getAbility().flags['cantsuppress']) return false;
    // 		}
    // 		if (!isFromFormeChange && !isTransform) {
    // 			const setAbilityEvent: boolean | null = this.battle.runEvent('SetAbility', this, source, sourceEffect, ability);
    // 			if (!setAbilityEvent) return setAbilityEvent;
    // 		}
    // 		this.battle.singleEvent('End', oldAbility, this.abilityState, this, source);
    // 		this.ability = ability.id;
    // 		this.abilityState = this.battle.initEffectState({ id: ability.id, target: this });
    // 		if (sourceEffect && !isFromFormeChange && !isTransform) {
    // 			if (source) {
    // 				this.battle.add('-ability', this, ability.name, oldAbility.name, `[from] ${sourceEffect.fullname}`, `[of] ${source}`);
    // 			} else {
    // 				this.battle.add('-ability', this, ability.name, oldAbility.name, `[from] ${sourceEffect.fullname}`);
    // 			}
    // 		}
    // 		if (ability.id && this.battle.gen > 3 &&
    // 			(!isTransform || oldAbility.id !== ability.id || this.battle.gen <= 4)) {
    // 			this.battle.singleEvent('Start', ability, this.abilityState, this, source);
    // 		}
    // 		return oldAbility.id;
    // 	}
    //
    pub fn set_ability(&mut self, ability_id: ID) -> ID {
        let old = self.ability.clone();
        self.ability = ability_id.clone();
        self.ability_state = EffectState::new(ability_id);
        old
    }
}
