use crate::*;

impl Dex {

    /// Check if source type is immune to target
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
            if self.get_effectiveness(source_type, target_type) == 0.0 {
                return false;
            }
        }
        true
    }
}
