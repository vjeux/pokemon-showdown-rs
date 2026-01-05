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
        _source_pos: Option<(usize, usize)>,
        _source_effect: Option<&ID>,
    ) -> bool {
        let id = ID::from(condition_id);

        eprintln!("[ADD_PSEUDOWEATHER] Attempting to add pseudoweather: {}", condition_id);

        // JavaScript: let state = this.pseudoWeather[status.id];
        // JavaScript: if (state) {
        //     if (!(status as any).onFieldRestart) return false;
        //     return this.battle.singleEvent('FieldRestart', status, state, this, source, sourceEffect);
        // }
        if self.field.pseudo_weather.contains_key(&id) {
            eprintln!("[ADD_PSEUDOWEATHER] Pseudoweather already exists, calling FieldRestart event");

            // JavaScript: return this.battle.singleEvent('FieldRestart', status, state, this, source, sourceEffect);
            // singleEvent sets this.effectState = state before calling the callback
            // We need to clone the state, set it, call the event, then write it back

            // Get the current state
            let state = self.field.pseudo_weather.get(&id).unwrap().clone();

            // Set current_effect_state to the cloned state
            let prev_effect_state = self.current_effect_state.take();
            self.current_effect_state = Some(state.clone());

            // Call the event
            let result = self.handle_condition_event("FieldRestart", condition_id, None);

            // Get the modified state back and update the field
            if let Some(modified_state) = self.current_effect_state.take() {
                self.field.pseudo_weather.insert(id.clone(), modified_state);
            }

            // Restore previous effect state
            self.current_effect_state = prev_effect_state;

            match result {
                crate::event::EventResult::Boolean(false) => {
                    eprintln!("[ADD_PSEUDOWEATHER] FieldRestart returned false");
                    false
                }
                crate::event::EventResult::Null => {
                    eprintln!("[ADD_PSEUDOWEATHER] FieldRestart returned null");
                    false
                }
                _ => {
                    eprintln!("[ADD_PSEUDOWEATHER] FieldRestart succeeded");
                    true
                }
            }
        } else {
            // JavaScript: state = this.pseudoWeather[status.id] = this.battle.initEffectState({ ... });
            eprintln!("[ADD_PSEUDOWEATHER] Creating new pseudoweather");

            let mut state = EffectState::new(id.clone());
            // TODO: Set duration from condition data
            state.duration = None;

            // JavaScript: if (!this.battle.singleEvent('FieldStart', status, state, this, source, sourceEffect)) {
            //     delete this.pseudoWeather[status.id];
            //     return false;
            // }

            // Add to field first so the event handlers can access it
            self.field.pseudo_weather.insert(id.clone(), state.clone());

            // Set current_effect_state to the new state
            let prev_effect_state = self.current_effect_state.take();
            self.current_effect_state = Some(state);

            eprintln!("[ADD_PSEUDOWEATHER] Calling FieldStart event");
            let result = self.handle_condition_event("FieldStart", condition_id, None);

            // Get the modified state back and update the field
            if let Some(modified_state) = self.current_effect_state.take() {
                self.field.pseudo_weather.insert(id.clone(), modified_state);
            }

            // Restore previous effect state
            self.current_effect_state = prev_effect_state;

            match result {
                crate::event::EventResult::Boolean(false) => {
                    eprintln!("[ADD_PSEUDOWEATHER] FieldStart returned false, removing pseudoweather");
                    self.field.pseudo_weather.remove(&id);
                    false
                }
                crate::event::EventResult::Null => {
                    eprintln!("[ADD_PSEUDOWEATHER] FieldStart returned null, removing pseudoweather");
                    self.field.pseudo_weather.remove(&id);
                    false
                }
                _ => {
                    eprintln!("[ADD_PSEUDOWEATHER] FieldStart succeeded");
                    // JavaScript: this.battle.runEvent('PseudoWeatherChange', source, source, status);
                    // TODO: Implement PseudoWeatherChange event
                    true
                }
            }
        }
    }
}
