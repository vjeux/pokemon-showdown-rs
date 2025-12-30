use crate::*;
use crate::dex_data::StatID;

impl Pokemon {

    /// Get the action speed (speed used for turn order)
    /// Equivalent to pokemon.ts getActionSpeed() (pokemon.ts:8-16)
    //
    // 	getActionSpeed() {
    // 		let speed = this.getStat('spe', false, false);
    // 		const trickRoomCheck = this.battle.ruleTable.has('twisteddimensionmod') ?
    // 			!this.battle.field.getPseudoWeather('trickroom') : this.battle.field.getPseudoWeather('trickroom');
    // 		if (trickRoomCheck) {
    // 			speed = 10000 - speed;
    // 		}
    // 		return this.battle.trunc(speed, 13);
    // 	}
    //
    pub fn get_action_speed(&self, battle: &Battle) -> i32 {
        // JS: let speed = this.getStat('spe', false, false);
        let mut speed = self.get_stat(StatID::Spe, false);

        // JS: const trickRoomCheck = this.battle.ruleTable.has('twisteddimensionmod') ?
        // JS:     !this.battle.field.getPseudoWeather('trickroom') : this.battle.field.getPseudoWeather('trickroom');
        // JS: if (trickRoomCheck) {
        // JS:     speed = 10000 - speed;
        // JS: }
        // Check for Trick Room pseudo-weather
        // Note: 'twisteddimensionmod' rule check not yet implemented
        let trick_room_id = ID::new("trickroom");
        if battle.field.has_pseudo_weather(&trick_room_id) {
            speed = 10000 - speed;
        }

        // JS: return this.battle.trunc(speed, 13);
        // Truncate to 13 bits (max value 8191) for Gen compatibility
        // Note: trunc with bits parameter is only used in older gens
        speed = speed.min(10000);

        speed
    }
}
