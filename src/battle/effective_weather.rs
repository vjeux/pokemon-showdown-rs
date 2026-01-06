///! Battle effective_weather method
///!
///! JavaScript source (field.ts effectiveWeather):
// 	effectiveWeather() {
// 		if (this.suppressingWeather()) return '';
// 		return this.weather;
// 	}

use crate::battle::Battle;
use crate::dex_data::ID;

impl Battle {
    pub fn effective_weather(&self) -> ID {
        // if (this.suppressingWeather()) return '';
        if self.suppressing_weather() {
            return ID::empty();
        }
        // return this.weather;
        self.field.weather.clone()
    }
}
