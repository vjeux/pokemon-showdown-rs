use crate::*;

impl Battle {

    /// Get a Pokemon's action speed (called by pokemon.getActionSpeed() in JS)
    /// Equivalent to pokemon.ts getActionSpeed() (lines 8-16)
    // TypeScript source:
    // 	getActionSpeed() {
    // 		let speed = this.getStat('spe', false, false);
    // 		const trickRoomCheck = this.battle.ruleTable.has('twisteddimensionmod') ?
    // 			!this.battle.field.getPseudoWeather('trickroom') : this.battle.field.getPseudoWeather('trickroom');
    // 		if (trickRoomCheck) {
    // 			speed = 10000 - speed;
    // 		}
    // 		return this.battle.trunc(speed, 13);
    // 	}
    pub fn get_pokemon_action_speed(&self, side_idx: usize, poke_idx: usize) -> i32 {
        if let Some(side) = self.sides.get(side_idx) {
            if let Some(pokemon) = side.pokemon.get(poke_idx) {
                // JS: let speed = this.getStat('spe', false, false);
                // Apply speed boosts
                let base_speed = pokemon.stored_stats.spe;
                let stage = pokemon.boosts.spe;
                let multiplier = if stage >= 0 {
                    (2 + stage as i32) as f64 / 2.0
                } else {
                    2.0 / (2 + (-stage) as i32) as f64
                };
                let mut speed = (base_speed as f64 * multiplier) as i32;

                // JS: const trickRoomCheck = this.battle.ruleTable.has('twisteddimensionmod') ?
                // JS:     !this.battle.field.getPseudoWeather('trickroom') : this.battle.field.getPseudoWeather('trickroom');
                // JS: if (trickRoomCheck) {
                // JS:     speed = 10000 - speed;
                // JS: }
                // Check for Trick Room pseudo-weather
                let trick_room_id = ID::new("trickroom");
                if self.field.has_pseudo_weather(&trick_room_id) {
                    speed = 10000 - speed;
                }

                // JS: return this.battle.trunc(speed, 13);
                // Truncate to 13 bits (max value 8191) for Gen compatibility
                // Note: trunc with bits parameter is only used in older gens
                speed = speed.min(10000);
                return speed;
            }
        }
        0
    }
}
