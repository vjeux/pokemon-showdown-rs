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
            // return this.battle.singleEvent('FieldRestart', status, state, this, source, sourceEffect);
            // TODO: Implement FieldRestart event
            return false;
        }

        // state = this.pseudoWeather[status.id] = this.battle.initEffectState({
        //     id: status.id,
        //     source,
        //     sourceSlot: source?.getSlot(),
        //     duration: status.duration,
        // });
        let mut state = EffectState::new(pseudo_weather_id.clone());
        state.source = source_pos;
        // TODO: Look up duration from dex
        // if (status.durationCallback) {
        //     if (!source) throw new Error(`setting fieldcond without a source`);
        //     state.duration = status.durationCallback.call(this.battle, source, source, sourceEffect);
        // }

        // if (!this.battle.singleEvent('FieldStart', status, state, this, source, sourceEffect)) {
        //     delete this.pseudoWeather[status.id];
        //     return false;
        // }
        let field_start_result = self.single_event(
            "FieldStart",
            &pseudo_weather_id,
            None,  // field as target
            source_pos,
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
