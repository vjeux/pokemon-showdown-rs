use crate::*;

impl Battle {

    /// Add split message for different players
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
            let event = secret[0];
            let args: Vec<Arg> = secret[1..].iter().map(|s| (*s).into()).collect();
            self.add(event, &args);
        }

        // JS: if (shared) { this.add(...shared); } else { this.log.push(''); }
        if let Some(shared_parts) = shared {
            if !shared_parts.is_empty() {
                let event = shared_parts[0];
                let args: Vec<Arg> = shared_parts[1..].iter().map(|s| (*s).into()).collect();
                self.add(event, &args);
            }
        } else {
            self.log.push(String::new());
        }
    }
}
