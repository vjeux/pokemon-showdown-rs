// NOTE: This method is NOT in JavaScript - Rust-specific implementation

use crate::*;

impl Pokemon {

    /// Get health string for protocol messages
    // TypeScript source:
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
    pub fn get_health(&self) -> String {
        // TODO: implement the same logic as JavaScript
        if self.hp == 0 {
            return "0 fnt".to_string();
        }

        let mut secret = format!("{}/{}", self.hp, self.maxhp);

        if !self.status.is_empty() {
            if self.status.as_str() == "slp" {
                // Would need statusState.time for full implementation
                secret.push_str(" slp");
            } else {
                secret.push_str(&format!(" {}", self.status.as_str()));
            }
        }

        secret
    }
}
