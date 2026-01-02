use crate::*;
use crate::event::EventResult;

impl Pokemon {
    /// Add a volatile condition to a Pokemon
    // addVolatile(
    //     status: string | Condition, source: Pokemon | null = null, sourceEffect: Effect | null = null,
    //     linkedStatus: string | Condition | null = null
    // ): boolean | any {
    //     let result;
    //     status = this.battle.dex.conditions.get(status);
    //     if (!this.hp && !status.affectsFainted) return false;
    //     if (linkedStatus && source && !source.hp) return false;
    //     if (this.battle.event) {
    //         if (!source) source = this.battle.event.source;
    //         if (!sourceEffect) sourceEffect = this.battle.effect;
    //     }
    //     if (!source) source = this;

    //     if (this.volatiles[status.id]) {
    //         if (!status.onRestart) return false;
    //         return this.battle.singleEvent('Restart', status, this.volatiles[status.id], this, source, sourceEffect);
    //     }
    //     if (!this.runStatusImmunity(status.id)) {
    //         this.battle.debug('immune to volatile status');
    //         if ((sourceEffect as Move)?.status) {
    //             this.battle.add('-immune', this);
    //         }
    //         return false;
    //     }
    //     result = this.battle.runEvent('TryAddVolatile', this, source, sourceEffect, status);
    //     if (!result) {
    //         this.battle.debug('add volatile [' + status.id + '] interrupted');
    //         return result;
    //     }
    //     this.volatiles[status.id] = this.battle.initEffectState({ id: status.id, name: status.name, target: this });
    //     if (source) {
    //         this.volatiles[status.id].source = source;
    //         this.volatiles[status.id].sourceSlot = source.getSlot();
    //     }
    //     if (sourceEffect) this.volatiles[status.id].sourceEffect = sourceEffect;
    //     if (status.duration) this.volatiles[status.id].duration = status.duration;
    //     if (status.durationCallback) {
    //         this.volatiles[status.id].duration = status.durationCallback.call(this.battle, this, source, sourceEffect);
    //     }
    //     result = this.battle.singleEvent('Start', status, this.volatiles[status.id], this, source, sourceEffect);
    //     if (!result) {
    //         // cancel
    //         delete this.volatiles[status.id];
    //         return result;
    //     }
    //     if (linkedStatus && source) {
    //         if (!source.volatiles[linkedStatus.toString()]) {
    //             source.addVolatile(linkedStatus, this, sourceEffect);
    //             source.volatiles[linkedStatus.toString()].linkedPokemon = [this];
    //             source.volatiles[linkedStatus.toString()].linkedStatus = status;
    //         } else {
    //             source.volatiles[linkedStatus.toString()].linkedPokemon.push(this);
    //         }
    //         this.volatiles[status.toString()].linkedPokemon = [source];
    //         this.volatiles[status.toString()].linkedStatus = linkedStatus;
    //     }
    //     return true;
    // }
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
        // JS: status = this.battle.dex.conditions.get(status);
        // JS: if (!this.hp && !status.affectsFainted) return false;
        // Note: Missing HP check with affectsFainted flag - would need condition data access

        // JS: if (linkedStatus && source && !source.hp) return false;
        // Note: Missing linkedStatus parameter and source HP check

        // JS: if (this.battle.event) {
        // JS:     if (!source) source = this.battle.event.source;
        // JS:     if (!sourceEffect) sourceEffect = this.battle.effect;
        // JS: }
        // JS: if (!source) source = this;
        // Note: Missing battle.event source/sourceEffect defaulting

        // Check if pokemon already has this volatile
        {
            let pokemon = match battle.pokemon_at(target_pos.0, target_pos.1) {
                Some(p) => p,
                None => return false,
            };

            // JS: if (this.volatiles[status.id]) {
            // JS:     if (!status.onRestart) return false;
            // JS:     return this.battle.singleEvent('Restart', status, this.volatiles[status.id], this, source, sourceEffect);
            // JS: }
            if pokemon.volatiles.contains_key(&volatile_id) {
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

        // JS: if (!this.runStatusImmunity(status.id)) {
        // JS:     this.battle.debug('immune to volatile status');
        // JS:     if ((sourceEffect as Move)?.status) {
        // JS:         this.battle.add('-immune', this);
        // JS:     }
        // JS:     return false;
        // JS: }
        // Note: Missing runStatusImmunity check for volatile

        // JS: result = this.battle.runEvent('TryAddVolatile', this, source, sourceEffect, status);
        // JS: if (!result) {
        // JS:     this.battle.debug('add volatile [' + status.id + '] interrupted');
        // JS:     return result;
        // JS: }
        // Note: Missing runEvent('TryAddVolatile')

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

        // JS: this.volatiles[status.id] = this.battle.initEffectState({ id: status.id, name: status.name, target: this });
        // JS: if (source) {
        // JS:     this.volatiles[status.id].source = source;
        // JS:     this.volatiles[status.id].sourceSlot = source.getSlot();
        // JS: }
        // JS: if (sourceEffect) this.volatiles[status.id].sourceEffect = sourceEffect;
        // Note: Missing source, sourceSlot, sourceEffect assignments to EffectState
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
                // JS: if (linkedStatus && source) {
                // JS:     if (!source.volatiles[linkedStatus.toString()]) {
                // JS:         source.addVolatile(linkedStatus, this, sourceEffect);
                // JS:         source.volatiles[linkedStatus.toString()].linkedPokemon = [this];
                // JS:         source.volatiles[linkedStatus.toString()].linkedStatus = status;
                // JS:     } else {
                // JS:         source.volatiles[linkedStatus.toString()].linkedPokemon.push(this);
                // JS:     }
                // JS:     this.volatiles[status.toString()].linkedPokemon = [source];
                // JS:     this.volatiles[status.toString()].linkedStatus = linkedStatus;
                // JS: }
                // Note: Missing linkedStatus bidirectional linking (Leech Seed, etc.)
                return true;
            }
        }
    }
}
