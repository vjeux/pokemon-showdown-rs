// NOTE: This method is NOT in JavaScript - Rust-specific implementation

use crate::*;
use crate::battle_stream::BattleStream;

impl BattleStream {

    /// Get mutable reference to battle
    /// Note: Rust-specific accessor. TypeScript accesses battle field directly.
    pub fn battle_mut(&mut self) -> Option<&mut Battle> {
        self.battle.as_mut()
    }
}
