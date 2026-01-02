// NOTE: This method is NOT in JavaScript - Rust-specific implementation

use crate::*;

impl Battle {

    /// Restart the battle (for testing)
    /// Restart a deserialized battle
    /// Equivalent to battle.ts restart() (battle.ts:1925-1929)
    ///
    /// JS: restart(send?: (type: string, data: string | string[]) => void) {
    ///   if (!this.deserialized) throw new Error('Attempt to restart a battle which has not been deserialized');
    ///   (this as any).send = send;
    /// }
    //
    // 	restart(send?: (type: string, data: string | string[]) => void) {
    // 		if (!this.deserialized) throw new Error('Attempt to restart a battle which has not been deserialized');
    //
    // 		(this as any).send = send;
    // 	}
    //
    pub fn restart(&mut self) {
        // JS: if (!this.deserialized) throw new Error(...)
        if !self.deserialized {
            panic!("Attempt to restart a battle which has not been deserialized");
        }

        // JS: (this as any).send = send;
        // Note: Rust doesn't have a send function parameter like JS, so this is a no-op
        // The send function in JS is used for sending updates to clients
    }
}
