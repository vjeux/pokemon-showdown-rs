// 1:1 port of toJSON from battle.ts
//
// JS Source:
//
// 	toJSON(): AnyObject {
// 		return State.serializeBattle(this);
// 	}

use crate::*;

impl Battle {
    /// Convert battle to JSON
    /// Equivalent to battle.ts toJSON() (battle.ts:2002-2004)
    pub fn to_json(&self) -> serde_json::Value {
        // JS: return State.serializeBattle(this);
        crate::state::serialize_battle(self)
    }
}
