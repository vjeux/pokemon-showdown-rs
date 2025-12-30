use crate::*;
use crate::battle_stream::BattleStream;

impl BattleStream {

    /// Start a new battle with options
    /// Note: Different from TS async start(). Corresponds to case 'start' in _writeLine() (battle-stream.ts:102-111)
    /// Creates a new Battle instance with the provided options.
    pub fn start(&mut self, options: BattleOptions) {
        self.battle = Some(Battle::new(options));
    }
}
