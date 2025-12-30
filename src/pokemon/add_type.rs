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
    pub fn add_type(&mut self, new_type: String) {
        if !self.types.contains(&new_type) {
            self.added_type = Some(new_type);
        }
    }
}
