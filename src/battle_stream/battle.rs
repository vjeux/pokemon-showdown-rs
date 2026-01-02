// NOTE: This method is NOT in JavaScript - Rust-specific implementation

use crate::*;
use crate::battle_stream::BattleStream;

impl BattleStream {

    /// Get reference to battle
    /// Note: Rust-specific accessor. TypeScript accesses battle field directly.
    pub fn battle(&self) -> Option<&Battle> {
        self.battle.as_ref()
    }
}
