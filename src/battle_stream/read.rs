use crate::battle_stream::BattleStream;

impl BattleStream {

    /// Read output from the battle
    /// Note: Simplified version. TypeScript uses Node.js stream read() (battle-stream.ts)
    /// Returns queued messages from the output queue.
    pub fn read(&mut self) -> Option<String> {
        // First return any queued messages
        if let Some(msg) = self.output_queue.pop_front() {
            return Some(msg);
        }

        // Then check battle log
        if let Some(ref battle) = self.battle {
            let log = battle.get_log();
            if !log.is_empty() {
                return Some(log);
            }
        }

        None
    }
}
