// Set Weather
//
// JavaScript: field.ts setWeather()
//
// setWeather(status: string | Condition, source: Pokemon | 'debug' | null = null, sourceEffect: Effect | null = null) {
//     status = this.battle.dex.conditions.get(status);
//     if (!sourceEffect && this.battle.effect) sourceEffect = this.battle.effect;
//     if (!source && this.battle.event?.target) source = this.battle.event.target;
//     if (source === 'debug') source = this.battle.sides[0].active[0];
//
//     if (this.weather === status.id) {
//         if (sourceEffect && sourceEffect.effectType === 'Ability') {
//             if (this.battle.gen > 5 || this.weatherState.duration === 0) {
//                 return false;
//             }
//         } else if (this.battle.gen > 2 || status.id === 'sandstorm') {
//             return false;
//         }
//     }
//     if (source) {
//         const result = this.battle.runEvent('SetWeather', source, source, status);
//         if (!result) {
//             if (result === false) {
//                 if ((sourceEffect as Move)?.weather) {
//                     this.battle.add('-fail', source, sourceEffect, '[from] ' + this.weather);
//                 } else if (sourceEffect && sourceEffect.effectType === 'Ability') {
//                     this.battle.add('-ability', source, sourceEffect, '[from] ' + this.weather, '[fail]');
//                 }
//             }
//             return null;
//         }
//     }
//     const prevWeather = this.weather;
//     const prevWeatherState = this.weatherState;
//     this.weather = status.id;
//     this.weatherState = this.battle.initEffectState({ id: status.id });
//     if (source) {
//         this.weatherState.source = source;
//         this.weatherState.sourceSlot = source.getSlot();
//     }
//     if (status.duration) {
//         this.weatherState.duration = status.duration;
//     }
//     if (status.durationCallback) {
//         if (!source) throw new Error(`setting weather without a source`);
//         this.weatherState.duration = status.durationCallback.call(this.battle, source, source, sourceEffect);
//     }
//     if (!this.battle.singleEvent('FieldStart', status, this.weatherState, this, source, sourceEffect)) {
//         this.weather = prevWeather;
//         this.weatherState = prevWeatherState;
//         return false;
//     }
//     this.battle.eachEvent('WeatherChange', sourceEffect);
//     return true;
// }

use crate::Battle;
use crate::dex_data::ID;

impl Battle {
    /// Set the weather
    /// JavaScript: field.ts setWeather()
    ///
    /// Returns:
    /// - Some(true) if weather was set successfully
    /// - Some(false) if weather couldn't be set but was blocked by an event
    /// - None if weather is already active and nothing changed
    pub fn set_weather(
        &mut self,
        weather_id: ID,
        source_pos: Option<(usize, usize)>,
        _source_effect: Option<ID>,
    ) -> Option<bool> {
        // Get current weather for comparison
        let current_weather = self.field.weather.clone();

        // If weather is already active, check if we should skip
        if current_weather == weather_id {
            // JavaScript: if (this.battle.gen > 2 || status.id === 'sandstorm') return false;
            // For Gen 9 (current gen), always return false if weather is already active
            // TODO: Add generation check and ability check for older gens
            return Some(false);
        }

        // Run 'SetWeather' event - can block the weather change
        if source_pos.is_some() {
            let result = self.run_event_bool(
                "SetWeather",
                source_pos,
                source_pos,
                Some(&weather_id),
            );

            // If event returned false, weather change is blocked
            if !result {
                // TODO: Add failure messages
                // JavaScript adds '-fail' or '-ability' messages here
                return None;
            }
        }

        // Save previous weather state in case we need to revert
        let prev_weather = self.field.weather.clone();
        let prev_weather_state = self.field.weather_state.clone();

        // Set new weather
        self.field.weather = weather_id.clone();
        self.field.weather_state = crate::event_system::EffectState::new(weather_id.clone());

        // Set source if provided
        if let Some(source_position) = source_pos {
            self.field.weather_state.source = Some(source_position);
        }

        // Set duration from condition data
        // JavaScript: if (status.duration) this.weatherState.duration = status.duration;
        // TODO: Get duration from dex condition data and call durationCallback
        // For now, hardcode duration=5 for all weather (standard Gen 9 duration)
        self.field.weather_state.duration = Some(5);

        // Fire 'FieldStart' event on the weather
        // If it returns false, revert to previous weather
        let field_start_result = self.single_event(
            "FieldStart",
            &weather_id,
            None, // field as target - we don't pass this in current arch
            source_pos,
            None,
        );

        // Check if the event returned false (using the EventResult enum)
        if matches!(field_start_result, crate::event::EventResult::Boolean(false)) {
            // Revert to previous weather
            self.field.weather = prev_weather;
            self.field.weather_state = prev_weather_state;
            return Some(false);
        }

        // Fire 'WeatherChange' event
        self.each_event("WeatherChange", None, None);

        Some(true)
    }
}
