use crate::*;

impl Pokemon {

    /// Cure status condition
    /// Returns (cured, status_id, removed_nightmare, silent) tuple for Battle to log
    // TypeScript source:
    // /** Unlike clearStatus, gives cure message */
    // 	cureStatus(silent = false) {
    // 		if (!this.hp || !this.status) return false;
    // 		this.battle.add('-curestatus', this, this.status, silent ? '[silent]' : '[msg]');
    // 		if (this.status === 'slp' && this.removeVolatile('nightmare')) {
    // 			this.battle.add('-end', this, 'Nightmare', '[silent]');
    // 		}
    // 		this.setStatus('');
    // 		return true;
    // 	}
    //
    // NOTE: Due to Rust borrow checker limitations, this returns data for the caller
    // to handle battle.add() calls, since we can't have &mut Pokemon and &mut Battle simultaneously
    pub fn cure_status(
        battle: &mut Battle,
        pokemon_pos: (usize, usize),
        silent: bool
    ) -> Option<(String, bool, bool)> {
        // Phase 1: Extract status info
        let (hp, status) = {
            let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => return None,
            };
            (pokemon.hp, pokemon.status.clone())
        };

        // JS: if (!this.hp || !this.status) return false;
        if hp == 0 || status.is_empty() {
            return None;
        }

        let status_str = status.as_str().to_string();

        // JS: this.battle.add('-curestatus', this, this.status, silent ? '[silent]' : '[msg]');
        // ✅ NOW IMPLEMENTED: silent parameter support
        // Returns status and silent flag for caller to log

        // Phase 2: Remove nightmare volatile if needed
        // JS: if (this.status === 'slp' && this.removeVolatile('nightmare')) {
        // JS:     this.battle.add('-end', this, 'Nightmare', '[silent]');
        // JS: }
        let removed_nightmare = if status_str == "slp" {
            let pokemon_mut = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => return None,
            };
            pokemon_mut.volatiles.remove(&ID::new("nightmare")).is_some()
        } else {
            false
        };

        // Phase 3: Clear status
        // JS: this.setStatus('');
        Pokemon::set_status(battle, pokemon_pos, ID::empty(), None, None, false);

        // Clear duration
        let pokemon_mut = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return None,
        };
        pokemon_mut.status_state.duration = None;

        // Return (status_id, removed_nightmare, silent) for caller to log
        // Caller should call:
        // - battle.add('-curestatus', pokemon, status, silent ? '[silent]' : '[msg]');
        // - if removed_nightmare: battle.add('-end', pokemon, 'Nightmare', '[silent]');
        Some((status_str, removed_nightmare, silent))
    }
}
