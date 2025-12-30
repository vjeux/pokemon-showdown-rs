use crate::*;

impl<'a> BattleActions<'a> {

    /// Calculate confusion damage
    /// Equivalent to getConfusionDamage in battle-actions.ts
    // TypeScript source:
    // /**
    // 	 * Confusion damage is unique - most typical modifiers that get run when calculating
    // 	 * damage (e.g. Huge Power, Life Orb, critical hits) don't apply. It also uses a 16-bit
    // 	 * context for its damage, unlike the regular damage formula (though this only comes up
    // 	 * for base damage).
    // 	 */
    // 	getConfusionDamage(pokemon: Pokemon, basePower: number) {
    // 		const tr = this.battle.trunc;
    //
    // 		const attack = pokemon.calculateStat('atk', pokemon.boosts['atk']);
    // 		const defense = pokemon.calculateStat('def', pokemon.boosts['def']);
    // 		const level = pokemon.level;
    // 		const baseDamage = tr(tr(tr(tr(2 * level / 5 + 2) * basePower * attack) / defense) / 50) + 2;
    //
    // 		// Damage is 16-bit context in self-hit confusion damage
    // 		let damage = tr(baseDamage, 16);
    // 		damage = this.battle.randomizer(damage);
    // 		return Math.max(1, damage);
    // 	}
    //
    pub fn get_confusion_damage(
        level: i32,
        attack: i32,
        defense: i32,
        base_power: i32,
        random_factor: i32,
    ) -> i32 {
        // int(int(int(2 * L / 5 + 2) * P * A / D) / 50) + 2
        let base_damage = ((2 * level / 5 + 2) * base_power * attack / defense) / 50 + 2;

        // Apply random factor (0.85 to 1.0)
        let damage = base_damage * random_factor / 100;

        damage.max(1)
    }
}
