// NOTE: This method is NOT in JavaScript - Rust-specific implementation

use crate::*;

impl Battle {

    /// Convert battle to JSON value
    /// Equivalent to battle.ts toJSON()
    ///
    //
    // 	toJSON(): AnyObject {
    // 		return State.serializeBattle(this);
    // 	}
    //
    pub fn to_json(&self) -> serde_json::Value {
        // Delegate to state::serialize_battle just like JavaScript
        crate::state::serialize_battle(self)
    }
}
