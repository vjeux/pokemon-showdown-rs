use crate::*;
use crate::battle_actions::AccuracyCheckParams;

impl<'a> BattleActions<'a> {

    /// Check accuracy for move hit
    /// Equivalent to hitStepAccuracy in battle-actions.ts
    // 	hitStepAccuracy(targets: Pokemon[], pokemon: Pokemon, move: ActiveMove) {
    // 		const hitResults = [];
    // 		for (const [i, target] of targets.entries()) {
    // 			this.battle.activeTarget = target;
    // 			// calculate true accuracy
    // 			let accuracy = move.accuracy;
    // 			if (move.ohko) { // bypasses accuracy modifiers
    // 				if (!target.isSemiInvulnerable()) {
    // 					accuracy = 30;
    // 					if (move.ohko === 'Ice' && this.battle.gen >= 7 && !pokemon.hasType('Ice')) {
    // 						accuracy = 20;
    // 					}
    // 					if (!target.volatiles['dynamax'] && pokemon.level >= target.level &&
    // 						(move.ohko === true || !target.hasType(move.ohko))) {
    // 						accuracy += (pokemon.level - target.level);
    // 					} else {
    // 						this.battle.add('-immune', target, '[ohko]');
    // 						hitResults[i] = false;
    // 						continue;
    // 					}
    // 				}
    // 			} else {
    // 				accuracy = this.battle.runEvent('ModifyAccuracy', target, pokemon, move, accuracy);
    // 				if (accuracy !== true) {
    // 					let boost = 0;
    // 					if (!move.ignoreAccuracy) {
    // 						const boosts = this.battle.runEvent('ModifyBoost', pokemon, null, null, { ...pokemon.boosts });
    // 						boost = this.battle.clampIntRange(boosts['accuracy'], -6, 6);
    // 					}
    // 					if (!move.ignoreEvasion) {
    // 						const boosts = this.battle.runEvent('ModifyBoost', target, null, null, { ...target.boosts });
    // 						boost = this.battle.clampIntRange(boost - boosts['evasion'], -6, 6);
    // 					}
    // 					if (boost > 0) {
    // 						accuracy = this.battle.trunc(accuracy * (3 + boost) / 3);
    // 					} else if (boost < 0) {
    // 						accuracy = this.battle.trunc(accuracy * 3 / (3 - boost));
    // 					}
    // 				}
    // 			}
    // 			if (
    // 				move.alwaysHit || (move.id === 'toxic' && this.battle.gen >= 8 && pokemon.hasType('Poison')) ||
    // 				(move.target === 'self' && move.category === 'Status' && !target.isSemiInvulnerable())
    // 			) {
    // 				accuracy = true; // bypasses ohko accuracy modifiers
    // 			} else {
    // 				accuracy = this.battle.runEvent('Accuracy', target, pokemon, move, accuracy);
    // 			}
    // 			if (accuracy !== true && !this.battle.randomChance(accuracy, 100)) {
    // 				if (move.smartTarget) {
    // 					move.smartTarget = false;
    // 				} else {
    // 					if (!move.spreadHit) this.battle.attrLastMove('[miss]');
    // 					this.battle.add('-miss', pokemon, target);
    // 				}
    // 				if (!move.ohko && pokemon.hasItem('blunderpolicy') && pokemon.useItem()) {
    // 					this.battle.boost({ spe: 2 }, pokemon);
    // 				}
    // 				hitResults[i] = false;
    // 				continue;
    // 			}
    // 			hitResults[i] = true;
    // 		}
    // 		return hitResults;
    // 	}
    //
    pub fn hit_step_accuracy(params: AccuracyCheckParams) -> bool {
        // Always hit moves
        if params.move_always_hit || params.move_accuracy == 0 {
            return true;
        }

        // Calculate effective accuracy
        let mut accuracy_stages = if params.ignore_accuracy {
            0
        } else {
            params.attacker_accuracy_boost
        };
        let evasion_stages = if params.ignore_evasion {
            0
        } else {
            params.defender_evasion_boost
        };
        accuracy_stages -= evasion_stages;

        // Apply stage modifier
        let (num, denom) = Self::get_accuracy_modifier(accuracy_stages);
        let effective_accuracy = params.move_accuracy * num / denom;

        // OHKO moves have special accuracy handling
        if params.move_ohko {
            return params.random_value < effective_accuracy.min(100);
        }

        params.random_value < effective_accuracy.min(100)
    }
}
