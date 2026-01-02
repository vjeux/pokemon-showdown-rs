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
    pub fn get_action_speed(&self, battle: &mut Battle) -> i32 {
        // JS: let speed = this.getStat('spe', false, false);
        let pokemon_pos = (self.side_index, self.position);
        let mut speed = battle.get_pokemon_stat(pokemon_pos, StatID::Spe, false, false);

        // JS: const trickRoomCheck = this.battle.ruleTable.has('twisteddimensionmod') ?
        // JS:     !this.battle.field.getPseudoWeather('trickroom') : this.battle.field.getPseudoWeather('trickroom');
        let trick_room_id = ID::new("trickroom");
        let has_trick_room = battle.field.has_pseudo_weather(&trick_room_id);

        let trick_room_check = if let Some(ref rule_table) = battle.rule_table {
            if rule_table.has("twisteddimensionmod") {
                !has_trick_room
            } else {
                has_trick_room
            }
        } else {
            has_trick_room
        };

        // JS: if (trickRoomCheck) {
        // JS:     speed = 10000 - speed;
        // JS: }
        if trick_room_check {
            speed = 10000 - speed;
        }

        // JS: return this.battle.trunc(speed, 13);
        // Truncate to 13 bits (max value 8191) for Gen compatibility
        battle.trunc(speed as f64, Some(13)) as i32
    }
}
