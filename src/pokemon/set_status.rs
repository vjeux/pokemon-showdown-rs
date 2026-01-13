use crate::*;
use crate::event::EventResult;
use crate::event_system::EffectState;
use crate::battle::{Effect, EffectType};

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
    pub fn set_status(
        battle: &mut Battle,
        pokemon_pos: (usize, usize),
        status: ID,
        source_pos: Option<(usize, usize)>,
        source_effect: Option<&Effect>,
        _ignore_immunities: bool,
    ) -> bool {
        // Phase 1: Extract HP and previous status info
        let (hp, prev_status, prev_status_state) = {
            let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => return false,
            };
            (pokemon.hp, pokemon.status.clone(), pokemon.status_state.clone())
        };

        // JS: if (!this.hp) return false;
        // ✅ NOW IMPLEMENTED: HP check - returns false if fainted
        if hp == 0 {
            return false;
        }

        // JS: status = this.battle.dex.conditions.get(status);
        // Note: In Rust we receive ID directly, would need Battle reference to get full condition data

        // JS: if (this.battle.event) {
        // JS:     if (!source) source = this.battle.event.source;
        // JS:     if (!sourceEffect) sourceEffect = this.battle.effect;
        // JS: }
        // JS: if (!source) source = this;
        // ✅ NOW IMPLEMENTED: source_pos defaults from event.source or to target pokemon
        let source_pos = if source_pos.is_some() {
            source_pos
        } else if let Some(ref event) = battle.event {
            if event.source.is_some() {
                event.source
            } else {
                Some(pokemon_pos)
            }
        } else {
            Some(pokemon_pos)
        };

        // JS: if (!sourceEffect) sourceEffect = this.battle.effect;
        // ✅ NOW IMPLEMENTED: source_effect defaults from battle.effect
        let source_effect_owned: Option<crate::battle::Effect>;
        let source_effect = if source_effect.is_some() {
            source_effect
        } else {
            source_effect_owned = battle.effect.clone();
            source_effect_owned.as_ref()
        };

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
        // FIXED: Changed from !prev_status.is_empty() to match JavaScript logic
        // JavaScript only prevents setting the SAME status, not ANY status change
        if prev_status == status {
            return false;
        }

        // JS: if (!ignoreImmunities && status.id && !(source?.hasAbility('corrosion') && ['tox', 'psn'].includes(status.id))) {
        // JS:     if (!this.runStatusImmunity(status.id === 'tox' ? 'psn' : status.id)) {
        // JS:         this.battle.debug('immune to status');
        // JS:         if ((sourceEffect as Move)?.status) {
        // JS:             this.battle.add('-immune', this);
        // JS:         }
        // JS:         return false;
        // JS:     }
        // JS: }
        // ✅ NOW IMPLEMENTED: runStatusImmunity check with Corrosion ability exception
        if !_ignore_immunities && !status.as_str().is_empty() {
            // Check if source has Corrosion ability (bypasses poison immunity)
            let source_has_corrosion = if let Some(src_pos) = source_pos {
                if let Some(src_pokemon) = battle.pokemon_at(src_pos.0, src_pos.1) {
                    src_pokemon.has_ability(battle, &["corrosion"])
                } else {
                    false
                }
            } else {
                false
            };

            let status_str = status.as_str();
            let is_poison_status = status_str == "tox" || status_str == "psn";

            // Corrosion only bypasses immunity for poison statuses
            if !(source_has_corrosion && is_poison_status) {
                // For 'tox', check immunity against 'psn'
                let immunity_check_status = if status_str == "tox" { "psn" } else { status_str };

                // Check if sourceEffect is a move with a status field (for message display)
                let source_effect_has_status = source_effect.map_or(false, |e| {
                    if e.effect_type == EffectType::Move {
                        if let Some(move_data) = battle.dex.moves.get(&e.id) {
                            move_data.status.is_some()
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                });

                if !Pokemon::run_status_immunity(battle, pokemon_pos, immunity_check_status, source_effect_has_status) {
                    battle.debug("immune to status");
                    return false;
                }
            }
        }

        // JS: const prevStatus = this.status;
        // JS: const prevStatusState = this.statusState;
        // ✅ NOW IMPLEMENTED (Session 24 Part 77): Store previous status for rollback

        // JS: if (status.id) {
        // JS:     const result = this.battle.runEvent('SetStatus', this, source, sourceEffect, status);
        // JS:     if (!result) {
        // JS:         this.battle.debug('set status [' + status.id + '] interrupted');
        // JS:         return result;
        // JS:     }
        // JS: }
        // ✅ NOW IMPLEMENTED (Session 24 Part 78): runEvent('SetStatus')
        // JavaScript passes status as 5th parameter (relayVar) for handlers like Sweet Veil to check
        if !status.as_str().is_empty() {
            let set_status_result = battle.run_event("SetStatus", Some(crate::event::EventTarget::Pokemon(pokemon_pos)), source_pos, source_effect, EventResult::String(status.to_string()), false, false);
            // Check if event returned a falsy value (Number(0), Boolean(false), or Null)
            let is_blocked = matches!(set_status_result,
                EventResult::Number(0) | EventResult::Boolean(false) | EventResult::Null
            );
            if is_blocked {
                return false;
            }
        }

        // Phase 2: Get source pokemon's active slot position before mutable borrow
        // JavaScript: this.statusState.sourceSlot = source.getSlot();
        // getSlot() uses this.position (the active slot index: 0, 1, 2...)
        let source_position = source_pos.and_then(|src_pos| {
            battle.pokemon_at(src_pos.0, src_pos.1).map(|p| p.position)
        });

        // Now mutate pokemon to set new status
        let pokemon_mut = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return false,
        };

        // JS: this.status = status.id;
        // JS: this.statusState = this.battle.initEffectState({ id: status.id, target: this });
        // JS: if (source) this.statusState.source = source;
        // JS: if (status.duration) this.statusState.duration = status.duration;
        // JS: if (status.durationCallback) {
        // JS:     this.statusState.duration = status.durationCallback.call(...);
        // JS: }
        pokemon_mut.status = status.clone();
        pokemon_mut.status_state = EffectState::new(status.clone());
        pokemon_mut.status_state.target = Some((pokemon_pos.0, pokemon_pos.1));
        // ✅ NOW IMPLEMENTED (Session 24 Part 28): source and source_effect assignments
        // source_slot should be the active slot position (pokemon.position), not party index
        if let Some(src_pos) = source_pos {
            pokemon_mut.status_state.source = Some(src_pos);
            pokemon_mut.status_state.source_slot = source_position;
        }
        if let Some(src_effect) = source_effect {
            pokemon_mut.status_state.source_effect = Some(src_effect.clone());
        }
        // Note: Missing duration and durationCallback logic (needs Battle/dex access)

        // JS: if (status.id && !this.battle.singleEvent('Start', status, this.statusState, this, source, sourceEffect)) {
        // JS:     this.battle.debug('status start [' + status.id + '] interrupted');
        // JS:     this.status = prevStatus;
        // JS:     this.statusState = prevStatusState;
        // JS:     return false;
        // JS: }
        // ✅ NOW IMPLEMENTED (Session 24 Part 77): singleEvent('Start') with rollback logic
        if !status.as_str().is_empty() {
            let start_result = battle.single_event("Start", &crate::battle::Effect::status(status.clone()), None, Some(pokemon_pos), source_pos, source_effect, None);
            // Check if event failed (returned false or null)
            let event_failed = matches!(start_result, EventResult::Boolean(false) | EventResult::Null);
            if event_failed {
                // Rollback status change
                let pokemon_mut = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
                    Some(p) => p,
                    None => return false,
                };
                pokemon_mut.status = prev_status;
                pokemon_mut.status_state = prev_status_state;
                return false;
            }
        }

        // JS: if (status.id && !this.battle.runEvent('AfterSetStatus', this, source, sourceEffect, status)) {
        // JS:     return false;
        // JS: }
        // ✅ NOW IMPLEMENTED (Session 24 Part 78): runEvent('AfterSetStatus')
        // Note: JavaScript passes status as 5th parameter (relayVar), but Rust run_event only accepts Option<i32>
        //       Passing None for now - handlers can check target pokemon's status field which has been set
        if !status.as_str().is_empty() {
            let after_result = battle.run_event("AfterSetStatus", Some(crate::event::EventTarget::Pokemon(pokemon_pos)), source_pos, source_effect, EventResult::Continue, false, false);
            // runEvent returns Option<i32>, None or Some(0) means failure
            if matches!(after_result, EventResult::Number(0)) || matches!(after_result, EventResult::Null) {
                return false;
            }
        }

        // JS: return true;
        true
    }
}
