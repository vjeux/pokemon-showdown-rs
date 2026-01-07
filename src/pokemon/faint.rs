use crate::*;
use crate::battle::FaintData;

impl Pokemon {

    /// Mark Pokemon as fainted and queue faint
    /// Returns the amount of damage dealt (HP before faint)
    ///
    /// This is an associated function (not a method) because it needs
    /// access to Battle for the faint_queue.
    /// Call as: Pokemon::faint(battle, pokemon_pos, source_pos, effect)
    // TypeScript source:
    // /**
    // 	 * This function only puts the pokemon in the faint queue;
    // 	 * actually setting of this.fainted comes later when the
    // 	 * faint queue is resolved.
    // 	 *
    // 	 * Returns the amount of damage actually dealt
    // 	 */
    // 	faint(source: Pokemon | null = null, effect: Effect | null = null) {
    // 		if (this.fainted || this.faintQueued) return 0;
    // 		const d = this.hp;
    // 		this.hp = 0;
    // 		this.switchFlag = false;
    // 		this.faintQueued = true;
    // 		this.battle.faintQueue.push({
    // 			target: this,
    // 			source,
    // 			effect,
    // 		});
    // 		return d;
    // 	}
    //
    pub fn faint(
        battle: &mut Battle,
        pokemon_pos: (usize, usize),
        source_pos: Option<(usize, usize)>,
        effect: Option<&ID>
    ) -> i32 {
        // Phase 1: Check if already fainted/queued and get HP
        let damage = {
            let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => return 0,
            };

            // JS: if (this.fainted || this.faintQueued) return 0;
            if pokemon.fainted || pokemon.faint_queued {
                return 0;
            }

            // JS: const d = this.hp;
            pokemon.hp
        };

        // Phase 2: Mark as fainted mutably
        {
            let pokemon = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => return 0,
            };

            // JS: this.hp = 0;
            pokemon.hp = 0;

            // JS: this.switchFlag = false;
            pokemon.switch_flag = None;

            // JS: this.faintQueued = true;
            pokemon.faint_queued = true;
        }

        // JS: this.battle.faintQueue.push({
        // JS:     target: this,
        // JS:     source,
        // JS:     effect,
        // JS: });
        battle.faint_queue.push(FaintData {
            target: pokemon_pos,
            source: source_pos,
            effect: effect.cloned(),
        });

        // JS: return d;
        damage
    }
}
