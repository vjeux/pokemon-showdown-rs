//! Add Pseudo-Weather
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Battle action to add pseudo-weather conditions (e.g., Trick Room, Echoed Voice stacking)
//!
//! JavaScript equivalent: field.ts addPseudoWeather()

use crate::*;
use crate::event_system::EffectState;
use crate::dex_data::ID;
use crate::battle::Effect;

impl Battle {

    /// Add a pseudo-weather condition
    /// Equivalent to field.ts addPseudoWeather()
    ///
    /// JavaScript source (field.ts:285-315):
    // addPseudoWeather(
    //     status: string | Condition,
    //     source: Pokemon | 'debug' | null = null,
    //     sourceEffect: Effect | null = null
    // ): boolean {
    //     if (!source && this.battle.event?.target) source = this.battle.event.target;
    //     if (source === 'debug') source = this.battle.sides[0].active[0];
    //     status = this.battle.dex.conditions.get(status);
    //
    //     let state = this.pseudoWeather[status.id];
    //     if (state) {
    //         if (!(status as any).onFieldRestart) return false;
    //         return this.battle.singleEvent('FieldRestart', status, state, this, source, sourceEffect);
    //     }
    //     state = this.pseudoWeather[status.id] = this.battle.initEffectState({
    //         id: status.id,
    //         source,
    //         sourceSlot: source?.getSlot(),
    //         duration: status.duration,
    //     });
    //     if (status.durationCallback) {
    //         if (!source) throw new Error(`setting fieldcond without a source`);
    //         state.duration = status.durationCallback.call(this.battle, source, source, sourceEffect);
    //     }
    //     if (!this.battle.singleEvent('FieldStart', status, state, this, source, sourceEffect)) {
    //         delete this.pseudoWeather[status.id];
    //         return false;
    //     }
    //     this.battle.runEvent('PseudoWeatherChange', source, source, status);
    //     return true;
    // }
    pub fn add_pseudoweather(
        &mut self,
        condition_id: &str,
        source_pos: Option<(usize, usize)>,
        _source_effect: Option<&Effect>,
    ) -> bool {
        // JavaScript: if (!source && this.battle.event?.target) source = this.battle.event.target;
        let source_pos = source_pos.or_else(|| self.event.as_ref().and_then(|e| e.target));

        let id = ID::from(condition_id);

        debug_elog!("[ADD_PSEUDOWEATHER] Attempting to add pseudoweather: {}", condition_id);

        // JavaScript: let state = this.pseudoWeather[status.id];
        // JavaScript: if (state) {
        //     if (!(status as any).onFieldRestart) return false;
        //     return this.battle.singleEvent('FieldRestart', status, state, this, source, sourceEffect);
        // }
        if self.field.pseudo_weather.contains_key(&id) {
            debug_elog!("[ADD_PSEUDOWEATHER] Pseudoweather already exists, calling FieldRestart event");

            // Look up condition name from dex
            let condition_name = self.dex.conditions().get_by_id(&id)
                .and_then(|c| c.name.clone())
                .or_else(|| self.dex.moves().get(id.as_str()).map(|m| m.name.clone()))
                .unwrap_or_else(|| id.to_string());

            // JavaScript: return this.battle.singleEvent('FieldRestart', status, state, this, source, sourceEffect);
            // Set up effect context so callbacks can use with_effect_state to access/modify state
            let prev_context = self.set_effect_context(crate::Effect {
                id: id.clone(),
                name: condition_name,
                effect_type: crate::battle::EffectType::FieldCondition,
                effect_holder: None,
                side_index: None,
                prankster_boosted: false,
            });

            // Call the event
            let result = self.handle_condition_event("FieldRestart", condition_id, None);

            // Restore previous context
            self.restore_effect_context(prev_context);

            match result {
                crate::event::EventResult::Boolean(false) => {
                    debug_elog!("[ADD_PSEUDOWEATHER] FieldRestart returned false");
                    false
                }
                crate::event::EventResult::Null => {
                    debug_elog!("[ADD_PSEUDOWEATHER] FieldRestart returned null");
                    false
                }
                _ => {
                    debug_elog!("[ADD_PSEUDOWEATHER] FieldRestart succeeded");
                    true
                }
            }
        } else {
            // JavaScript: state = this.pseudoWeather[status.id] = this.battle.initEffectState({ ... });
            debug_elog!("[ADD_PSEUDOWEATHER] Creating new pseudoweather");

            let mut state = EffectState::new(id.clone());

            // JavaScript: source, sourceSlot: source?.getSlot(),
            state.source = source_pos;
            state.source_slot = source_pos.and_then(|pos| {
                self.pokemon_at(pos.0, pos.1).map(|p| p.position)
            });

            // Get duration from condition data
            // Check both standalone conditions and conditions embedded in moves
            let duration = self.dex.conditions().get_by_id(&id)
                .and_then(|c| c.duration)
                .or_else(|| {
                    self.dex.moves().get(id.as_str())
                        .and_then(|m| m.condition.as_ref())
                        .and_then(|c| c.duration)
                });
            state.duration = duration;
            debug_elog!("[ADD_PSEUDOWEATHER] Set initial duration: {:?}", duration);

            // JavaScript: if (!this.battle.singleEvent('FieldStart', status, state, this, source, sourceEffect)) {
            //     delete this.pseudoWeather[status.id];
            //     return false;
            // }

            // Add to field first so the event handlers can access it
            self.field.pseudo_weather.insert(id.clone(), state);

            // Look up condition name from dex
            let condition_name = self.dex.conditions().get_by_id(&id)
                .and_then(|c| c.name.clone())
                .or_else(|| self.dex.moves().get(id.as_str()).map(|m| m.name.clone()))
                .unwrap_or_else(|| id.to_string());

            // Set up effect context so callbacks can use with_effect_state to access/modify state
            let prev_context = self.set_effect_context(crate::Effect {
                id: id.clone(),
                name: condition_name,
                effect_type: crate::battle::EffectType::FieldCondition,
                effect_holder: None,
                side_index: None,
                prankster_boosted: false,
            });

            debug_elog!("[ADD_PSEUDOWEATHER] Calling FieldStart event");
            let result = self.handle_condition_event("FieldStart", condition_id, None);

            // Restore previous context
            self.restore_effect_context(prev_context);

            match result {
                crate::event::EventResult::Boolean(false) => {
                    debug_elog!("[ADD_PSEUDOWEATHER] FieldStart returned false, removing pseudoweather");
                    self.field.pseudo_weather.shift_remove(&id);
                    false
                }
                crate::event::EventResult::Null => {
                    debug_elog!("[ADD_PSEUDOWEATHER] FieldStart returned null, removing pseudoweather");
                    self.field.pseudo_weather.shift_remove(&id);
                    false
                }
                _ => {
                    debug_elog!("[ADD_PSEUDOWEATHER] FieldStart succeeded");
                    // JavaScript: this.battle.runEvent('PseudoWeatherChange', source, source, status);
                    // TODO: Implement PseudoWeatherChange event
                    true
                }
            }
        }
    }
}
