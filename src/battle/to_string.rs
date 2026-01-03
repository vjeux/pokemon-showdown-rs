// 1:1 port of toString from battle.ts
//
// JS Source:
//
// 	toString() {
// 		return `Battle: ${this.format}`;
// 	}

use crate::*;

impl Battle {
    /// Convert battle to string representation
    /// Equivalent to battle.ts toString() (battle.ts:2006-2008)
    pub fn to_string(&self) -> String {
        // JS: return `Battle: ${this.format}`;
        format!("Battle: {}", self.format_id.to_str())
    }
}
