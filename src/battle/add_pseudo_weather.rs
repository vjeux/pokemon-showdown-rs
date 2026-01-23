///! Battle add_pseudo_weather method
///!
///! JavaScript source (field.ts addPseudoWeather):
// addPseudoWeather(
// 		status: string | Condition,
// 		source: Pokemon | 'debug' | null = null,
// 		sourceEffect: Effect | null = null
// 	): boolean {
// 		if (!source && this.battle.event?.target) source = this.battle.event.target;
// 		if (source === 'debug') source = this.battle.sides[0].active[0];
// 		status = this.battle.dex.conditions.get(status);
//
// 		let state = this.pseudoWeather[status.id];
// 		if (state) {
// 			if (!(status as any).onFieldRestart) return false;
// 			return this.battle.singleEvent('FieldRestart', status, state, this, source, sourceEffect);
// 		}
// 		state = this.pseudoWeather[status.id] = this.battle.initEffectState({
// 			id: status.id,
// 			source,
// 			sourceSlot: source?.getSlot(),
// 			duration: status.duration,
// 		});
// 		if (status.durationCallback) {
// 			if (!source) throw new Error(`setting fieldcond without a source`);
// 			state.duration = status.durationCallback.call(this.battle, source, source, sourceEffect);
// 		}
// 		if (!this.battle.singleEvent('FieldStart', status, state, this, source, sourceEffect)) {
// 			delete this.pseudoWeather[status.id];
// 			return false;
// 		}
// 		this.battle.runEvent('PseudoWeatherChange', source, source, status);
// 		return true;
// 	}

use crate::battle::Battle;
use crate::dex_data::ID;
use crate::event_system::EffectState;

impl Battle {
    pub fn add_pseudo_weather(&mut self, pseudo_weather_id: ID, source_pos: Option<(usize, usize)>) -> bool {
        // let state = this.pseudoWeather[status.id];
        // if (state) {
        if self.field.pseudo_weather.contains_key(&pseudo_weather_id) {
            // if (!(status as any).onFieldRestart) return false;
            // Check if the condition has an onFieldRestart callback
            if !self.condition_has_callback(&pseudo_weather_id.as_str(), "FieldRestart") {
                return false;
            }
            // return this.battle.singleEvent('FieldRestart', status, state, this, source, sourceEffect);
            // Call the FieldRestart event which may toggle off the pseudo weather
            let result = self.single_event(
                "FieldRestart",
                &crate::battle::Effect::field_condition(pseudo_weather_id.clone()),
                None,
                None,  // field as target
                source_pos,
                None,
                None,
            );
            // FieldRestart should return true if handled, false otherwise
            return !matches!(result, crate::event::EventResult::Boolean(false));
        }

        // state = this.pseudoWeather[status.id] = this.battle.initEffectState({
        //     id: status.id,
        //     source,
        //     sourceSlot: source?.getSlot(),
        //     duration: status.duration,
        // });
        let mut state = EffectState::new(pseudo_weather_id.clone());
        state.source = source_pos;
        // Set unique effect_order for tie-breaking when sorting handlers
        // JavaScript: initEffectState increments effectOrder counter
        state.effect_order = self.effect_order;
        self.effect_order += 1;

        // ✅ IMPLEMENTED: Look up duration from dex
        // JavaScript: duration: status.duration
        if let Some(condition_data) = self.dex.conditions().get_by_id(&pseudo_weather_id) {
            state.duration = condition_data.duration;
        }

        // if (status.durationCallback) {
        //     if (!source) throw new Error(`setting fieldcond without a source`);
        //     state.duration = status.durationCallback.call(this.battle, source, source, sourceEffect);
        // }
        // Call durationCallback if it exists for this pseudo-weather
        if source_pos.is_some() {
            let duration_result = self.call_duration_callback(
                &pseudo_weather_id,
                source_pos,  // target
                source_pos,  // source (same as target in JS)
                None,        // sourceEffect
            );
            if let crate::event::EventResult::Number(duration) = duration_result {
                state.duration = Some(duration);
            }
        }

        // if (!this.battle.singleEvent('FieldStart', status, state, this, source, sourceEffect)) {
        //     delete this.pseudoWeather[status.id];
        //     return false;
        // }
        let field_start_result = self.single_event(
            "FieldStart",
            &crate::battle::Effect::field_condition(pseudo_weather_id.clone()),
            None,
            None,  // field as target
            source_pos,
            None,
            None,
        );

        // Check if event returned false (event system returns Boolean(false) on failure)
        if matches!(field_start_result, crate::event::EventResult::Boolean(false)) {
            // Don't add to pseudo_weather
            return false;
        }

        // Add to pseudo_weather
        self.field.pseudo_weather.insert(pseudo_weather_id.clone(), state);

        // this.battle.runEvent('PseudoWeatherChange', source, source, status);
        // TODO: Implement PseudoWeatherChange event
        // self.run_event("PseudoWeatherChange", source_pos, source_pos);

        true
    }
}
