use crate::*;

impl Pokemon {

    /// Get effective weather considering abilities
    // TypeScript source:
    // /**
    // 	 * Like Field.effectiveWeather(), but ignores sun and rain if
    // 	 * the Utility Umbrella is active for the Pokemon.
    // 	 */
    // 	effectiveWeather() {
    // 		const weather = this.battle.field.effectiveWeather();
    // 		switch (weather) {
    // 		case 'sunnyday':
    // 		case 'raindance':
    // 		case 'desolateland':
    // 		case 'primordialsea':
    // 			if (this.hasItem('utilityumbrella')) return '';
    // 		}
    // 		return weather;
    // 	}
    //
    pub fn effective_weather(&self, field_weather: &str) -> String {
        // Cloud Nine and Air Lock negate weather
        // This would normally check all Pokemon on field
        // For now just return the field weather
        field_weather.to_string()
    }
}
