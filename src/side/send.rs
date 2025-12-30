use crate::side::*;
use crate::*;

impl Side {

    /// Send message to client
    /// Equivalent to side.ts send()
    //
    // 	send(...parts: (string | number | Function | AnyObject)[]) {
    // 		const sideUpdate = '|' + parts.map(part => {
    // 			if (typeof part !== 'function') return part;
    // 			return part(this);
    // 		}).join('|');
    // 		this.battle.send('sideupdate', `${this.id}\n${sideUpdate}`);
    // 	}
    //
    pub fn send(&self, _message: &str) {
        // In the full implementation, this would send to client
    }
}
