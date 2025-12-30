use crate::*;

impl Pokemon {

    /// Update speed (called when boosts or conditions change)
    /// Equivalent to pokemon.ts updateSpeed() (pokemon.ts)
    //
    // 	updateSpeed() {
    // 		this.speed = this.getActionSpeed();
    // 	}
    //
    pub fn update_speed(&mut self, battle: &Battle) {
        // JS: this.speed = this.getActionSpeed();
        self.speed = self.get_action_speed(battle);
    }
}
