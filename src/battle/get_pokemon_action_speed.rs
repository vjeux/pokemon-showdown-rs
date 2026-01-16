// NOTE: This method is NOT in JavaScript - Rust-specific implementation

use crate::*;
use crate::dex_data::StatID;

impl Battle {

    /// Get a Pokemon's action speed (equivalent to pokemon.getActionSpeed() in JavaScript)
    /// JavaScript source (sim/pokemon.ts):
    /// ```js
    /// getActionSpeed() {
    ///     let speed = this.getStat('spe', false, false);
    ///     const trickRoomCheck = this.battle.ruleTable.has('twisteddimensionmod') ?
    ///         !this.battle.field.getPseudoWeather('trickroom') : this.battle.field.getPseudoWeather('trickroom');
    ///     if (trickRoomCheck) {
    ///         speed = 10000 - speed;
    ///     }
    ///     return this.battle.trunc(speed, 13);
    /// }
    /// ```
    pub fn get_pokemon_action_speed(&mut self, side_idx: usize, poke_idx: usize) -> i32 {
        if let Some(_pokemon) = self.sides.get(side_idx).and_then(|s| s.pokemon.get(poke_idx)) {
            // let speed = this.getStat('spe', false, false);
            let speed = self.get_pokemon_stat((side_idx, poke_idx), StatID::Spe, false, false);
            debug_elog!("[GET_POKEMON_ACTION_SPEED] turn={}, pokemon=({}, {}), speed={}", self.turn, side_idx, poke_idx, speed);

            // const trickRoomCheck = this.battle.ruleTable.has('twisteddimensionmod') ?
            //     !this.battle.field.getPseudoWeather('trickroom') : this.battle.field.getPseudoWeather('trickroom');
            let trick_room_active = self.field.has_pseudo_weather(&ID::from("trickroom"));
            let twisted_dimension_mod = self.rule_table.as_ref().map_or(false, |rt| rt.has("twisteddimensionmod"));
            let trick_room_check = if twisted_dimension_mod {
                !trick_room_active
            } else {
                trick_room_active
            };

            // if (trickRoomCheck) { speed = 10000 - speed; }
            let speed = if trick_room_check {
                10000 - speed
            } else {
                speed
            };

            // return this.battle.trunc(speed, 13);
            self.trunc(speed as f64, Some(13)) as i32
        } else {
            0
        }
    }
}
