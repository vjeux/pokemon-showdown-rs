// JS Source:
//
// /**
//  * Like Field.effectiveWeather(), but ignores sun and rain if
//  * the Utility Umbrella is active for the Pokemon.
//  */
// effectiveWeather() {
// 	const weather = this.battle.field.effectiveWeather();
// 	switch (weather) {
// 	case 'sunnyday':
// 	case 'raindance':
// 	case 'desolateland':
// 	case 'primordialsea':
// 		if (this.hasItem('utilityumbrella')) return '';
// 	}
// 	return weather;
// }

use crate::*;

impl Pokemon {
    /// Get effective weather considering abilities and Utility Umbrella
    /// Equivalent to pokemon.ts effectiveWeather()
    pub fn effective_weather<'a>(&self, battle: &Battle, field_weather: &'a str) -> &'a str {
        // JS: switch (weather) {
        // JS: case 'sunnyday': case 'raindance': case 'desolateland': case 'primordialsea':
        //         if (this.hasItem('utilityumbrella')) return '';
        match field_weather {
            "sunnyday" | "raindance" | "desolateland" | "primordialsea" => {
                // JS: if (this.hasItem('utilityumbrella')) return '';
                if self.has_item(battle, &["utilityumbrella"]) {
                    "" // Weather negated by Utility Umbrella
                } else {
                    field_weather
                }
            }
            // JS: return weather;
            _ => field_weather,
        }
    }
}
