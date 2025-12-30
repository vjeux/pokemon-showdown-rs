use crate::*;

impl<'a> BattleActions<'a> {

    /// Check if Pokemon can Terastallize
    /// Equivalent to canTerastallize in battle-actions.ts
    //
    // 	canTerastallize(pokemon: Pokemon) {
    // 		if (pokemon.getItem().zMove || pokemon.canMegaEvo || this.dex.gen !== 9) {
    // 			return null;
    // 		}
    // 		return pokemon.teraType;
    // 	}
    //
    pub fn can_terastallize(
        item_is_z_move: bool,
        can_mega_evo: bool,
        gen: u8,
        tera_type: Option<&str>,
    ) -> Option<String> {
        if item_is_z_move || can_mega_evo || gen != 9 {
            return None;
        }
        tera_type.map(|t| t.to_string())
    }
}
