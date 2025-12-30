use crate::*;

impl Pokemon {

    /// Update speed (called when boosts or conditions change)
    //
    // 	updateSpeed() {
    // 		this.speed = this.getActionSpeed();
    // 	}
    //
    pub fn update_speed(&mut self) {
        // JS: this.speed = this.getActionSpeed();
        self.speed = self.get_action_speed();
    }
}
