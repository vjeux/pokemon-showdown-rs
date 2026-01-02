use crate::*;

impl Pokemon {

    /// Set HP directly, returns delta
    ///
    /// This is an associated function (not a method) because it needs
    /// access to Battle for the trunc() method.
    /// Call as: Pokemon::set_hp(battle, pokemon_pos, target_hp)
    // TypeScript source:
    // /** Sets HP, returns delta */
    // 	sethp(d: number) {
    // 		if (!this.hp) return 0;
    // 		d = this.battle.trunc(d);
    // 		if (isNaN(d)) return;
    // 		if (d < 1) d = 1;
    // 		d -= this.hp;
    // 		this.hp += d;
    // 		if (this.hp > this.maxhp) {
    // 			d -= this.hp - this.maxhp;
    // 			this.hp = this.maxhp;
    // 		}
    // 		return d;
    // 	}
    //
    pub fn set_hp(battle: &mut Battle, pokemon_pos: (usize, usize), mut target_hp: i32) -> i32 {
        // Phase 1: Get current HP and maxhp
        let (current_hp, max_hp) = {
            let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => return 0,
            };

            // JS: if (!this.hp) return 0;
            if pokemon.hp == 0 {
                return 0;
            }

            (pokemon.hp, pokemon.maxhp)
        };

        // JS: d = this.battle.trunc(d);
        // ✅ NOW IMPLEMENTED: battle.trunc for integer truncation
        target_hp = battle.trunc(target_hp as f64, None) as i32;

        // JS: if (isNaN(d)) return;
        // Note: In Rust, we can't have NaN for i32, so this check is not needed

        // JS: if (d < 1) d = 1;
        if target_hp < 1 {
            target_hp = 1;
        }

        // JS: d -= this.hp;
        // Calculate delta
        let mut delta = target_hp - current_hp;

        // Phase 2: Apply delta
        let pokemon = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return 0,
        };

        // JS: this.hp += d;
        pokemon.hp += delta;

        // JS: if (this.hp > this.maxhp) {
        // JS:     d -= this.hp - this.maxhp;
        // JS:     this.hp = this.maxhp;
        // JS: }
        // ✅ NOW IMPLEMENTED: Overflow handling with delta adjustment
        if pokemon.hp > max_hp {
            delta -= pokemon.hp - max_hp;
            pokemon.hp = max_hp;
        }

        // JS: return d;
        delta
    }
}
