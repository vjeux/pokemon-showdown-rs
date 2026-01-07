///! Battle clear_weather method
///!
///! JavaScript source (field.ts clearWeather):
// 	clearWeather() {
// 		if (!this.weather) return false;
// 		const prevWeather = this.getWeather();
// 		this.battle.singleEvent('FieldEnd', prevWeather, this.weatherState, this);
// 		this.weather = '';
// 		this.battle.clearEffectState(this.weatherState);
// 		this.battle.eachEvent('WeatherChange');
// 		return true;
// 	}

use crate::battle::Battle;
use crate::dex_data::ID;
use crate::event_system::EffectState;

impl Battle {
    pub fn clear_weather(&mut self) -> bool {
        // if (!this.weather) return false;
        if self.field.weather.is_empty() {
            return false;
        }

        // const prevWeather = this.getWeather();
        let prev_weather = self.field.weather.clone();

        // this.battle.singleEvent('FieldEnd', prevWeather, this.weatherState, this);
        self.single_event(
            "FieldEnd",
            &crate::battle::Effect::weather(prev_weather.clone()),
            None,
            None,  // field as target
            None,
            None,
            None,
        );

        // this.weather = '';
        self.field.weather = ID::empty();

        // this.battle.clearEffectState(this.weatherState);
        self.field.weather_state = EffectState::new(ID::empty());

        // this.battle.eachEvent('WeatherChange');
        self.each_event("WeatherChange", None, None);

        true
    }
}
