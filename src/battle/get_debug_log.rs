use crate::*;
use crate::battle::extract_channel_messages::extract_channel_messages;

impl Battle {

    /// Get the debug log (all lines)
    /// Equivalent to battle.ts getDebugLog()
    ///
    /// JavaScript:
    /// getDebugLog() {
    ///     const channelMessages = extractChannelMessages(this.log.join('\n'), [-1]);
    ///     return channelMessages[-1].join('\n');
    /// }
    pub fn get_debug_log(&self) -> String {
        // JS: const channelMessages = extractChannelMessages(this.log.join('\n'), [-1]);
        let log_text = self.log.join("\n");
        let channel_messages = extract_channel_messages(&log_text, &[-1]);

        // JS: return channelMessages[-1].join('\n');
        if let Some(messages) = channel_messages.get(&-1) {
            messages.join("\n")
        } else {
            String::new()
        }
    }
}
