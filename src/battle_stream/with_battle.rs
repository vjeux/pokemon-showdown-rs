use crate::*;
use crate::battle_stream::BattleStream;
use crate::battle_stream::ReplayMode;
use std::collections::VecDeque;

impl BattleStream {

    /// Create a battle stream with existing battle
    /// Note: Rust-specific constructor. No JavaScript equivalent.
    pub fn with_battle(battle: Battle) -> Self {
        Self {
            battle: Some(battle),
            output_queue: VecDeque::new(),
            input_buffer: String::new(),
            debug: false,
            no_catch: false,
            replay: ReplayMode::Off,
            keep_alive: false,
        }
    }
}
