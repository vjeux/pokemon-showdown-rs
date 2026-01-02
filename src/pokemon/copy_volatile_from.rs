use crate::*;

impl Pokemon {

    /// Copy volatiles from another Pokemon (for Baton Pass, etc.)
    //
    // 	copyVolatileFrom(pokemon: Pokemon, switchCause?: string | boolean) {
    // 		this.clearVolatile();
    // 		if (switchCause !== 'shedtail') this.boosts = pokemon.boosts;
    // 		for (const i in pokemon.volatiles) {
    // 			if (switchCause === 'shedtail' && i !== 'substitute') continue;
    // 			if (this.battle.dex.conditions.getByID(i as ID).noCopy) continue;
    // 			// shallow clones
    // 			this.volatiles[i] = this.battle.initEffectState({ ...pokemon.volatiles[i], target: this });
    // 			if (this.volatiles[i].linkedPokemon) {
    // 				delete pokemon.volatiles[i].linkedPokemon;
    // 				delete pokemon.volatiles[i].linkedStatus;
    // 				for (const linkedPoke of this.volatiles[i].linkedPokemon) {
    // 					const linkedPokeLinks = linkedPoke.volatiles[this.volatiles[i].linkedStatus].linkedPokemon;
    // 					linkedPokeLinks[linkedPokeLinks.indexOf(pokemon)] = this;
    // 				}
    // 			}
    // 		}
    // 		pokemon.clearVolatile();
    // 		for (const i in this.volatiles) {
    // 			const volatile = this.getVolatile(i) as Condition;
    // 			this.battle.singleEvent('Copy', volatile, this.volatiles[i], this);
    // 		}
    // 	}
    //
    pub fn copy_volatile_from(&mut self, source: &Pokemon, copy_type: &str) {
        // JS: this.clearVolatile();
        // Note: Missing clearVolatile() call at start

        // JS: if (switchCause !== 'shedtail') this.boosts = pokemon.boosts;
        // ✅ NOW FIXED: Only copy boosts if NOT shedtail (was incorrectly copying for shedtail too)

        match copy_type {
            "copyvolatile" | "batonpass" => {
                // Copy stat boosts (NOT for shedtail)
                self.boosts = source.boosts;

                // JS: for (const i in pokemon.volatiles) {
                // JS:     if (switchCause === 'shedtail' && i !== 'substitute') continue;
                // JS:     if (this.battle.dex.conditions.getByID(i as ID).noCopy) continue;
                // JS:     this.volatiles[i] = this.battle.initEffectState({ ...pokemon.volatiles[i], target: this });
                // JS:     if (this.volatiles[i].linkedPokemon) {
                // JS:         delete pokemon.volatiles[i].linkedPokemon;
                // JS:         delete pokemon.volatiles[i].linkedStatus;
                // JS:         for (const linkedPoke of this.volatiles[i].linkedPokemon) {
                // JS:             const linkedPokeLinks = linkedPoke.volatiles[this.volatiles[i].linkedStatus].linkedPokemon;
                // JS:             linkedPokeLinks[linkedPokeLinks.indexOf(pokemon)] = this;
                // JS:         }
                // JS:     }
                // JS: }
                // Note: Hardcoded copyable list instead of using noCopy flag from condition data
                // Should loop through all volatiles and check if condition.noCopy is set
                // Copy certain volatiles
                let copyable = [
                    "aquaring",
                    "confusion",
                    "curse",
                    "embargo",
                    "focusenergy",
                    "gmaxchistrike",
                    "healblock",
                    "ingrain",
                    "laserfocus",
                    "leechseed",
                    "magnetrise",
                    "perishsong",
                    "powertrick",
                    "substitute",
                    "telekinesis",
                    "torment",
                ];

                for volatile_id in &copyable {
                    let id = ID::new(volatile_id);
                    if source.has_volatile(&id) {
                        if let Some(state) = source.get_volatile(&id) {
                            self.volatiles.insert(id, state.clone());
                        }
                    }
                }
                // Note: Missing linkedPokemon bidirectional link updating
            }
            "shedtail" => {
                // Shed Tail only copies the substitute, NOT boosts
                // JS: if (switchCause !== 'shedtail') this.boosts = pokemon.boosts;
                // ✅ NOW FIXED: Do NOT copy boosts for shedtail

                let sub_id = ID::new("substitute");
                if source.has_volatile(&sub_id) {
                    if let Some(state) = source.get_volatile(&sub_id) {
                        self.volatiles.insert(sub_id, state.clone());
                    }
                }
            }
            _ => {}
        }

        // JS: pokemon.clearVolatile();
        // Note: Missing source.clearVolatile() call - would need &mut source

        // JS: for (const i in this.volatiles) {
        // JS:     const volatile = this.getVolatile(i) as Condition;
        // JS:     this.battle.singleEvent('Copy', volatile, this.volatiles[i], this);
        // JS: }
        // Note: Missing singleEvent('Copy') calls for each copied volatile
    }
}
