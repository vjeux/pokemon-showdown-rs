use crate::side::*;
use crate::*;

impl Side {

    /// Emit request to client
    /// Equivalent to side.ts emitRequest()
    //
    // 	emitRequest(update: ChoiceRequest = this.activeRequest!, updatedRequest = false) {
    // 		if (updatedRequest) (this.activeRequest as MoveRequest | SwitchRequest).update = true;
    // 		this.battle.send('sideupdate', `${this.id}\n|request|${JSON.stringify(update)}`);
    // 		this.activeRequest = update;
    // 	}
    //
    pub fn emit_request(&self, request: &serde_json::Value) {
        // In the full implementation, this would send to client
        let _ = request;
    }
}
