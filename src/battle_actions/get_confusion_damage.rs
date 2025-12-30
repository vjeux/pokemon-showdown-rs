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
    /// Note: This is a static function that takes pre-calculated stats and a randomizer result
    /// The actual Battle method would call this with values from pokemon.calculateStat() and battle.randomizer()
    pub fn get_confusion_damage(
        level: i32,
        attack: i32,
        defense: i32,
        base_power: i32,
        random_factor: i32,
    ) -> i32 {
        // JS: const baseDamage = tr(tr(tr(tr(2 * level / 5 + 2) * basePower * attack) / defense) / 50) + 2;
        // We need to truncate at each step to match JavaScript
        // Note: Using integer division in Rust already truncates, matching trunc() for positive integers
        let step1 = (2 * level / 5 + 2) as u32; // tr(2 * level / 5 + 2)
        let step2 = (step1 * base_power as u32) as u32; // tr(step1 * basePower)
        let step3 = (step2 * attack as u32) as u32; // tr(step2 * attack)
        let step4 = (step3 / defense.max(1) as u32) as u32; // tr(step3 / defense)
        let base_damage = (step4 / 50) as u32 + 2; // tr(step4 / 50) + 2

        // JS: let damage = tr(baseDamage, 16);
        // 16-bit truncation: (baseDamage >>> 0) % (2 ** 16) = baseDamage % 65536
        let damage = crate::dex::Dex::trunc(base_damage as f64, 16);

        // JS: damage = this.battle.randomizer(damage);
        // randomizer applies (100 - random(16)) / 100, giving 0.85-1.0 multiplier
        // Here we assume the caller already applied randomizer and passed the result as random_factor
        // Actually, looking closer, random_factor is the final multiplied result already
        // So we just use the damage with the random factor that was passed in
        let final_damage = crate::dex::Dex::trunc(damage as f64 * random_factor as f64 / 100.0, 0);

        // JS: return Math.max(1, damage);
        final_damage.max(1) as i32
    }
}
