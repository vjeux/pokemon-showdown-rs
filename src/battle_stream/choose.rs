//! BattleStream::choose - Send choice to battle
//!
//! 1:1 port of choose from battle-stream.ts

use crate::battle_stream::BattleStream;

impl BattleStream {
    /// Send a choice to the battle
    /// Equivalent to choose() in battle-stream.ts
    ///
    /// JavaScript (battle-stream.ts):
    ///   choose(choice: string) {
    ///     void this.stream.write(choice);
    ///   }
    pub fn choose(&mut self, choice: &str) {
        // JS: void this.stream.write(choice);
        self.write(choice);
    }
}
