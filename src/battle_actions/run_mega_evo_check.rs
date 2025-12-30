use crate::*;

impl<'a> BattleActions<'a> {

    // =========================================================================
    // MEGA EVOLUTION / TERASTALLIZE EXECUTION
    // =========================================================================

    /// Run Mega Evolution
    /// Equivalent to runMegaEvo in battle-actions.ts
    /// Returns the mega forme ID if successful
    pub fn run_mega_evo_check(
        _species_name: &str,
        mega_forme: Option<&str>,
        already_mega: bool,
    ) -> Option<String> {
        if already_mega {
            return None;
        }
        mega_forme.map(|s| s.to_string())
    }
}
