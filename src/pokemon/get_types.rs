use crate::*;

impl Pokemon {

    /// Get the types of this Pokemon
    //
    // 	getTypes(excludeAdded?: boolean, preterastallized?: boolean): string[] {
    // 		if (!preterastallized && this.terastallized && this.terastallized !== 'Stellar') {
    // 			return [this.terastallized];
    // 		}
    // 		const types = this.battle.runEvent('Type', this, null, null, this.types);
    // 		if (!types.length) types.push(this.battle.gen >= 5 ? 'Normal' : '???');
    // 		if (!excludeAdded && this.addedType) return types.concat(this.addedType);
    // 		return types;
    // 	}
    //
    pub fn get_types(&self, exclude_added: bool) -> Vec<String> {
        let mut types = self.types.clone();
        if !exclude_added {
            if let Some(ref added) = self.added_type {
                if !types.contains(added) {
                    types.push(added.clone());
                }
            }
        }
        // Handle Terastallization
        if let Some(ref tera) = self.terastallized {
            return vec![tera.clone()];
        }
        types
    }
}
