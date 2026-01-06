use crate::*;

impl Pokemon {

    /// Check if Pokemon has a specific type
    //
    // 	hasType(type: string | string[]) {
    // 		const thisTypes = this.getTypes();
    // 		if (typeof type === 'string') {
    // 			return thisTypes.includes(type);
    // 		}
    //
    // 		for (const typeName of type) {
    // 			if (thisTypes.includes(typeName)) return true;
    // 		}
    // 		return false;
    // 	}
    //
    pub fn has_type(&self, battle: &Battle, type_name: &str) -> bool {
        // JS: const thisTypes = this.getTypes();
        // JS: return thisTypes.includes(type);
        // Note: Type comparison should be case-insensitive
        let type_name_lower = type_name.to_lowercase();
        self.get_types(battle, false)
            .iter()
            .any(|t| t.to_lowercase() == type_name_lower)
    }
}
