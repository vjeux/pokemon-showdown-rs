use crate::*;

impl<'a> BattleActions<'a> {

    /// Run Mega Evolution
    /// Equivalent to battle-actions.ts runMegaEvo()
    pub fn run_mega_evo_stub(
        pokemon_index: usize,
        can_mega_evo: Option<&str>,
        can_ultra_burst: Option<&str>,
    ) -> Option<String> {
        // Return the target forme if mega evolution is possible
        let _ = pokemon_index;

        if let Some(species_id) = can_mega_evo {
            return Some(species_id.to_string());
        }

        if let Some(species_id) = can_ultra_burst {
            return Some(species_id.to_string());
        }

        None
    }
}
