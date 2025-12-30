use crate::*;
use crate::battle_stream::BattleStream;
use crate::battle_stream::BattleStreamOptions;
use std::collections::VecDeque;

impl BattleStream {

    /// Create a new battle stream with options
    /// Equivalent to BattleStream constructor in battle-stream.ts
    pub fn with_options(options: BattleStreamOptions) -> Self {
        Self {
            battle: None,
            output_queue: VecDeque::new(),
            input_buffer: String::new(),
            debug: options.debug,
            no_catch: options.no_catch,
            replay: options.replay,
            keep_alive: options.keep_alive,
        }
    }
}
