use crate::*;
use crate::event_system::EffectState;

impl Pokemon {

    /// Set ability
    ///
    /// This is an associated function (not a method) because it needs
    /// access to Battle for singleEvent calls and gen checks.
    /// Call as: Pokemon::set_ability(battle, pokemon_pos, ability_id, ...)
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
    pub fn set_ability(
        battle: &mut Battle,
        pokemon_pos: (usize, usize),
        ability_id: ID,
        source_pos: Option<(usize, usize)>,
        source_effect: Option<&ID>,
        _is_from_forme_change: bool,
        is_transform: bool,
    ) -> ID {
        // Phase 1: Extract HP and old ability info
        let (hp, old_ability_id) = {
            let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => return ID::default(),
            };

            // JS: if (!this.hp) return false;
            // ✅ NOW IMPLEMENTED: HP check - returns empty ID (equivalent to false) if fainted
            (pokemon.hp, pokemon.ability.clone())
        };

        if hp == 0 {
            return ID::default();
        }

        // JS: if (typeof ability === 'string') ability = this.battle.dex.abilities.get(ability);
        // Note: In Rust we receive ID directly

        // JS: if (!sourceEffect && this.battle.effect) sourceEffect = this.battle.effect;
        // ✅ NOW IMPLEMENTED (Session 24 Part 29): source_pos and source_effect parameters
        // Note: battle.event source/sourceEffect defaulting still missing

        // JS: const oldAbility = this.battle.dex.abilities.get(this.ability);
        // Already have old_ability_id from Phase 1

        // JS: if (!isFromFormeChange) {
        // JS:     if (ability.flags['cantsuppress'] || this.getAbility().flags['cantsuppress']) return false;
        // JS: }
        // ✅ NOW IMPLEMENTED (Session 24 Part 91): cantsuppress flag check
        if !_is_from_forme_change {
            // Check if new ability has cantsuppress flag
            let new_ability_cantsuppress = battle.dex.abilities()
                .get_by_id(&ability_id)
                .and_then(|ability| ability.flags.get("cantsuppress"))
                .map(|v| *v != 0) // i32 flags: non-zero = true
                .unwrap_or(false);

            // Check if current ability has cantsuppress flag
            let old_ability_cantsuppress = battle.dex.abilities()
                .get_by_id(&old_ability_id)
                .and_then(|ability| ability.flags.get("cantsuppress"))
                .map(|v| *v != 0) // i32 flags: non-zero = true
                .unwrap_or(false);

            // If either ability has cantsuppress, return false (empty ID)
            if new_ability_cantsuppress || old_ability_cantsuppress {
                return ID::default();
            }
        }

        // JS: if (!isFromFormeChange && !isTransform) {
        // JS:     const setAbilityEvent = this.battle.runEvent('SetAbility', this, source, sourceEffect, ability);
        // JS:     if (!setAbilityEvent) return setAbilityEvent;
        // JS: }
        // ✅ NOW IMPLEMENTED (Session 24 Part 79): runEvent('SetAbility')
        // ✅ NOW IMPLEMENTED (Session 24 Part 91): isFromFormeChange and isTransform conditional checks
        // Note: JavaScript passes ability as 5th parameter (relayVar), but Rust run_event only accepts Option<i32>
        //       Passing None for now - handlers can check pokemon's ability field after it's set
        if !_is_from_forme_change && !is_transform {
            let set_ability_result = battle.run_event("SetAbility", Some(pokemon_pos), source_pos, source_effect, None);
            // runEvent returns Option<i32>, None or Some(0) means failure
            if set_ability_result == Some(0) || set_ability_result == None {
                return ID::default();
            }
        }

        // JS: this.battle.singleEvent('End', oldAbility, this.abilityState, this, source);
        // ✅ NOW IMPLEMENTED (Session 24 Part 76): singleEvent('End') for old ability
        if !old_ability_id.as_str().is_empty() {
            battle.single_event("End", &old_ability_id, Some(pokemon_pos), source_pos, None);
        }

        // Phase 2: Mutate pokemon to set new ability
        let pokemon_mut = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return ID::default(),
        };

        // JS: this.ability = ability.id;
        // JS: this.abilityState = this.battle.initEffectState({ id: ability.id, target: this });
        pokemon_mut.ability = ability_id.clone();
        pokemon_mut.ability_state = EffectState::new(ability_id.clone());
        pokemon_mut.ability_state.target = Some((pokemon_pos.0, pokemon_pos.1));
        // ✅ NOW IMPLEMENTED (Session 24 Part 29): source and source_effect assignments
        if let Some(src_pos) = source_pos {
            pokemon_mut.ability_state.source = Some(src_pos);
            pokemon_mut.ability_state.source_slot = Some(src_pos.1);
        }
        if let Some(src_effect) = source_effect {
            pokemon_mut.ability_state.source_effect = Some(src_effect.clone());
        }

        // JS: if (sourceEffect && !isFromFormeChange && !isTransform) {
        // JS:     if (source) {
        // JS:         this.battle.add('-ability', this, ability.name, oldAbility.name, `[from] ${sourceEffect.fullname}`, `[of] ${source}`);
        // JS:     } else {
        // JS:         this.battle.add('-ability', this, ability.name, oldAbility.name, `[from] ${sourceEffect.fullname}`);
        // JS:     }
        // JS: }
        // ✅ NOW IMPLEMENTED (Session 24 Part 87): battle.add message for ability changes
        // Note: Using IDs as approximation for ability names (fullname requires Dex access)
        if let Some(src_effect) = source_effect {
            if !_is_from_forme_change && !is_transform {
                let pokemon_ident = {
                    let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                        Some(p) => p,
                        None => return old_ability_id,
                    };
                    pokemon.get_slot()
                };

                if let Some(src_pos) = source_pos {
                    let source_ident = {
                        let source = match battle.pokemon_at(src_pos.0, src_pos.1) {
                            Some(p) => p,
                            None => return old_ability_id,
                        };
                        source.get_slot()
                    };

                    battle.add("-ability", &[
                        pokemon_ident.as_str().into(),
                        ability_id.as_str().into(),
                        old_ability_id.as_str().into(),
                        format!("[from] {}", src_effect.as_str()).into(),
                        format!("[of] {}", source_ident).into(),
                    ]);
                } else {
                    battle.add("-ability", &[
                        pokemon_ident.as_str().into(),
                        ability_id.as_str().into(),
                        old_ability_id.as_str().into(),
                        format!("[from] {}", src_effect.as_str()).into(),
                    ]);
                }
            }
        }

        // JS: if (ability.id && this.battle.gen > 3 && ...) {
        // JS:     this.battle.singleEvent('Start', ability, this.abilityState, this, source);
        // JS: }
        // ✅ NOW IMPLEMENTED (Session 24 Part 76): singleEvent('Start') for new ability with gen check
        if !ability_id.as_str().is_empty() && battle.gen > 3 {
            // JS: (!isTransform || oldAbility.id !== ability.id || this.battle.gen <= 4)
            if !is_transform || old_ability_id != ability_id || battle.gen <= 4 {
                battle.single_event("Start", &ability_id, Some(pokemon_pos), source_pos, source_effect);
            }
        }

        // JS: return oldAbility.id;
        old_ability_id
    }
}
