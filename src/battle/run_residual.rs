use crate::*;

impl Battle {

    /// Process end-of-turn residual effects
    /// Equivalent to battle.ts case 'residual' (battle.ts:2810-2817)
    pub fn run_residual(&mut self) {
        // this.add('');
        self.add("", &[]);

        // this.clearActiveMove(true);
        self.clear_active_move(true);

        // this.updateSpeed();
        self.update_speed();

        // residualPokemon = this.getAllActive().map(pokemon => [pokemon, pokemon.getUndynamaxedHP()] as const);
        // Note: We don't track residualPokemon yet for EmergencyExit handling
        // This will be needed when implementing EmergencyExit abilities

        // this.eachEvent('Residual');
        // NOTE: In JavaScript, this processes item/ability residuals for each Pokemon
        self.each_event("Residual", None, None);

        // this.fieldEvent('Residual');
        self.field_event("Residual", None);

        // if (!this.ended) this.add('upkeep');
        if !self.ended {
            self.add("upkeep", &[]);
        }
    }
}
