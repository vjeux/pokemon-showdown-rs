use crate::*;
use crate::dex_data::StatID;

impl Pokemon {

    /// Update speed (called when boosts or conditions change)
    //
    // 	updateSpeed() {
    // 		this.speed = this.getActionSpeed();
    // 	}
    //
    pub fn update_speed(&mut self) {
        self.speed = self.get_stat(StatID::Spe, false);
    }
}
