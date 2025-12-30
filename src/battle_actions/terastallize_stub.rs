use crate::*;
use crate::battle_actions::TerastallizeResult;

impl<'a> BattleActions<'a> {

    /// Terastallize a Pokemon
    /// Equivalent to battle-actions.ts terastallize()
    pub fn terastallize_stub(
        pokemon_index: usize,
        tera_type: &str,
        species_base_species: &str,
    ) -> TerastallizeResult {
        let _ = pokemon_index;

        // Handle Ogerpon special case
        if species_base_species == "Ogerpon"
            && !["Fire", "Grass", "Rock", "Water"].contains(&tera_type)
        {
            return TerastallizeResult::InvalidOgerpon;
        }

        TerastallizeResult::Success {
            tera_type: tera_type.to_string(),
            forme_change: if species_base_species == "Ogerpon" {
                Some("ogerpontealtera".to_string())
            } else if species_base_species == "Terapagos-Terastal" {
                Some("Terapagos-Stellar".to_string())
            } else {
                None
            },
        }
    }
}
