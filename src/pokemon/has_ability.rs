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
    pub fn has_ability(&self, battle: &Battle, abilities: &[&str]) -> bool {
        // JS: if (Array.isArray(ability)) {
        // JS:     if (!ability.map(toID).includes(this.ability)) return false;
        // JS: } else {
        // JS:     if (toID(ability) !== this.ability) return false;
        // JS: }
        let ability_id = self.ability.as_str();
        let matches = abilities
            .iter()
            .any(|&a| crate::dex_data::to_id(a) == ability_id);

        if !matches {
            return false;
        }

        // JS: return !this.ignoringAbility();
        !self.ignoring_ability(battle)
    }
}
