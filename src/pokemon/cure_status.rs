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
    pub fn cure_status(&mut self, silent: bool) -> Option<(String, bool, bool)> {
        // JS: if (!this.hp || !this.status) return false;
        if self.hp == 0 || self.status.is_empty() {
            return None;
        }

        let status = self.status.as_str().to_string();

        // JS: this.battle.add('-curestatus', this, this.status, silent ? '[silent]' : '[msg]');
        // ✅ NOW IMPLEMENTED: silent parameter support
        // Returns status and silent flag for caller to log

        // JS: if (this.status === 'slp' && this.removeVolatile('nightmare')) {
        // JS:     this.battle.add('-end', this, 'Nightmare', '[silent]');
        // JS: }
        let removed_nightmare = if status == "slp" {
            self.volatiles.remove(&ID::new("nightmare")).is_some()
        } else {
            false
        };

        // JS: this.setStatus('');
        self.set_status(ID::empty(), None, None, false);
        self.status_state.duration = None;

        // Return (status_id, removed_nightmare, silent) for caller to log
        // Caller should call:
        // - battle.add('-curestatus', pokemon, status, silent ? '[silent]' : '[msg]');
        // - if removed_nightmare: battle.add('-end', pokemon, 'Nightmare', '[silent]');
        Some((status, removed_nightmare, silent))
    }
}
