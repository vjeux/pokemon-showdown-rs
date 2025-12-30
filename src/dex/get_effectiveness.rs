use crate::*;

impl Dex {

    /// Get type effectiveness multiplier
    /// 0 = immune (0x), 1 = not very effective (0.5x), 2 = neutral (1x), 3 = super effective (2x)
    /// Returns the numeric multiplier
    // TypeScript source:
    //
    //
    // 	getEffectiveness(
    // 		source: { type: string } | string,
    // 		target: { getTypes: () => string[] } | { types: string[] } | string[] | string
    // 	): number {
    // 		const sourceType: string = typeof source !== 'string' ? source.type : source;
    // 		// @ts-expect-error really wish TS would support this
    // 		const targetTyping: string[] | string = target.getTypes?.() || target.types || target;
    // 		let totalTypeMod = 0;
    // 		if (Array.isArray(targetTyping)) {
    // 			for (const type of targetTyping) {
    // 				totalTypeMod += this.getEffectiveness(sourceType, type);
    // 			}
    // 			return totalTypeMod;
    // 		}
    // 		const typeData = this.types.get(targetTyping);
    // 		if (!typeData) return 0;
    // 		switch (typeData.damageTaken[sourceType]) {
    // 		case 1: return 1; // super-effective
    // 		case 2: return -1; // resist
    // 		// in case of weird situations like Gravity, immunity is handled elsewhere
    // 		default: return 0;
    // 		}
    // 	}
    //
    pub fn get_effectiveness(&self, attack_type: &str, defend_type: &str) -> f64 {
        if let Some(type_data) = self.types().get(defend_type) {
            if let Some(&effectiveness) = type_data.damage_taken.get(attack_type) {
                return match effectiveness {
                    0 => 1.0, // Neutral
                    1 => 2.0, // Super effective
                    2 => 0.5, // Not very effective
                    3 => 0.0, // Immune
                    _ => 1.0,
                };
            }
        }
        1.0 // Default to neutral
    }
}
