use crate::*;

impl Battle {

    /// Process end-of-turn residual effects
    /// Equivalent to battle.ts case 'residual' (battle.ts:2810-2817)
    pub fn run_residual(&mut self) {
        // this.add('');
        self.add_log("", &[]);

        // this.clearActiveMove(true);
        self.clear_active_move(true);

        // this.updateSpeed();
        self.update_speed();

        // residualPokemon = this.getAllActive().map(pokemon => [pokemon, pokemon.getUndynamaxedHP()] as const);
        // Note: We don't track residualPokemon yet for EmergencyExit handling
        // This will be needed when implementing EmergencyExit abilities

        // this.fieldEvent('Residual');
        self.field_event("Residual");

        // if (!this.ended) this.add('upkeep');
        if !self.ended {
            self.add_log("upkeep", &[]);
        }
    }
}
