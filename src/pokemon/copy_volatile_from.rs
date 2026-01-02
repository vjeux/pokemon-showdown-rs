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
        // TODO: implement the same logic as JavaScript
        // We should not hardcode any conditions that's not in the original source.
        
        match copy_type {
            "copyvolatile" | "batonpass" => {
                // Copy stat boosts
                self.boosts = source.boosts;

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
            }
            "shedtail" => {
                // Shed Tail only copies the substitute
                let sub_id = ID::new("substitute");
                if source.has_volatile(&sub_id) {
                    if let Some(state) = source.get_volatile(&sub_id) {
                        self.volatiles.insert(sub_id, state.clone());
                    }
                }
            }
            _ => {}
        }
    }
}
