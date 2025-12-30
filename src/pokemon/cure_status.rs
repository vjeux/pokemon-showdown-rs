use crate::*;

impl Pokemon {

    /// Cure status condition
    /// Returns (cured, status_id, removed_nightmare) tuple for Battle to log
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
    pub fn cure_status(&mut self) -> Option<(String, bool)> {
        // JS: if (!this.hp || !this.status) return false;
        if self.hp == 0 || self.status.is_empty() {
            return None;
        }

        let status = self.status.as_str().to_string();

        // JS: if (this.status === 'slp' && this.removeVolatile('nightmare')) {
        let removed_nightmare = if status == "slp" {
            self.volatiles.remove(&ID::new("nightmare")).is_some()
        } else {
            false
        };

        // JS: this.setStatus('');
        self.set_status(ID::empty());
        self.status_state.duration = None;

        // Return (status_id, removed_nightmare) for caller to log
        Some((status, removed_nightmare))
    }
}
