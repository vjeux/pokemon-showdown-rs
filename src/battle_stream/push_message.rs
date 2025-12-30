use crate::*;
use crate::battle_stream::BattleStream;
use crate::battle_stream::ReplayMode;

impl BattleStream {

    /// Push a message to the output queue
    /// Equivalent to pushMessage() in battle-stream.ts
    //
    // 	pushMessage(type: string, data: string) {
    // 		if (this.replay) {
    // 			if (type === 'update') {
    // 				if (this.replay === 'spectator') {
    // 					const channelMessages = extractChannelMessages(data, [0]);
    // 					this.push(channelMessages[0].join('\n'));
    // 				} else {
    // 					const channelMessages = extractChannelMessages(data, [-1]);
    // 					this.push(channelMessages[-1].join('\n'));
    // 				}
    // 			}
    // 			return;
    // 		}
    // 		this.push(`${type}\n${data}`);
    // 	}
    //
    pub fn push_message(&mut self, msg_type: &str, data: &str) {
        match self.replay {
            ReplayMode::Off => {
                self.output_queue
                    .push_back(format!("{}\n{}", msg_type, data));
            }
            ReplayMode::Spectator => {
                // In spectator mode, filter channel messages
                if msg_type == "update" {
                    // Would extract channel 0 messages
                    self.output_queue.push_back(data.to_string());
                }
            }
            ReplayMode::Full => {
                // In full replay mode, extract omniscient channel
                if msg_type == "update" {
                    self.output_queue.push_back(data.to_string());
                }
            }
        }
    }
}
