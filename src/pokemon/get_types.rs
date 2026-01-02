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
    pub fn get_types(&self, battle: &Battle, exclude_added: bool) -> Vec<String> {
        self.get_types_full(battle, exclude_added, false)
    }

    pub fn get_types_full(&self, battle: &Battle, exclude_added: bool, preterastallized: bool) -> Vec<String> {
        // JS: if (!preterastallized && this.terastallized && this.terastallized !== 'Stellar') return [this.terastallized];
        if !preterastallized {
            if let Some(ref tera) = self.terastallized {
                if tera != "Stellar" {
                    return vec![tera.clone()];
                }
            }
        }

        // JS: const types = this.battle.runEvent('Type', this, null, null, this.types);
        // Note: runEvent('Type') not called - would need event system infrastructure
        let mut types = self.types.clone();

        // JS: if (!types.length) types.push(this.battle.gen >= 5 ? 'Normal' : '???');
        // ✅ NOW IMPLEMENTED: Gen check for default type
        if types.is_empty() {
            if battle.gen >= 5 {
                types.push("Normal".to_string());
            } else {
                types.push("???".to_string());
            }
        }

        // JS: if (!excludeAdded && this.addedType) return types.concat(this.addedType);
        // JavaScript: addedType is string, check if not empty
        if !exclude_added && !self.added_type.is_empty() {
            if !types.contains(&self.added_type) {
                types.push(self.added_type.clone());
            }
        }
        types
    }
}
