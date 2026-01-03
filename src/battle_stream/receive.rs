//! BattleStream::receive - Process input chunk
//!
//! 1:1 port of receive from battle-stream.ts

use crate::battle_stream::BattleStream;

impl BattleStream {
    /// Process incoming data chunk
    /// Equivalent to receive() in battle-stream.ts
    ///
    /// JavaScript (battle-stream.ts):
    ///   receive(chunk: string) {
    ///     for (const line of chunk.split('\n')) {
    ///       this.receiveLine(line);
    ///     }
    ///   }
    pub fn receive(&mut self, chunk: &str) {
        // JS: for (const line of chunk.split('\n'))
        for line in chunk.split('\n') {
            // JS: this.receiveLine(line);
            self.receive_line(line);
        }
    }
}
