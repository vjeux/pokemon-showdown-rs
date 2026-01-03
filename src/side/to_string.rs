// Side::toString - Convert Side to string representation
//
// JS Source:
//
// 	toString() {
// 		return `${this.id}: ${this.name}`;
// 	}

use crate::*;

impl Side {
    /// Convert Side to string representation
    /// Equivalent to toString() in side.ts
    ///
    /// JavaScript (side.ts):
    ///   toString() {
    ///     return `${this.id}: ${this.name}`;
    ///   }
    pub fn to_string(&self) -> String {
        // return `${this.id}: ${this.name}`;
        format!("{}: {}", self.id.to_str(), self.name)
    }
}
