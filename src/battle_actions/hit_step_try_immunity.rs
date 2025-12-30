use crate::*;
use crate::data::typechart::get_effectiveness_multi;

impl<'a> BattleActions<'a> {

    /// Hit step try immunity
    /// Equivalent to hitStepTryImmunity in battle-actions.ts
    // 	hitStepTryImmunity(targets: Pokemon[], pokemon: Pokemon, move: ActiveMove) {
    // 		const hitResults = [];
    // 		for (const [i, target] of targets.entries()) {
    // 			if (this.battle.gen >= 6 && move.flags['powder'] && target !== pokemon && !this.dex.getImmunity('powder', target)) {
    // 				this.battle.debug('natural powder immunity');
    // 				this.battle.add('-immune', target);
    // 				hitResults[i] = false;
    // 			} else if (!this.battle.singleEvent('TryImmunity', move, {}, target, pokemon, move)) {
    // 				this.battle.add('-immune', target);
    // 				hitResults[i] = false;
    // 			} else if (this.battle.gen >= 7 && move.pranksterBoosted && pokemon.hasAbility('prankster') &&
    // 				!targets[i].isAlly(pokemon) && !this.dex.getImmunity('prankster', target)) {
    // 				this.battle.debug('natural prankster immunity');
    // 				if (target.illusion || !(move.status && !this.dex.getImmunity(move.status, target))) {
    // 					this.battle.hint("Since gen 7, Dark is immune to Prankster moves.");
    // 				}
    // 				this.battle.add('-immune', target);
    // 				hitResults[i] = false;
    // 			} else {
    // 				hitResults[i] = true;
    // 			}
    // 		}
    // 		return hitResults;
    // 	}
    //
    pub fn hit_step_try_immunity(
        move_type: &str,
        defender_types: &[String],
        defender_ability: &str,
        ignore_immunity: bool,
    ) -> bool {
        if ignore_immunity {
            return true;
        }

        // Check type immunity
        let effectiveness = get_effectiveness_multi(move_type, defender_types);
        if effectiveness == 0.0 {
            return false;
        }

        // Check ability immunity (simplified)
        !matches!(
            (defender_ability, move_type),
            ("voltabsorb" | "lightningrod" | "motordrive", "Electric")
                | ("waterabsorb" | "stormdrain" | "dryskin", "Water")
                | ("flashfire", "Fire")
                | ("sapsipper", "Grass")
                | ("levitate", "Ground")
        )
    }
}
