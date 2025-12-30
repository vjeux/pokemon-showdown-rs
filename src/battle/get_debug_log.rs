use crate::*;

impl Battle {

    /// Get the debug log (all lines)
    /// Equivalent to battle.ts getDebugLog()
    //
    // 	getDebugLog() {
    // 		const channelMessages = extractChannelMessages(this.log.join('\n'), [-1]);
    // 		return channelMessages[-1].join('\n');
    // 	}
    //
    // NOTE: This is a simplified version that doesn't use extractChannelMessages yet.
    // extractChannelMessages handles split messages (different content for different players).
    // For now, this just returns all log lines joined together, which works if split
    // messages aren't being used extensively.
    // TODO: Implement extractChannelMessages to properly handle split messages
    pub fn get_debug_log(&self) -> String {
        // JS: const channelMessages = extractChannelMessages(this.log.join('\n'), [-1]);
        // JS: return channelMessages[-1].join('\n');
        // Simplified: just join all lines (works if no split messages)
        self.log.join("\n")
    }
}
