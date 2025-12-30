use crate::*;

impl Battle {

    /// Get the debug log (all lines)
    //
    // 	getDebugLog() {
    // 		const channelMessages = extractChannelMessages(this.log.join('\n'), [-1]);
    // 		return channelMessages[-1].join('\n');
    // 	}
    //
    pub fn get_debug_log(&self) -> String {
        self.log.join("\n")
    }
}
