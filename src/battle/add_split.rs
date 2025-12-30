use crate::*;

impl Battle {

    // =========================================================================
    // REMAINING METHODS (ported from battle.ts for complete 1:1 port)
    // =========================================================================

    /// Add split message for different players
    /// Equivalent to battle.ts addSplit()
    ///
    //
    // 	addSplit(side: SideID, secret: Part[], shared?: Part[]) {
    // 		this.log.push(`|split|${side}`);
    // 		this.add(...secret);
    // 		if (shared) {
    // 			this.add(...shared);
    // 		} else {
    // 			this.log.push('');
    // 		}
    // 	}
    //
    pub fn add_split(&mut self, side_id: &str, secret: &[&str], shared: Option<&[&str]>) {
        // JS: this.log.push(`|split|${side}`);
        self.log.push(format!("|split|{}", side_id));

        // JS: this.add(...secret);
        if !secret.is_empty() {
            let entry = format!("|{}", secret.join("|"));
            self.log.push(entry);
        }

        // JS: if (shared) { this.add(...shared); } else { this.log.push(''); }
        if let Some(shared_parts) = shared {
            if !shared_parts.is_empty() {
                let entry = format!("|{}", shared_parts.join("|"));
                self.log.push(entry);
            }
        } else {
            self.log.push(String::new());
        }
    }
}
