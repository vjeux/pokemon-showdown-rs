use crate::*;

impl Battle {

    /// Add a move log entry and track the last move line
    //
    // 	addMove(...args: (string | number | Function | AnyObject)[]) {
    // 		this.lastMoveLine = this.log.length;
    // 		// eslint-disable-next-line @typescript-eslint/no-base-to-string
    // 		this.log.push(`|${args.join('|')}`);
    // 	}
    //
    pub fn add_move(&mut self, parts: &[&str]) {
        self.last_move_line = self.log.len() as i32;
        let entry = format!("|{}", parts.join("|"));
        self.log.push(entry);
    }
}
