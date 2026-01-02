use crate::*;

impl Pokemon {

    /// Try to set status with immunity checks
    /// Equivalent to trySetStatus in pokemon.ts
    //
    // 	trySetStatus(status: string | Condition, source: Pokemon | null = null, sourceEffect: Effect | null = null) {
    // 		return this.setStatus(this.status || status, source, sourceEffect);
    // 	}
    //
    pub fn try_set_status(&mut self, status_id: ID, _source_effect: Option<&str>) -> bool {
        // JS: return this.setStatus(this.status || status, source, sourceEffect);
        //
        // If already has a status, setStatus will be called with the current status
        // and should return false. Otherwise, setStatus is called with the new status.
        //
        // Note: The JavaScript version passes (this.status || status) which evaluates to
        // the current status if it exists, or the new status if not. Then setStatus
        // checks if it's the same status and fails. This is functionally equivalent to
        // checking if we already have a status and returning false.
        if !self.status.is_empty() {
            return false;
        }

        // Call setStatus which will handle all immunity checks and events
        self.set_status(status_id, None, None, false)
    }
}
