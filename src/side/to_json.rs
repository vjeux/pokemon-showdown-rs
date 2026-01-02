// NOTE: This method is NOT in JavaScript - Rust-specific implementation

use crate::side::*;

impl Side {

    /// Convert to JSON
    /// Equivalent to side.ts toJSON()
    //
    // 	toJSON(): AnyObject {
    // 		return State.serializeSide(this);
    // 	}
    //
    pub fn to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "name": self.name,
            "id": self.id_str(),
            "n": self.n,
            "pokemonLeft": self.pokemon_left,
            "active": self.active,
            "sideConditions": self.side_conditions.keys().map(|k| k.as_str().to_string()).collect::<Vec<_>>()
        })
    }
}
