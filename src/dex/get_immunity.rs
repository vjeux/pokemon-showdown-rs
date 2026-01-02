// NOTE: This method is NOT in JavaScript - Rust-specific implementation

use crate::*;

impl Dex {

    /// Check if source type is immune to target
    /// Returns false if the target is immune; true otherwise.
    /// Equivalent to getImmunity() in dex.ts
    // TypeScript source:
    // /**
    // 	 * Returns false if the target is immune; true otherwise.
    // 	 * Also checks immunity to some statuses.
    // 	 */
    // 	getImmunity(
    // 		source: { type: string } | string,
    // 		target: { getTypes: () => string[] } | { types: string[] } | string[] | string
    // 	): boolean {
    // 		const sourceType: string = typeof source !== 'string' ? source.type : source;
    // 		// @ts-expect-error really wish TS would support this
    // 		const targetTyping: string[] | string = target.getTypes?.() || target.types || target;
    // 		if (Array.isArray(targetTyping)) {
    // 			for (const type of targetTyping) {
    // 				if (!this.getImmunity(sourceType, type)) return false;
    // 			}
    // 			return true;
    // 		}
    // 		const typeData = this.types.get(targetTyping);
    // 		if (typeData && typeData.damageTaken[sourceType] === 3) return false;
    // 		return true;
    // 	}
    //
    pub fn get_immunity(&self, source_type: &str, target_types: &[String]) -> bool {
        for target_type in target_types {
            // Check damage_taken directly for immunity (value === 3)
            if let Some(type_data) = self.types().get(target_type) {
                if let Some(&damage_taken) = type_data.damage_taken.get(source_type) {
                    if damage_taken == 3 {
                        return false; // Immune
                    }
                }
            }
        }
        true
    }
}
