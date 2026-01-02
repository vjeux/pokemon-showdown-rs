use crate::*;

impl Pokemon {
    /// Get updated details string for protocol
    //
    // 	getUpdatedDetails(level?: number) {
    // 		let name = this.species.name;
    // 		if (['Greninja-Bond', 'Rockruff-Dusk'].includes(name)) name = this.species.baseSpecies;
    // 		if (!level) level = this.level;
    // 		return name + (level === 100 ? '' : `, L${level}`) +
    // 			(this.gender === '' ? '' : `, ${this.gender}`) + (this.set.shiny ? ', shiny' : '');
    // 	}
    //
    pub fn get_updated_details(&self) -> String {
        // JS: let name = this.species.name;
        // JS: if (['Greninja-Bond', 'Rockruff-Dusk'].includes(name)) name = this.species.baseSpecies;
        let mut details = self.species_id.as_str().to_string();

        // Note: Greninja-Bond and Rockruff-Dusk special case not implemented
        // Would need species data to get baseSpecies

        // JS: return name + (level === 100 ? '' : `, L${level}`)
        if self.level != 100 {
            details.push_str(&format!(", L{}", self.level));
        }

        // JS: + (this.gender === '' ? '' : `, ${this.gender}`)
        if self.gender != Gender::None {
            details.push_str(&format!(", {}", self.gender.to_str()));
        }

        // JS: + (this.set.shiny ? ', shiny' : '');
        // ✅ NOW IMPLEMENTED: shiny flag from Pokemon struct
        if self.shiny {
            details.push_str(", shiny");
        }

        details
    }
}
