///! Battle remove_pseudo_weather method
///!
///! JavaScript source (field.ts removePseudoWeather):
// 	removePseudoWeather(status: string | Effect) {
// 		status = this.battle.dex.conditions.get(status);
// 		const state = this.pseudoWeather[status.id];
// 		if (!state) return false;
// 		this.battle.singleEvent('FieldEnd', status, state, this);
// 		delete this.pseudoWeather[status.id];
// 		return true;
// 	}

use crate::battle::Battle;
use crate::dex_data::ID;

impl Battle {
    pub fn remove_pseudo_weather(&mut self, id: &ID) -> bool {
        // status = this.battle.dex.conditions.get(status);
        // const state = this.pseudoWeather[status.id];
        // if (!state) return false;
        if !self.field.pseudo_weather.contains_key(id) {
            return false;
        }

        // this.battle.singleEvent('FieldEnd', status, state, this);
        self.single_event(
            "FieldEnd",
            id,
            None,  // field as target
            None,
            None,
        );

        // delete this.pseudoWeather[status.id];
        self.field.pseudo_weather.remove(id);

        true
    }
}
