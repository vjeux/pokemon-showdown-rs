use crate::*;

impl Battle {

    /// Add a log entry
    /// Equivalent to Battle.add() in battle.ts (called throughout for protocol logging)
    //
    // 	add(...args: (ProtocolArg | ProtocolArgs)[]) {
    // 		this.log.push(`|${args.join('|')}`);
    // 	}
    //
    pub fn add_log(&mut self, event_type: &str, args: &[&str]) {
        let mut entry = format!("|{}", event_type);
        for arg in args {
            entry.push('|');
            entry.push_str(arg);
        }
        self.log.push(entry);
    }
}
