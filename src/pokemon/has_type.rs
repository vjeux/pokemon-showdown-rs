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
    pub fn has_type(&self, type_name: &str) -> bool {
        self.get_types(false)
            .iter()
            .any(|t| t.to_lowercase() == type_name.to_lowercase())
    }
}
