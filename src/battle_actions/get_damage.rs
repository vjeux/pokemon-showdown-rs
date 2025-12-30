use crate::*;
use crate::battle_actions::DamageCalcParams;

impl<'a> BattleActions<'a> {

    // =========================================================================
    // DAMAGE CALCULATION HELPERS
    // =========================================================================

    /// Get damage from getDamage in battle-actions.ts
    /// This is a comprehensive damage calculation
    // TypeScript source:
    // /**
    // 	 * 0 is a success dealing 0 damage, such as from False Swipe at 1 HP.
    // 	 *
    // 	 * Normal PS return value rules apply:
    // 	 * undefined = success, null = silent failure, false = loud failure
    // 	 */
    // 	getDamage(
    // 		source: Pokemon, target: Pokemon, move: string | number | ActiveMove,
    // 		suppressMessages = false
    // 	): number | undefined | null | false {
    // 		if (typeof move === 'string') move = this.dex.getActiveMove(move);
    //
    // 		if (typeof move === 'number') {
    // 			const basePower = move;
    // 			move = new Dex.Move({
    // 				basePower,
    // 				type: '???',
    // 				category: 'Physical',
    // 				willCrit: false,
    // 			}) as ActiveMove;
    // 			move.hit = 0;
    // 		}
    //
    // 		if (!target.runImmunity(move, !suppressMessages)) {
    // 			return false;
    // 		}
    //
    // 		if (move.ohko) return this.battle.gen === 3 ? target.hp : target.maxhp;
    // 		if (move.damageCallback) return move.damageCallback.call(this.battle, source, target);
    // 		if (move.damage === 'level') {
    // 			return source.level;
    // 		} else if (move.damage) {
    // 			return move.damage;
    // 		}
    //
    // 		const category = this.battle.getCategory(move);
    //
    // 		let basePower: number | false | null = move.basePower;
    // 		if (move.basePowerCallback) {
    // 			basePower = move.basePowerCallback.call(this.battle, source, target, move);
    // 		}
    // 		if (!basePower) return basePower === 0 ? undefined : basePower;
    // 		basePower = this.battle.clampIntRange(basePower, 1);
    //
    // 		let critMult;
    // 		let critRatio = this.battle.runEvent('ModifyCritRatio', source, target, move, move.critRatio || 0);
    // 		if (this.battle.gen <= 5) {
    // 			critRatio = this.battle.clampIntRange(critRatio, 0, 5);
    // 			critMult = [0, 16, 8, 4, 3, 2];
    // 		} else {
    // 			critRatio = this.battle.clampIntRange(critRatio, 0, 4);
    // 			if (this.battle.gen === 6) {
    // 				critMult = [0, 16, 8, 2, 1];
    // 			} else {
    // 				critMult = [0, 24, 8, 2, 1];
    // 			}
    // 		}
    //
    // 		const moveHit = target.getMoveHitData(move);
    // 		moveHit.crit = move.willCrit || false;
    // 		if (move.willCrit === undefined) {
    // 			if (critRatio) {
    // 				moveHit.crit = this.battle.randomChance(1, critMult[critRatio]);
    // 			}
    // 		}
    //
    // 		if (moveHit.crit) {
    // 			moveHit.crit = this.battle.runEvent('CriticalHit', target, null, move);
    // 		}
    //
    // 		// happens after crit calculation
    // 		basePower = this.battle.runEvent('BasePower', source, target, move, basePower, true);
    //
    // 		if (!basePower) return 0;
    // 		basePower = this.battle.clampIntRange(basePower, 1);
    // 		// Hacked Max Moves have 0 base power, even if you Dynamax
    // 		if ((!source.volatiles['dynamax'] && move.isMax) || (move.isMax && this.dex.moves.get(move.baseMove).isMax)) {
    // 			basePower = 0;
    // 		}
    //
    // 		const dexMove = this.dex.moves.get(move.id);
    // 		if (source.terastallized && (source.terastallized === 'Stellar' ?
    // 			!source.stellarBoostedTypes.includes(move.type) : source.hasType(move.type)) &&
    // 			basePower < 60 && dexMove.priority <= 0 && !dexMove.multihit &&
    // 			// Hard move.basePower check for moves like Dragon Energy that have variable BP
    // 			!((move.basePower === 0 || move.basePower === 150) && move.basePowerCallback)
    // 		) {
    // 			basePower = 60;
    // 		}
    //
    // 		const level = source.level;
    //
    // 		const attacker = move.overrideOffensivePokemon === 'target' ? target : source;
    // 		const defender = move.overrideDefensivePokemon === 'source' ? source : target;
    //
    // 		const isPhysical = move.category === 'Physical';
    // 		let attackStat: StatIDExceptHP = move.overrideOffensiveStat || (isPhysical ? 'atk' : 'spa');
    // 		const defenseStat: StatIDExceptHP = move.overrideDefensiveStat || (isPhysical ? 'def' : 'spd');
    //
    // 		const statTable = { atk: 'Atk', def: 'Def', spa: 'SpA', spd: 'SpD', spe: 'Spe' };
    //
    // 		let atkBoosts = attacker.boosts[attackStat];
    // 		let defBoosts = defender.boosts[defenseStat];
    //
    // 		let ignoreNegativeOffensive = !!move.ignoreNegativeOffensive;
    // 		let ignorePositiveDefensive = !!move.ignorePositiveDefensive;
    //
    // 		if (moveHit.crit) {
    // 			ignoreNegativeOffensive = true;
    // 			ignorePositiveDefensive = true;
    // 		}
    // 		const ignoreOffensive = !!(move.ignoreOffensive || (ignoreNegativeOffensive && atkBoosts < 0));
    // 		const ignoreDefensive = !!(move.ignoreDefensive || (ignorePositiveDefensive && defBoosts > 0));
    //
    // 		if (ignoreOffensive) {
    // 			this.battle.debug('Negating (sp)atk boost/penalty.');
    // 			atkBoosts = 0;
    // 		}
    // 		if (ignoreDefensive) {
    // 			this.battle.debug('Negating (sp)def boost/penalty.');
    // 			defBoosts = 0;
    // 		}
    //
    // 		let attack = attacker.calculateStat(attackStat, atkBoosts, 1, source);
    // 		let defense = defender.calculateStat(defenseStat, defBoosts, 1, target);
    //
    // 		attackStat = (category === 'Physical' ? 'atk' : 'spa');
    //
    // 		// Apply Stat Modifiers
    // 		attack = this.battle.runEvent('Modify' + statTable[attackStat], source, target, move, attack);
    // 		defense = this.battle.runEvent('Modify' + statTable[defenseStat], target, source, move, defense);
    //
    // 		if (this.battle.gen <= 4 && ['explosion', 'selfdestruct'].includes(move.id) && defenseStat === 'def') {
    // 			defense = this.battle.clampIntRange(Math.floor(defense / 2), 1);
    // 		}
    //
    // 		const tr = this.battle.trunc;
    //
    // 		// int(int(int(2 * L / 5 + 2) * A * P / D) / 50);
    // 		const baseDamage = tr(tr(tr(tr(2 * level / 5 + 2) * basePower * attack) / defense) / 50);
    //
    // 		// Calculate damage modifiers separately (order differs between generations)
    // 		return this.modifyDamage(baseDamage, source, target, move, suppressMessages);
    // 	}
    //
    pub fn get_damage(params: DamageCalcParams) -> i32 {
        if params.base_power == 0 || params.type_effectiveness == 0.0 {
            return 0;
        }

        // Base damage formula
        let base_damage =
            ((2 * params.attacker_level / 5 + 2) * params.base_power * params.attacker_attack
                / params.defender_defense.max(1))
                / 50
                + 2;

        // Apply modifiers in order
        let mut damage = base_damage as f64;

        // STAB
        damage *= params.stab_modifier;

        // Type effectiveness
        damage *= params.type_effectiveness;

        // Critical hit
        if params.is_crit {
            damage *= 1.5;
        }

        // Random factor
        damage = damage * (params.random_factor as f64) / 100.0;

        // Other modifiers (abilities, items, weather, etc.)
        for modifier in params.other_modifiers {
            damage = (damage * modifier).floor();
        }

        (damage as i32).max(1)
    }
}
