//! BattleStream::write_end - Signal end of input
//!
//! 1:1 port of writeEnd from battle-stream.ts

use crate::battle_stream::BattleStream;

impl BattleStream {
    /// Signal end of input stream
    /// Equivalent to writeEnd() in battle-stream.ts
    ///
    /// JavaScript (battle-stream.ts):
    ///   writeEnd() {
    ///     return stream.writeEnd();
    ///   }
    pub fn write_end(&mut self) {
        // Mark battle as ended if it exists
        if let Some(battle) = self.battle.as_mut() {
            battle.ended = true;
        }
    }
}
