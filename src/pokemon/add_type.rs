use crate::*;

impl Pokemon {

    /// Add a type (for Forest's Curse, Trick-or-Treat)
    // TypeScript source:
    // /** Removes any types added previously and adds another one. */
    // 	addType(newType: string) {
    // 		if (this.terastallized) return false;
    // 		this.addedType = newType;
    // 		return true;
    // 	}
    //
    pub fn add_type(&mut self, new_type: String) -> bool {
        // JS: if (this.terastallized) return false;
        if self.terastallized.is_some() {
            return false;
        }
        // JS: this.addedType = newType;
        self.added_type = Some(new_type);
        // JS: return true;
        true
    }
}
