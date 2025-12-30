use crate::battle_stream::BattleStream;

impl BattleStream {

    /// Check if battle has ended
    /// Note: Rust-specific helper. TypeScript accesses battle.ended directly.
    pub fn ended(&self) -> bool {
        self.battle.as_ref().is_none_or(|b| b.ended)
    }
}
