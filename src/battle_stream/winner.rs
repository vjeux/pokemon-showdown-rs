use crate::battle_stream::BattleStream;

impl BattleStream {

    /// Get winner if battle ended
    /// Note: Rust-specific helper. TypeScript accesses battle.winner directly.
    pub fn winner(&self) -> Option<String> {
        self.battle.as_ref().and_then(|b| b.winner.clone())
    }
}
