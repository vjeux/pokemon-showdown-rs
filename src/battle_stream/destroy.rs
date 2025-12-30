use crate::*;
use crate::battle_stream::BattleStream;

impl BattleStream {

    /// Destroy the stream
    /// Equivalent to _destroy() in battle-stream.ts
    pub fn destroy(&mut self) {
        self.battle = None;
        self.output_queue.clear();
    }
}
