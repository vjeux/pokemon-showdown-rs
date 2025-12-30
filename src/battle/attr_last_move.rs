use crate::*;

impl Battle {

    /// Attribute damage to last move
    /// Equivalent to battle.ts attrLastMove()
    //
    // 	attrLastMove(...args: (string | number | Function | AnyObject)[]) {
    // 		if (this.lastMoveLine < 0) return;
    // 		if (this.log[this.lastMoveLine].startsWith('|-anim|')) {
    // 			if (args.includes('[still]')) {
    // 				this.log.splice(this.lastMoveLine, 1);
    // 				this.lastMoveLine = -1;
    // 				return;
    // 			}
    // 		} else if (args.includes('[still]')) {
    // 			// If no animation plays, the target should never be known
    // 			const parts = this.log[this.lastMoveLine].split('|');
    // 			parts[4] = '';
    // 			this.log[this.lastMoveLine] = parts.join('|');
    // 		}
    // 		// eslint-disable-next-line @typescript-eslint/no-base-to-string
    // 		this.log[this.lastMoveLine] += `|${args.join('|')}`;
    // 	}
    //
    pub fn attr_last_move(&mut self, args: &[&str]) {
        // JS: if (this.lastMoveLine < 0) return;
        if self.last_move_line < 0 {
            return;
        }

        let line_idx = self.last_move_line as usize;
        if line_idx >= self.log.len() {
            return;
        }

        // Check if it's an animation line
        if self.log[line_idx].starts_with("|-anim|") {
            // JS: if (args.includes('[still]'))
            if args.contains(&"[still]") {
                // Remove the animation line
                self.log.remove(line_idx);
                self.last_move_line = -1;
                return;
            }
        } else if args.contains(&"[still]") {
            // If no animation plays, the target should never be known
            let parts: Vec<&str> = self.log[line_idx].split('|').collect();
            let mut new_parts = parts.clone();
            if new_parts.len() > 4 {
                new_parts[4] = "";
            }
            self.log[line_idx] = new_parts.join("|");
        }

        // Append attributes to the log line
        let attrs = args.join("|");
        self.log[line_idx] = format!("{}|{}", self.log[line_idx], attrs);
    }
}
