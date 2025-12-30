use crate::*;

impl Battle {

    /// Show a hint to the player
    /// Equivalent to battle.ts hint(hint, once?, side?)
    ///
    //
    // 	hint(hint: string, once?: boolean, side?: Side) {
    // 		if (this.hints.has(side ? `${side.id}|${hint}` : hint)) return;
    //
    // 		if (side) {
    // 			this.addSplit(side.id, ['-hint', hint]);
    // 		} else {
    // 			this.add('-hint', hint);
    // 		}
    //
    // 		if (once) this.hints.add(side ? `${side.id}|${hint}` : hint);
    // 	}
    //
    pub fn hint(&mut self, hint_text: &str, once: bool, side_id: Option<SideID>) {
        // JS: if (this.hints.has(side ? `${side.id}|${hint}` : hint)) return;
        let hint_key = if let Some(sid) = side_id {
            format!("{}|{}", sid.to_str(), hint_text)
        } else {
            hint_text.to_string()
        };

        if self.hints.contains(&hint_key) {
            return;
        }

        // JS: if (side) { this.addSplit(side.id, ['-hint', hint]); } else { this.add('-hint', hint); }
        if let Some(sid) = side_id {
            self.add_split(sid.to_str(), &["-hint", hint_text], None);
        } else {
            self.add("-hint", &[Arg::Str(hint_text)]);
        }

        // JS: if (once) this.hints.add(side ? `${side.id}|${hint}` : hint);
        if once {
            self.hints.insert(hint_key);
        }
    }
}
