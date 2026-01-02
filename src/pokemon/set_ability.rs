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
        // JS: if (!this.hp) return false;
        // ✅ NOW IMPLEMENTED: HP check - returns empty ID (equivalent to false) if fainted
        if self.hp == 0 {
            return ID::default();
        }

        // JS: if (typeof ability === 'string') ability = this.battle.dex.abilities.get(ability);
        // Note: In Rust we receive ID directly, would need Battle reference to get full ability data

        // JS: if (!sourceEffect && this.battle.effect) sourceEffect = this.battle.effect;
        // Note: Missing source and sourceEffect parameters

        // JS: const oldAbility = this.battle.dex.abilities.get(this.ability);
        let old = self.ability.clone();

        // JS: if (!isFromFormeChange) {
        // JS:     if (ability.flags['cantsuppress'] || this.getAbility().flags['cantsuppress']) return false;
        // JS: }
        // Note: Missing cantsuppress flag check - would need ability data access

        // JS: if (!isFromFormeChange && !isTransform) {
        // JS:     const setAbilityEvent = this.battle.runEvent('SetAbility', this, source, sourceEffect, ability);
        // JS:     if (!setAbilityEvent) return setAbilityEvent;
        // JS: }
        // Note: Missing runEvent('SetAbility') - would need Battle reference

        // JS: this.battle.singleEvent('End', oldAbility, this.abilityState, this, source);
        // Note: Missing singleEvent('End') for old ability - would need Battle reference

        // JS: this.ability = ability.id;
        // JS: this.abilityState = this.battle.initEffectState({ id: ability.id, target: this });
        self.ability = ability_id.clone();
        self.ability_state = EffectState::new(ability_id.clone());
        self.ability_state.target = Some((self.side_index, self.position));

        // JS: if (sourceEffect && !isFromFormeChange && !isTransform) {
        // JS:     this.battle.add('-ability', ...);
        // JS: }
        // Note: Missing battle.add message - would need Battle reference

        // JS: if (ability.id && this.battle.gen > 3 && ...) {
        // JS:     this.battle.singleEvent('Start', ability, this.abilityState, this, source);
        // JS: }
        // Note: Missing singleEvent('Start') for new ability - would need Battle reference and gen check

        // JS: return oldAbility.id;
        old
    }
}
