// NOTE: This method is NOT in JavaScript - Rust-specific implementation

use crate::*;

impl Pokemon {

    /// Convert to JSON representation
    /// Equivalent to toJSON in pokemon.ts
    //
    // 	toJSON(): AnyObject {
    // 		return State.serializePokemon(this);
    // 	}
    //
    pub fn to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "name": self.name,
            "species": self.species_id.as_str(),
            "hp": self.hp,
            "maxhp": self.maxhp,
            "level": self.level,
            "status": if self.status.is_empty() { None } else { Some(self.status.as_str()) },
            "isActive": self.is_active,
            "position": self.position,
            "side": self.side_index
        })
    }
}
