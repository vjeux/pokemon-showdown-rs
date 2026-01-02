use crate::*;

impl Pokemon {

    /// Update max HP (for forme changes)
    ///
    /// This is an associated function (not a method) because it needs
    /// mutable access to Battle while operating on a Pokemon within that Battle.
    /// Call as: Pokemon::update_max_hp(battle, pokemon_pos, new_base_max_hp)
    //
    // 	updateMaxHp() {
    // 		const newBaseMaxHp = this.battle.statModify(this.species.baseStats, this.set, 'hp');
    // 		if (newBaseMaxHp === this.baseMaxhp) return;
    // 		this.baseMaxhp = newBaseMaxHp;
    // 		const newMaxHP = this.volatiles['dynamax'] ? (2 * this.baseMaxhp) : this.baseMaxhp;
    // 		this.hp = this.hp <= 0 ? 0 : Math.max(1, newMaxHP - (this.maxhp - this.hp));
    // 		this.maxhp = newMaxHP;
    // 		if (this.hp) this.battle.add('-heal', this, this.getHealth, '[silent]');
    // 	}
    //
    pub fn update_max_hp(battle: &mut Battle, pokemon_pos: (usize, usize), new_base_max_hp: i32) {
        // JS: const newBaseMaxHp = this.battle.statModify(this.species.baseStats, this.set, 'hp');
        // Note: Takes new_base_max_hp as parameter instead of calculating from species
        // Note: JavaScript calculates it from species.baseStats and set

        // Phase 1: Extract data immutably
        let (base_maxhp, has_dynamax) = {
            let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => return,
            };
            (pokemon.base_maxhp, pokemon.has_volatile(&ID::new("dynamax")))
        };

        // JS: if (newBaseMaxHp === this.baseMaxhp) return;
        if new_base_max_hp == base_maxhp {
            return;
        }

        // Phase 2: Get mutable reference and update
        let pokemon = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return,
        };

        // JS: this.baseMaxhp = newBaseMaxHp;
        pokemon.base_maxhp = new_base_max_hp;

        // JS: const newMaxHP = this.volatiles['dynamax'] ? (2 * this.baseMaxhp) : this.baseMaxhp;
        let old_maxhp = pokemon.maxhp;
        let new_max_hp = if has_dynamax {
            2 * new_base_max_hp
        } else {
            new_base_max_hp
        };
        pokemon.maxhp = new_max_hp;

        // JS: this.hp = this.hp <= 0 ? 0 : Math.max(1, newMaxHP - (this.maxhp - this.hp));
        // Adjust current HP proportionally
        if pokemon.hp > 0 {
            let hp_lost = old_maxhp - pokemon.hp;
            pokemon.hp = new_max_hp.saturating_sub(hp_lost).max(1);
        }

        // JS: if (this.hp) this.battle.add('-heal', this, this.getHealth, '[silent]');
        // ✅ NOW IMPLEMENTED: battle.add('-heal') message
        if pokemon.hp > 0 {
            let health = pokemon.get_health();
            let ident = pokemon.get_slot();
            battle.add("-heal", &[ident.into(), health.into(), "[silent]".into()]);
        }
    }
}
