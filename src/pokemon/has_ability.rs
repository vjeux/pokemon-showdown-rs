use crate::*;

impl Pokemon {

    /// Check if Pokemon has a specific ability
    //
    // 	hasAbility(ability: string | string[]) {
    // 		if (Array.isArray(ability)) {
    // 			if (!ability.map(toID).includes(this.ability)) return false;
    // 		} else {
    // 			if (toID(ability) !== this.ability) return false;
    // 		}
    // 		return !this.ignoringAbility();
    // 	}
    //
    pub fn has_ability(&self, abilities: &[&str]) -> bool {
        let ability_id = self.ability.as_str();
        abilities
            .iter()
            .any(|&a| crate::dex_data::to_id(a) == ability_id)
    }
}
