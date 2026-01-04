// NOTE: This method is NOT in JavaScript - Rust-specific implementation

use crate::battle_stream::BattleStream;
use crate::battle_stream::ReplayMode;
use std::collections::VecDeque;

impl BattleStream {
    /// Create a new battle stream
    /// Note: Rust-specific constructor. Use with_options() for equivalent to TS constructor.
    pub fn new() -> Self {
        Self {
            battle: None,
            output_queue: VecDeque::new(),
            debug: false,
            no_catch: false,
            replay: ReplayMode::Off,
            keep_alive: false,
            last_request: None,
        }
    }
}
