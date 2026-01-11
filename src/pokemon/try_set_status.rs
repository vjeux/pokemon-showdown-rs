use crate::*;
use crate::battle::Effect;

impl Pokemon {

    /// Try to set status with immunity checks
    /// Equivalent to trySetStatus in pokemon.ts
    //
    // 	trySetStatus(status: string | Condition, source: Pokemon | null = null, sourceEffect: Effect | null = null) {
    // 		return this.setStatus(this.status || status, source, sourceEffect);
    // 	}
    //
    pub fn try_set_status(
        battle: &mut Battle,
        pokemon_pos: (usize, usize),
        status_id: ID,
        source_pos: Option<(usize, usize)>,
        source_effect: Option<&Effect>
    ) -> bool {
        // JS: return this.setStatus(this.status || status, source, sourceEffect);
        //
        // If already has a status, setStatus will be called with the current status
        // and should return false. Otherwise, setStatus is called with the new status.
        //
        // Note: The JavaScript version passes (this.status || status) which evaluates to
        // the current status if it exists, or the new status if not. Then setStatus
        // checks if it's the same status and fails. This is functionally equivalent to
        // checking if we already have a status and returning false.
        let has_status = {
            let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => return false,
            };
            !pokemon.status.is_empty()
        };

        if has_status {
            return false;
        }

        // Call setStatus which will handle all immunity checks and events
        Pokemon::set_status(battle, pokemon_pos, status_id, source_pos, source_effect, false)
    }
}
