///! Battle is_weather method
///!
///! JavaScript source (field.ts isWeather):
// 	isWeather(weather: string | string[]) {
// 		const ourWeather = this.effectiveWeather();
// 		if (!Array.isArray(weather)) {
// 			return ourWeather === toID(weather);
// 		}
// 		return weather.map(toID).includes(ourWeather);
// 	}

use crate::battle::Battle;
use crate::dex_data::ID;

impl Battle {
    pub fn is_weather(&self, weather: &str) -> bool {
        // const ourWeather = this.effectiveWeather();
        let our_weather = self.effective_weather();
        // if (!Array.isArray(weather)) {
        // 	return ourWeather === toID(weather);
        // }
        let weather_id = ID::new(weather);
        our_weather == weather_id
        // NOTE: Array case handled at call sites
    }
}
