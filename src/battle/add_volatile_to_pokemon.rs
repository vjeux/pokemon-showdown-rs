use crate::*;

impl Battle {
    /// Add a volatile condition to a Pokemon
    /// This is a wrapper around Pokemon::add_volatile that handles duration callbacks
    /// JavaScript equivalent: pokemon.addVolatile(status, source, sourceEffect)
    pub fn add_volatile_to_pokemon(
        &mut self,
        pokemon_pos: (usize, usize),
        volatile_id: ID,
        source_pos: Option<(usize, usize)>,
    ) -> bool {
        // Check if pokemon already has this volatile
        {
            let pokemon = match self.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => return false,
            };

            if pokemon.volatiles.contains_key(&volatile_id) {
                return false;
            }
        }

        // Get default duration from dex.conditions
        // JS: if (status.duration) this.volatiles[status.id].duration = status.duration;
        let default_duration = self.dex.conditions.get(&volatile_id)
            .and_then(|cond| cond.duration);

        // Check if condition has a durationCallback
        // JS: if (status.durationCallback) {
        //     this.volatiles[status.id].duration = status.durationCallback.call(this.battle, this, source, sourceEffect);
        // }
        let callback_duration = crate::data::duration_callbacks::dispatch_duration_callback(
            self,
            volatile_id.as_str(),
            pokemon_pos,
            source_pos,
        );

        // durationCallback overrides default duration
        let final_duration = callback_duration.or(default_duration);

        // Add the volatile
        let pokemon_mut = match self.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return false,
        };

        // Create effect state with duration
        let mut state = crate::event_system::EffectState::new(volatile_id.clone());
        state.duration = final_duration;

        pokemon_mut.volatiles.insert(volatile_id, state);
        true
    }
}
