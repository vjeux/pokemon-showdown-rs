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
        // JS: if (!preterastallized && this.terastallized && this.terastallized !== 'Stellar') return [this.terastallized];
        if let Some(ref tera) = self.terastallized {
            if tera != "Stellar" {
                return vec![tera.clone()];
            }
        }

        // JS: const types = this.battle.runEvent('Type', this, null, null, this.types);
        // Note: runEvent('Type') not called - would need Battle reference
        let mut types = self.types.clone();

        // JS: if (!types.length) types.push(this.battle.gen >= 5 ? 'Normal' : '???');
        // Note: Gen check not implemented - would need Battle reference for gen
        if types.is_empty() {
            types.push("Normal".to_string()); // Assumes gen >= 5
        }

        // JS: if (!excludeAdded && this.addedType) return types.concat(this.addedType);
        if !exclude_added {
            if let Some(ref added) = self.added_type {
                if !types.contains(added) {
                    types.push(added.clone());
                }
            }
        }
        types
    }
}
