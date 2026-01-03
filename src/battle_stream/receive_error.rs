//! BattleStream::receive_error - Handle error from battle
//!
//! 1:1 port of receiveError from battle-stream.ts

use crate::battle_stream::BattleStream;

impl BattleStream {
    /// Handle error received from battle
    /// Equivalent to receiveError() in battle-stream.ts
    ///
    /// JavaScript (battle-stream.ts):
    ///   receiveError(error: Error) {
    ///     throw error;
    ///   }
    pub fn receive_error(&self, error: String) -> Result<(), String> {
        // JS: throw error;
        Err(error)
    }
}
