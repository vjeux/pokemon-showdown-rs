use crate::*;

impl Pokemon {
    /// Add a volatile condition to a Pokemon
    /// JavaScript equivalent: pokemon.addVolatile(status, source, sourceEffect)
    ///
    /// In Rust, this is an associated function (not a method) because it needs
    /// mutable access to Battle while operating on a Pokemon within that Battle.
    /// Call as: Pokemon::add_volatile(battle, target_pos, volatile_id, source_pos)
    pub fn add_volatile(
        battle: &mut Battle,
        target_pos: (usize, usize),
        volatile_id: ID,
        source_pos: Option<(usize, usize)>,
    ) -> bool {
        // Check if pokemon already has this volatile
        {
            let pokemon = match battle.pokemon_at(target_pos.0, target_pos.1) {
                Some(p) => p,
                None => return false,
            };

            if pokemon.volatiles.contains_key(&volatile_id) {
                // JavaScript: if (pokemon.volatiles[status.id]) {
                //     if (!status.onRestart) return false;
                //     return this.singleEvent('Restart', status, this.volatiles[status.id], target, source, sourceEffect);
                // }
                // Call onRestart callback if the volatile already exists
                let restart_result = crate::data::condition_callbacks::dispatch_on_restart(
                    battle,
                    volatile_id.as_str(),
                    target_pos,
                );

                // If onRestart returns false or Continue, return false (volatile not re-added)
                use crate::event::EventResult;
                match restart_result {
                    EventResult::Boolean(false) | EventResult::Continue => return false,
                    _ => return true,
                }
            }
        }

        // Get default duration from dex.conditions
        // JS: if (status.duration) this.volatiles[status.id].duration = status.duration;
        let default_duration = battle.dex.conditions.get(&volatile_id)
            .and_then(|cond| cond.duration);

        // Check if condition has a durationCallback
        // JS: if (status.durationCallback) {
        //     this.volatiles[status.id].duration = status.durationCallback.call(this.battle, this, source, sourceEffect);
        // }
        let callback_duration = crate::data::duration_callbacks::dispatch_duration_callback(
            battle,
            volatile_id.as_str(),
            target_pos,
            source_pos,
        );

        // durationCallback overrides default duration
        let final_duration = callback_duration.or(default_duration);

        // Add the volatile
        let pokemon_mut = match battle.pokemon_at_mut(target_pos.0, target_pos.1) {
            Some(p) => p,
            None => return false,
        };

        // Create effect state with duration
        let mut state = crate::event_system::EffectState::new(volatile_id.clone());
        state.duration = final_duration;

        pokemon_mut.volatiles.insert(volatile_id.clone(), state);

        // JavaScript: result = this.battle.singleEvent("Start", status, this.volatiles[status.id], this, source, sourceEffect);
        // Call the Start event for the newly added volatile
        let start_result = battle.single_event(
            "Start",
            &volatile_id,
            Some(target_pos),
            source_pos,
            None,
        );

        // JavaScript: if (!result) { delete this.volatiles[status.id]; return result; }
        // If Start event returns false/failure, remove the volatile
        use crate::event::EventResult;
        match start_result {
            EventResult::Boolean(false) => {
                // Start event failed, remove the volatile
                let pokemon_mut = match battle.pokemon_at_mut(target_pos.0, target_pos.1) {
                    Some(p) => p,
                    None => return false,
                };
                pokemon_mut.volatiles.remove(&volatile_id);
                return false;
            }
            _ => {
                // Start event succeeded or returned non-Boolean
                return true;
            }
        }
    }
}
