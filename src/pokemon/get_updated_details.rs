use crate::*;

impl Pokemon {

    // ==========================================
    // Methods ported from pokemon.ts
    // ==========================================

    /// String representation of Pokemon
    //
    // 	toString() {
    // 		const fullname = (this.illusion) ? this.illusion.fullname : this.fullname;
    // 		return this.isActive ? this.getSlot() + fullname.slice(2) : fullname;
    // 	}
    //
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
        let mut details = self.species_id.as_str().to_string();
        if self.level != 100 {
            details.push_str(&format!(", L{}", self.level));
        }
        if self.gender != Gender::None {
            details.push_str(&format!(", {}", self.gender.to_str()));
        }
        details
    }
}
