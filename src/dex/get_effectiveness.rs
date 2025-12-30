use crate::*;

impl Dex {

    /// Get type effectiveness modifier
    /// Returns: 1 = super-effective, -1 = resist, 0 = neutral/immune
    /// Note: This returns a modifier value, not the actual damage multiplier
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
    pub fn get_effectiveness(&self, attack_type: &str, defend_type: &str) -> i32 {
        if let Some(type_data) = self.types().get(defend_type) {
            if let Some(&effectiveness) = type_data.damage_taken.get(attack_type) {
                return match effectiveness {
                    1 => 1,  // super-effective
                    2 => -1, // resist
                    _ => 0,  // default (neutral/immune)
                };
            }
        }
        0 // Default to 0
    }
}
