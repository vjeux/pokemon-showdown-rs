use crate::*;
use crate::dex_data::StatID;

impl Pokemon {

    /// Get the action speed (speed used for turn order)
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
    pub fn get_action_speed(&self) -> i32 {
        let mut speed = self.get_stat(StatID::Spe, false);

        // Paralysis halves speed
        if self.status.as_str() == "par" {
            speed /= 2;
        }

        speed
    }
}
