// JS Source:
//
// 	getHealth() {
// 		if (!this.hp) {
// 			return {
// 				side: this.side.id,
// 				secret: '0 fnt',
// 				shared: '0 fnt',
// 				percentage: 0,
// 			};
// 		}
// 		let secret = `${this.hp}/${this.maxhp}`;
// 		const ratio = this.hp / this.maxhp;
// 		const percentage = Math.ceil(ratio * 100);
// 		if ((this.side.battle.reportExactHP || this.side.battle.reportPercentages) && this.side.battle.gen > 1) {
// 			return {
// 				side: this.side.id,
// 				secret,
// 				shared: (this.side.battle.reportExactHP ? secret : `${percentage}/100`),
// 				percentage,
// 			};
// 		}
// 		let ratioString: string;
// 		if (percentage === 100 && ratio < 1) {
// 			if (ratio >= 0.995) {
// 				ratioString = '100y';
// 			} else {
// 				ratioString = '99y';
// 			}
// 		} else {
// 			ratioString = percentage + 'y';
// 		}
// 		if (!this.status) {
// 			secret += ' 100';
// 		} else if (this.status === 'slp') {
// 			secret += ` slp ${this.statusState.time}`;
// 		} else {
// 			secret += ` ${this.status}`;
// 		}
// 		return {
// 			side: this.side.id,
// 			secret,
// 			shared: ratioString,
// 			percentage,
// 		};
// 	}
//
// Note: In Rust, we return the "secret" field as a String for use in protocol messages.
// The full object with side/secret/shared/percentage is not needed in Rust's architecture.

use crate::*;

impl Pokemon {
    /// Get health string for protocol messages
    /// Equivalent to pokemon.ts getHealth(), returns the "secret" field
    pub fn get_health(&self) -> String {
        // JS: if (!this.hp) { return { ... secret: '0 fnt' ... }; }
        if self.hp == 0 {
            return "0 fnt".to_string();
        }

        // JS: let secret = `${this.hp}/${this.maxhp}`;
        let mut secret = format!("{}/{}", self.hp, self.maxhp);

        // JS: if (!this.status) { secret += ' 100'; }
        // JS: else if (this.status === 'slp') { secret += ` slp ${this.statusState.time}`; }
        // JS: else { secret += ` ${this.status}`; }
        if self.status.is_empty() {
            secret.push_str(" 100");
        } else if self.status.as_str() == "slp" {
            // StatusState.time tracking would go here
            // For now, just append slp without time
            secret.push_str(" slp");
        } else {
            secret.push_str(&format!(" {}", self.status.as_str()));
        }

        secret
    }
}
