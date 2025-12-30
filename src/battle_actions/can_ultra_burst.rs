use crate::*;

impl<'a> BattleActions<'a> {

    /// Check if Pokemon can Ultra Burst
    /// Equivalent to canUltraBurst in battle-actions.ts
    //
    // 	canUltraBurst(pokemon: Pokemon) {
    // 		if (['Necrozma-Dawn-Wings', 'Necrozma-Dusk-Mane'].includes(pokemon.baseSpecies.name) &&
    // 			pokemon.getItem().id === 'ultranecroziumz') {
    // 			return "Necrozma-Ultra";
    // 		}
    // 		return null;
    // 	}
    //
    pub fn can_ultra_burst(species_name: &str, item_id: &str) -> Option<String> {
        if (species_name == "Necrozma-Dawn-Wings" || species_name == "Necrozma-Dusk-Mane")
            && item_id == "ultranecroziumz"
        {
            return Some("Necrozma-Ultra".to_string());
        }
        None
    }
}
