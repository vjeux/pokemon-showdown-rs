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
        // ✅ NOW IMPLEMENTED: Greninja-Bond (Greninja-Ash) and Rockruff-Dusk use baseSpecies
        // Note: JavaScript uses species.name ('Greninja-Bond'), Rust uses species_id ('greninjabond'/'greninjaash')
        let species_str = self.species_id.as_str();
        let name = if species_str == "greninjabond" || species_str == "greninjaash" || species_str == "rockruffdusk" {
            self.base_species.as_str()
        } else {
            species_str
        };

        let mut details = name.to_string();

        // JS: return name + (level === 100 ? '' : `, L${level}`)
        if self.level != 100 {
            details.push_str(&format!(", L{}", self.level));
        }

        // JS: + (this.gender === '' ? '' : `, ${this.gender}`)
        if self.gender != Gender::None {
            details.push_str(&format!(", {}", self.gender.to_str()));
        }

        // JS: + (this.set.shiny ? ', shiny' : '');
        if self.set.shiny {
            details.push_str(", shiny");
        }

        details
    }
}
